<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { marked } from 'marked'
import 'github-markdown-css/github-markdown-dark.css'

interface Report {
  id: number;
  date: string;
  content: string;
  repoIds: string;
  createdAt: string;
}

const reports = ref<Report[]>([])
const loading = ref(false)
const filterDate = ref<[Date, Date] | null>(null)
const selectedReport = ref<Report | null>(null)
const drawerVisible = ref(false)
const renderedContent = ref('')

const loadReports = async () => {
  loading.value = true
  try {
    let dateFrom = null
    let dateTo = null
    if (filterDate.value) {
      dateFrom = filterDate.value[0].toISOString().split('T')[0]
      dateTo = filterDate.value[1].toISOString().split('T')[0]
    }
    const result = await invoke<Report[]>('list_reports', { dateFrom, dateTo })
    reports.value = result
  } catch (err) {
    ElMessage.error('加载历史失败: ' + err)
  } finally {
    loading.value = false
  }
}

const showDetail = async (report: Report) => {
  selectedReport.value = report
  renderedContent.value = await marked.parse(report.content)
  drawerVisible.value = true
}

const deleteReport = (id: number) => {
  ElMessageBox.confirm('确定要永久删除这份日报吗？', '提示', {
    confirmButtonText: '删除',
    cancelButtonText: '取消',
    type: 'warning',
    customClass: 'gitdaily-message-box'
  }).then(async () => {
    try {
      await invoke('delete_report', { id })
      reports.value = reports.value.filter(r => r.id !== id)
      ElMessage.success('删除成功')
      if (selectedReport.value?.id === id) drawerVisible.value = false
    } catch (err) {
      ElMessage.error('删除失败: ' + err)
    }
  })
}

const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text)
  ElMessage.success('已复制到剪贴板')
}

const exportToFile = async (report: Report) => {
  try {
    const path = await save({
      filters: [{ name: 'Markdown', extensions: ['md'] }],
      defaultPath: `GitDaily_${report.date}.md`
    })
    
    if (path) {
      await writeTextFile(path, report.content)
      ElMessage.success('文件已导出成功')
    }
  } catch (err) {
    ElMessage.error('导出失败: ' + err)
  }
}

onMounted(() => {
  loadReports()
})
</script>

<template>
  <div class="h-full flex flex-col p-8 max-w-6xl mx-auto bg-bg-base">
    <header class="flex justify-between items-end mb-10">
      <div>
        <h1 class="text-3xl font-bold text-text-title mb-2">历史记录</h1>
        <p class="text-[14px] text-text-secondary">在此查阅您过去生成的所有日报记录。</p>
      </div>
      <div class="flex items-center gap-3">
        <el-date-picker
          v-model="filterDate"
          type="daterange"
          range-separator="-"
          start-placeholder="开始"
          end-placeholder="结束"
          class="!w-64 !h-[34px]"
          @change="loadReports"
        />
        <button @click="loadReports" class="industrial-btn-secondary !h-[34px] !py-0 px-4 text-[13px] border-border-base hover:border-primary">
          <el-icon class="mr-1"><Search /></el-icon>
          筛选
        </button>
      </div>
    </header>

    <!-- Reports Grid -->
    <div v-if="reports.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 overflow-y-auto custom-scrollbar pr-2 pb-10">
      <div 
        v-for="report in reports" 
        :key="report.id"
        class="lark-card lark-card-hover p-6 cursor-pointer flex flex-col bg-bg-container shadow-lark relative border-b-2 border-b-transparent hover:border-b-primary"
        @click="showDetail(report)"
      >
        <div class="flex justify-between items-start mb-4">
          <div class="flex flex-col">
            <span class="text-[11px] font-bold text-primary uppercase tracking-wider mb-1">{{ report.date }}</span>
            <span class="text-[10px] text-text-placeholder">
              {{ report.createdAt ? new Date(report.createdAt).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) : '' }} 生成
            </span>
          </div>
          <button 
            class="w-7 h-7 rounded-md opacity-0 group-hover:opacity-100 transition-opacity hover:bg-red-50 text-text-placeholder hover:text-red-500 flex items-center justify-center"
            @click.stop="deleteReport(report.id)"
          >
            <el-icon :size="14"><Delete /></el-icon>
          </button>
        </div>
        <p class="text-[13px] text-text-secondary line-clamp-3 leading-relaxed mb-6 flex-1">
          {{ report.content.replace(/[#*|`]/g, '').replace(/\n+/g, ' ').trim().substring(0, 150) }}
        </p>
        <div class="flex items-center text-primary text-[12px] font-bold transition-all">
          <span>查看详情</span>
          <el-icon class="ml-1"><ArrowRight /></el-icon>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="flex-1 flex flex-col items-center justify-center text-center opacity-30 mt-20 select-none">
      <el-icon :size="80"><Collection /></el-icon>
      <h3 class="text-2xl font-display font-medium mt-6">空空如也</h3>
      <p class="text-sm mt-2">当您生成完第一份日报后，它会出现在这里</p>
    </div>

    <el-drawer
      v-model="drawerVisible"
      size="650px"
      :with-header="false"
      class="report-drawer"
    >
      <div v-if="selectedReport" class="h-full flex flex-col bg-bg-container text-text-title">
        <header class="p-6 border-b border-border-base flex justify-between items-center bg-bg-base/30">
          <div>
            <div class="text-[11px] font-bold text-primary uppercase tracking-wider mb-1">汇报日期：{{ selectedReport.date }}</div>
            <h2 class="text-xl font-bold text-text-title">日报记录详情</h2>
          </div>
          <div class="flex gap-2">
            <button class="w-9 h-9 rounded-lg hover:bg-black/5 dark:hover:bg-white/5 flex items-center justify-center transition-colors" @click="exportToFile(selectedReport)">
              <el-icon :size="18"><Download /></el-icon>
            </button>
            <button class="w-9 h-9 rounded-lg hover:bg-black/5 dark:hover:bg-white/5 flex items-center justify-center transition-colors" @click="copyToClipboard(selectedReport.content)">
              <el-icon :size="18"><CopyDocument /></el-icon>
            </button>
            <button class="w-9 h-9 rounded-lg hover:bg-red-50 flex items-center justify-center transition-colors text-text-placeholder hover:text-red-500" @click="deleteReport(selectedReport.id)">
              <el-icon :size="18"><Delete /></el-icon>
            </button>
          </div>
        </header>
        <div class="flex-1 p-10 overflow-y-auto custom-scrollbar flex flex-col items-center bg-bg-base border-l border-border-base">
          <div class="lark-card p-12 bg-bg-container shadow-lark w-full max-w-2xl h-fit mb-10 shrink-0">
            <div 
              v-html="renderedContent" 
              class="markdown-body"
            ></div>
          </div>
        </div>
      </div>
    </el-drawer>
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

:deep(.report-drawer) {
  background-color: var(--bg-base) !important;
  border-left: 1px solid var(--border-base);
}

:deep(.el-drawer__header) {
  background-color: var(--bg-container) !important;
  border-bottom: 1px solid var(--border-base);
  margin-bottom: 0;
  padding: 16px 24px;
}

.gitdaily-message-box {
  background-color: var(--bg-surface) !important;
  border: 1px solid var(--border-subtle);
}
</style>
