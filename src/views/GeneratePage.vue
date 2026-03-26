<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { ElMessage } from "element-plus";
import { useRepoStore } from "../stores/repoStore";
import { useGenerateStore, CommitRecord } from "../stores/generateStore";
import { useSettingsStore } from "../stores/settingsStore";
import { marked } from "marked";
import "github-markdown-css/github-markdown-dark.css";

const repoStore = useRepoStore();
const generateStore = useGenerateStore();
const settingsStore = useSettingsStore();

const renderedMarkdown = ref("");

const updatePreview = async () => {
  renderedMarkdown.value = await marked.parse(generateStore.generatedContent);
};

// Watch generatedContent to update preview
watch(() => generateStore.generatedContent, updatePreview);

const prompts = ref<any[]>([]);
const selectedRepoIds = ref<number[]>([]);
const dateRange = ref<[Date, Date]>([new Date(), new Date()]);
const generating = ref(false);
const commitsLoading = ref(false);

const loadInitialData = async () => {
  try {
    const [repos, promptList] = await Promise.all([
      invoke<any[]>("list_repositories"),
      invoke<any[]>("list_prompts"),
    ]);
    repoStore.setRepositories(repos);
    prompts.value = promptList;

    // Select active repos by default
    selectedRepoIds.value = repos.filter((r) => r.isActive).map((r) => r.id);

    // Select default prompt
    const defaultPrompt = promptList.find((p) => p.is_default);
    if (defaultPrompt) generateStore.selectedPromptId = defaultPrompt.id;
  } catch (err) {
    console.error(err);
  }
};

const fetchCommits = async () => {
  if (selectedRepoIds.value.length === 0) {
    ElMessage.warning("请至少选择一个仓库");
    return;
  }

  commitsLoading.value = true;
  try {
    const formatLocalDate = (d: Date) => {
      const year = d.getFullYear();
      const month = String(d.getMonth() + 1).padStart(2, "0");
      const day = String(d.getDate()).padStart(2, "0");
      return `${year}-${month}-${day}`;
    };

    const query = {
      repoIds: selectedRepoIds.value,
      dateFrom: formatLocalDate(dateRange.value[0]),
      dateTo: formatLocalDate(dateRange.value[1]),
      author: settingsStore.gitUserName,
    };
    const result = await invoke<CommitRecord[]>("fetch_commits", { query });
    generateStore.commits = result;
    if (result.length === 0) {
      ElMessage.info("该时间范围内没有发现提交记录");
    }
  } catch (err) {
    ElMessage.error("获取提交失败: " + err);
  } finally {
    commitsLoading.value = false;
  }
};

const unlistens = ref<UnlistenFn[]>([]);

const generateReport = async () => {
  if (generateStore.commits.length === 0) {
    ElMessage.warning("请先获取并选择提交记录");
    return;
  }
  if (!generateStore.selectedPromptId) {
    ElMessage.warning("请选择一个生成模板");
    return;
  }

  generating.value = true;
  generateStore.generatedContent = "";

  try {
    // 1. Set up listeners
    const unlistenChunk = await listen<string>(
      "report-stream-chunk",
      (event) => {
        generateStore.generatedContent += event.payload;
      }
    );
    const unlistenDone = await listen<string>(
      "report-stream-done",
      async (event) => {
        generateStore.generatedContent = event.payload;
        generating.value = false;

        // Save to History
        try {
          const formatLocalDate = (d: Date) => {
            const year = d.getFullYear();
            const month = String(d.getMonth() + 1).padStart(2, "0");
            const day = String(d.getDate()).padStart(2, "0");
            return `${year}-${month}-${day}`;
          };

          const reportData = {
            id: 0, // Assigned by DB
            date: formatLocalDate(dateRange.value[1]), // Use end date as report date
            rawCommits: JSON.stringify(generateStore.commits),
            content: event.payload,
            repoIds: selectedRepoIds.value.join(","),
            promptId: generateStore.selectedPromptId,
            createdAt: new Date().toISOString(),
          };
          await invoke("save_report", { report: reportData });
          ElMessage.success("日报已生成并保存");
        } catch (err) {
          ElMessage.error("保存历史记录失败: " + err);
        }
      }
    );
    unlistens.value.push(unlistenChunk, unlistenDone);

    // 2. Start generation
    const prompt = prompts.value.find(
      (p) => p.id === generateStore.selectedPromptId
    );
    await invoke("generate_report_stream", {
      promptContent: prompt.content,
      commits: generateStore.commits,
    });
  } catch (err) {
    ElMessage.error("生成失败: " + err);
    generating.value = false;
  }
};

