import { defineStore } from 'pinia';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { ElMessage } from 'element-plus';

export interface CommitRecord {
  hash: string;
  shortHash: string;
  authorName: string;
  authorEmail: string;
  timestamp: number;
  datetime: string;
  message: string;
  repoName: string;
  isMerge: boolean;
  diffStat?: {
    filesChanged: string[];
    insertions: number;
    deletions: number;
  };
}

export const useGenerateStore = defineStore('generate', {
  state: () => ({
    commits: [] as CommitRecord[],
    dateRangeValue: [new Date(), new Date()] as [Date, Date],
    selectedRepoIds: [] as number[],
    selectedPromptId: null as number | null,
    generatedContent: '',
    isGenerating: false,
    hasInitializedListeners: false,
  }),
  actions: {
    async initializeListeners() {
      if (this.hasInitializedListeners) return;

      try {
        await listen<string>('report-stream-chunk', (event) => {
          this.generatedContent += event.payload;
          this.isGenerating = true;
        });

        await listen<string>('report-stream-done', async (event) => {
          this.generatedContent = event.payload;
          this.isGenerating = false;

          // 自动保存至 SQLite 数据库
          try {
            const formatLocalDate = (d: Date) => {
              const year = d.getFullYear();
              const month = String(d.getMonth() + 1).padStart(2, '0');
              const day = String(d.getDate()).padStart(2, '0');
              return `${year}-${month}-${day}`;
            };

            const reportData = {
              id: 0, // 由数据库自增分配
              date: formatLocalDate(this.dateRangeValue[1]), // 使用结束日期作为日报日期
              rawCommits: JSON.stringify(this.commits),
              content: event.payload,
              repoIds: this.selectedRepoIds.join(','),
              promptId: this.selectedPromptId,
              createdAt: new Date().toISOString(),
            };

            await invoke('save_report', { report: reportData });
            ElMessage.success('日报已自动生成并保存！');
          } catch (err) {
            console.error('Failed to save generated report:', err);
            ElMessage.error('自动保存历史记录失败: ' + err);
          }
        });

        this.hasInitializedListeners = true;
        console.log('Global Tauri stream listeners initialized successfully.');
      } catch (err) {
        console.error('Failed to initialize global stream listeners:', err);
      }
    },
    async startGeneration(promptContent: string) {
      this.generatedContent = '';
      this.isGenerating = true;

      // 确保全局监听器已经初始化
      await this.initializeListeners();

      try {
        await invoke('generate_report_stream', {
          promptContent,
          commits: this.commits,
        });
      } catch (err) {
        this.isGenerating = false;
        console.error('Failed to start stream generation:', err);
        ElMessage.error('开始生成失败: ' + err);
      }
    }
  }
});
