use std::process::Command;
use crate::models::{CommitRecord, GitLogQuery};
use crate::errors::{AppError, Result};
use std::sync::Mutex;
use rusqlite::Connection;

pub struct GitService;

impl GitService {
    fn create_git_command(path: Option<&str>) -> Command {
        let mut cmd = Command::new("git");
        
        // 修复 MacOS 打包后环境变量丢失的问题
        #[cfg(target_os = "macos")]
        {
            let current_path = std::env::var("PATH").unwrap_or_default();
            let home = std::env::var("HOME").unwrap_or_default();
            
            // 拼接常用的 Git 路径（Homebrew 路径等）
            let new_path = format!(
                "{}:/usr/local/bin:/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin", 
                current_path
            );
            
            cmd.env("PATH", new_path);
            if !home.is_empty() {
                cmd.env("HOME", home);
            }
        }

        if let Some(p) = path {
            cmd.current_dir(p);
        }
        cmd
    }

    pub fn get_git_user_name(repo_path: Option<&str>) -> Result<String> {
        let mut cmd = Self::create_git_command(repo_path);
        cmd.arg("config").arg("user.name");
        
        let output = cmd.output()?;
        if output.status.success() {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !name.is_empty() {
                return Ok(name);
            }
        }

        // 尝试全局配置
        let mut global_cmd = Self::create_git_command(None);
        global_cmd.args(["config", "--global", "user.name"]);
        let global_output = global_cmd.output()?;
        
        Ok(String::from_utf8_lossy(&global_output.stdout).trim().to_string())
    }

    pub fn fetch_commits(
        conn: &Mutex<Connection>,
        query: GitLogQuery,
    ) -> Result<Vec<CommitRecord>> {
        let conn = conn.lock().unwrap();
        
        // 1. Resolve repo paths
        let mut repos = Vec::new();
        for id in query.repo_ids {
            let mut stmt = conn.prepare("SELECT name, path FROM repositories WHERE id = ?1")?;
            let repo = stmt.query_row([id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;
            repos.push(repo);
        }

        let mut all_commits = Vec::new();
        // 只有当查询显式要求过滤作者时才使用设置
        let author = query.author;

        // 2. Fetch from each repo
        for (name, path) in repos {
            let repo_commits = Self::fetch_from_repo(&name, &path, &query.date_from, &query.date_to, author.as_deref())?;
            all_commits.extend(repo_commits);
        }

        // 3. Sort by timestamp descending
        all_commits.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(all_commits)
    }

    fn fetch_from_repo(
        repo_name: &str,
        path: &str,
        date_from: &str,
        date_to: &str,
        author: Option<&str>,
    ) -> Result<Vec<CommitRecord>> {
        // Use a unique separator to reliably split commits
        let separator = "==COMMIT_START==";
        let format = format!("{}%H|%h|%an|%ae|%at|%ai|%s", separator);
        
        let args = vec![
            "log".to_string(),
            format!("--format={}", format),
            "--stat".to_string(),
            format!("--after={} 00:00:00", date_from),
            format!("--before={} 23:59:59", date_to),
        ];

        let mut args_with_author = args.clone();
        let mut has_author_filter = false;
        if let Some(a) = author {
            if !a.trim().is_empty() {
                args_with_author.push(format!("--author={}", a));
                has_author_filter = true;
            }
        }

        println!("Executing git in {}: git {}", path, args_with_author.join(" "));

        let mut cmd = Self::create_git_command(Some(path));
        cmd.args(&args_with_author);
        
        let output = cmd.output()?;
        let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();

        // 诊断逻辑：如果带作者过滤没读到，且我们确实设置了过滤，尝试不带作者过滤读一次
        if stdout.trim().is_empty() && has_author_filter {
            println!("No commits found for author '{}'. Retrying without author filter...", author.unwrap());
            let mut fallback_cmd = Self::create_git_command(Some(path));
            fallback_cmd.args(&args);
            let fallback_output = fallback_cmd.output()?;
            
            if fallback_output.status.success() {
                let fallback_stdout = String::from_utf8_lossy(&fallback_output.stdout);
                if !fallback_stdout.trim().is_empty() {
                    println!("Diagnostic: Found commits WITHOUT author filter. Your configured Git username may be incorrect.");
                    // 为了定时任务能跑通，如果带作者过滤不到，我们可以选择降级到“不限作者”
                    stdout = fallback_stdout.to_string();
                }
            }
        }

        if !output.status.success() && stdout.trim().is_empty() {
            return Err(AppError::GitError(String::from_utf8_lossy(&output.stderr).to_string()));
        }

        let mut commits = Vec::new();

        // Split by our custom separator
        let commit_blocks: Vec<&str> = stdout.split(separator).filter(|s| !s.trim().is_empty()).collect();

        for block in commit_blocks {
            let mut lines = block.lines();
            let header = lines.next().unwrap_or("");
            let parts: Vec<&str> = header.split('|').collect();
            if parts.len() < 7 { continue; }

            // Rest of lines are stat info until the end of this block
            let mut files = Vec::new();
            let mut insertions = 0;
            let mut deletions = 0;

            for line in lines {
                let trimmed = line.trim();
                if trimmed.is_empty() { continue; }
                
                // Parse individual file line: " path/to/file | 5 +++--"
                if trimmed.contains('|') && (trimmed.contains('+') || trimmed.contains('-')) {
                    if let Some(file_part) = trimmed.split('|').next() {
                        files.push(file_part.trim().to_string());
                    }
                }
                
                // Parse summary line: " 2 files changed, 5 insertions(+), 3 deletions(-)"
                if trimmed.contains("files changed") || trimmed.contains("file changed") {
                    let parts: Vec<&str> = trimmed.split(',').collect();
                    for p in parts {
                        let p = p.trim();
                        if p.contains("insertion") {
                             insertions = p.split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0);
                        } else if p.contains("deletion") {
                             deletions = p.split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0);
                        }
                    }
                }
            }

            commits.push(CommitRecord {
                hash: parts[0].to_string(),
                short_hash: parts[1].to_string(),
                author_name: parts[2].to_string(),
                author_email: parts[3].to_string(),
                timestamp: parts[4].parse().unwrap_or(0),
                datetime: parts[5].to_string(),
                message: parts[6].to_string(),
                repo_name: repo_name.to_string(),
                diff_stat: Some(crate::models::DiffStat {
                    files_changed: files,
                    insertions,
                    deletions,
                }),
                is_merge: parts[6].to_lowercase().contains("merge branch"),
            });
        }

        Ok(commits)
    }
}