onUnmounted(() => {
  unlistens.value.forEach((u) => u());
});

const copyToClipboard = () => {
  navigator.clipboard.writeText(generateStore.generatedContent);
  ElMessage.success("已复制到剪贴板");
};

onMounted(() => {
  loadInitialData();
});
</script>

<template>
  <div class="h-full flex overflow-hidden bg-bg-base">
    <!-- Left Panel: Config -->
    <div
      class="w-80 flex-shrink-0 border-r border-border-base bg-bg-container flex flex-col shadow-sm"
    >
      <div class="p-5 border-b border-border-base">
        <h2 class="text-lg font-bold text-text-title mb-5">生成配置</h2>

        <div class="space-y-6">
          <!-- Repos -->
          <div>
            <label
              class="block text-[11px] font-bold text-text-placeholder uppercase tracking-wider mb-2.5"
              >选择仓库</label
            >
            <div
              class="space-y-1 max-h-48 overflow-y-auto pr-1 custom-scrollbar"
            >
              <div
                v-for="repo in repoStore.repositories"
                :key="repo.id"
                @click="
                  selectedRepoIds.includes(repo.id)
                    ? (selectedRepoIds = selectedRepoIds.filter(
                        (id) => id !== repo.id
                      ))
                    : selectedRepoIds.push(repo.id)
                "
                class="flex items-center gap-2.5 px-2 py-1.5 rounded-md cursor-pointer transition-colors"
                :class="[
                  selectedRepoIds.includes(repo.id)
                    ? 'bg-primary/10 text-primary'
                    : 'hover:bg-black/5 dark:hover:bg-white/5 text-text-secondary',
                ]"
              >
                <div
                  class="w-3.5 h-3.5 rounded border border-border-base flex items-center justify-center transition-colors"
                  :class="{
                    'bg-primary border-primary': selectedRepoIds.includes(
                      repo.id
                    ),
                  }"
                >
                  <el-icon
                    v-if="selectedRepoIds.includes(repo.id)"
                    :size="10"
                    color="white"
                    ><Select
                  /></el-icon>
                </div>
                <span class="text-[13px] truncate font-medium">{{
                  repo.name
                }}</span>
              </div>
            </div>
          </div>

          <!-- Date Range -->
          <div>
            <label
              class="block text-[11px] font-bold text-text-placeholder uppercase tracking-wider mb-2"
              >日期范围</label
            >
            <el-date-picker
              v-model="dateRange"
              type="daterange"
              range-separator="-"
              start-placeholder="开始"
              end-placeholder="结束"
              size="small"
              class="!w-full l-picker"
              :clearable="false"
            />
          </div>

          <!-- Prompt Template -->
          <div>
            <label
              class="block text-[11px] font-bold text-text-placeholder uppercase tracking-wider mb-2"
              >生成模板</label
            >
            <el-select
              v-model="generateStore.selectedPromptId"
              placeholder="选择模板"
              size="small"
              class="!w-full"
            >
              <el-option
                v-for="item in prompts"
                :key="item.id"
                :label="item.title"
                :value="item.id"
              />
            </el-select>
          </div>

          <button
            class="industrial-btn-primary w-full text-[13px] py-1.5 mt-2"
            @click="fetchCommits"
            :disabled="commitsLoading"
          >
            <el-icon v-if="commitsLoading" class="animate-spin mr-1"
              ><Loading
            /></el-icon>
            <el-icon v-else class="mr-1"><RefreshRight /></el-icon>
            获取提交记录
          </button>
        </div>
      </div>

      <!-- Commits Preview List -->
      <div class="flex-1 overflow-hidden flex flex-col">
        <div
          class="px-5 py-3 flex justify-between items-center bg-bg-base/50 border-b border-border-base"
        >
          <span
            class="text-[11px] font-bold text-text-placeholder uppercase tracking-wider"
            >提交预览 ({{ generateStore.commits.length }})</span
          >
        </div>
        <div class="flex-1 overflow-y-auto p-3 space-y-2 custom-scrollbar">
          <div
            v-if="generateStore.commits.length === 0"
            class="text-center py-12 opacity-20"
          >
            <el-icon :size="32"><Collection /></el-icon>
            <p class="mt-2 text-[12px] font-medium">暂无记录</p>
          </div>
          <div
            v-for="commit in generateStore.commits"
            :key="commit.hash"
            class="p-2.5 border-b border-border-base/50 hover:bg-black/5 dark:hover:bg-white/5 transition-colors"
          >
            <div class="flex justify-between items-center mb-1">
              <span class="text-[10px] font-mono text-primary font-bold">{{
                commit.shortHash
              }}</span>
              <span class="text-[10px] text-text-placeholder">{{
                commit.repoName
              }}</span>
            </div>
            <p
              class="text-[12px] text-text-secondary line-clamp-2 leading-snug"
            >
              {{ commit.message }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Right Panel: Result Area -->
    <div
      class="flex-1 flex flex-col bg-bg-base border-l border-border-base relative"
    >
      <!-- Toolbar -->
      <header
        class="min-h-[56px] flex-shrink-0 flex items-center justify-between px-8 border-b border-border-base bg-bg-container relative z-10 flex-nowrap"
      >
        <div class="flex items-center gap-3 shrink-0">
          <div
            class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center shrink-0"
          >
            <el-icon class="text-primary" :size="18"><MagicStick /></el-icon>
          </div>
          <h2 class="text-lg font-bold text-text-title whitespace-nowrap">
            生成结果
          </h2>
        </div>
        <div class="flex gap-2 shrink-0">
          <button
            v-if="generateStore.generatedContent"
            class="industrial-btn-secondary text-[13px] py-1.5 whitespace-nowrap"
            @click="copyToClipboard"
          >
            <el-icon class="mr-1"><CopyDocument /></el-icon>
            复制全文
          </button>
          <button
            class="industrial-btn-primary text-[13px] py-1.5 !px-5 whitespace-nowrap"
            @click="generateReport"
            :disabled="generating || generateStore.commits.length === 0"
          >
            <el-icon v-if="generating" class="animate-spin mr-1"
              ><Loading
            /></el-icon>
            <el-icon v-else class="mr-1"><MagicStick /></el-icon>
            {{ generateStore.generatedContent ? "重新生成" : "开始生成" }}
          </button>
        </div>
      </header>

      <!-- Content Area -->
      <div
        class="flex-1 overflow-y-auto p-12 relative z-10 custom-scrollbar flex justify-center"
      >
        <transition name="fade-up" mode="out-in">
          <div
            v-if="generating"
            class="h-full flex flex-col items-center justify-center text-center"
          >
            <el-icon class="animate-spin text-primary mb-4" :size="48"
              ><Loading
            /></el-icon>
            <h3 class="text-xl font-bold text-text-title mb-1.5">
              正在分析数据...
            </h3>
            <p class="text-[13px] text-text-secondary">
              AI 正在根据您的提交记录编撰日报
            </p>
          </div>

          <div
            v-else-if="generateStore.generatedContent"
            class="w-full max-w-4xl"
          >
            <div class="lark-card p-12 bg-bg-container shadow-lark">
              <div v-html="renderedMarkdown" class="markdown-body"></div>
            </div>
          </div>

          <div
            v-else
            class="h-full flex flex-col items-center justify-center text-center opacity-40 select-none"
          >
            <div
              class="w-24 h-24 border border-dashed border-border-base rounded-2xl flex items-center justify-center mb-5"
            >
              <el-icon :size="32" class="text-text-placeholder"
                ><MagicStick
              /></el-icon>
            </div>
            <h3 class="text-xl font-bold text-text-title">待生成</h3>
            <p
              class="text-[13px] mt-2 max-w-xs leading-relaxed text-text-secondary"
            >
              在左侧挑选好项目和时间范围，<br />让 AI 帮您完成枯燥的记录工作。
            </p>
          </div>
        </transition>
      </div>
    </div>
  </div>
</template>

<style scoped>

.custom-date-picker :deep(.el-input__wrapper) {
  color: var(--text-secondary);
}
.custom-date-picker :deep(.el-input__wrapper) {
  background-color: var(--bg-base);
  border: 1px solid var(--border-base);
  box-shadow: none !important;
}

.custom-select :deep(.el-input__wrapper) {
  background-color: var(--bg-base);
  border: 1px solid var(--border-base);
  box-shadow: none !important;
}

.fade-up-enter-active,
.fade-up-leave-active {
  transition: all 0.5s cubic-bezier(0.16, 1, 0.3, 1);
}
.fade-up-enter-from {
  opacity: 0;
  transform: translateY(20px);
}
.fade-up-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}
</style>
