<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { 
  Edit, 
  Files, 
  FolderOpened, 
  TrendCharts,
  ArrowRight,
  Monitor
} from '@element-plus/icons-vue'
import { useRepoStore } from '../stores/repoStore'
import ReportHeatmap from '../components/ReportHeatmap.vue'

const router = useRouter()
const repoStore = useRepoStore()
const recentReportsCount = ref(0)
const loading = ref(true)

const loadStats = async () => {
  try {
    const [repos, reports] = await Promise.all([
      invoke<any[]>('list_repositories'),
      invoke<any[]>('list_reports', { dateFrom: null, dateTo: null })
    ])
    repoStore.setRepositories(repos)
    recentReportsCount.value = reports.length
  } catch (err) {
    console.error(err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadStats()
})

const getGreeting = () => {
  const hour = new Date().getHours()
  if (hour < 6) return '凌晨好'
  if (hour < 12) return '上午好'
  if (hour < 14) return '中午好'
  if (hour < 18) return '下午好'
  return '晚上好'
}

const todayFormatted = new Intl.DateTimeFormat('zh-CN', { 
  year: 'numeric', 
  month: 'long', 
  day: 'numeric',
  weekday: 'long' 
}).format(new Date())

const quickCards = [
  {
    title: '生成见解',
    desc: '让 AI 深度解析您的提交，生成极具洞察力的工作日报。',
    icon: Edit,
    path: '/generate'
  },
  {
    title: '回顾历史',
    desc: '沉淀每一天的成长，支持多日期筛选与一键 MD 导出。',
    icon: Files,
    path: '/history'
  },
  {
    title: '管理资产',
    desc: '高效追踪本地仓库，实时感知多项目的开发活跃度。',
    icon: FolderOpened,
    path: '/repos'
  }
]
</script>

<template>
  <div class="p-8 max-w-6xl mx-auto flex flex-col">
    <!-- Header Section -->
    <header class="mb-10">
      <div class="flex items-center gap-2 text-primary font-medium mb-4">
        <el-icon :size="18"><TrendCharts /></el-icon>
        <span class="text-xs uppercase tracking-wider">效率看板 / System Dashboard</span>
      </div>
      <h1 class="text-3xl font-bold text-text-title mb-2">
        {{ getGreeting() }}，开发者
      </h1>
      <p class="text-[14px] text-text-secondary">
        {{ todayFormatted }} · 开始记录今日的卓越进度
      </p>
    </header>

    <!-- Quick Actions Grid -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
      <div 
        v-for="card in quickCards" 
        :key="card.title"
        class="lark-card lark-card-hover p-6 group flex flex-col h-full bg-bg-container"
        @click="router.push(card.path)"
      >
        <div class="w-10 h-10 rounded-lg bg-primary/5 flex items-center justify-center mb-4 group-hover:bg-primary/10 transition-colors">
          <el-icon class="text-primary" :size="20"><component :is="card.icon" /></el-icon>
        </div>
        <h3 class="text-lg font-bold text-text-title mb-2">{{ card.title }}</h3>
        <p class="text-[13px] text-text-secondary leading-normal mb-6 flex-1">{{ card.desc }}</p>
        
        <div class="flex items-center text-primary text-[13px] font-semibold">
          <span>进入模块</span>
          <el-icon class="ml-1 transition-transform group-hover:translate-x-1"><ArrowRight /></el-icon>
        </div>
      </div>
    </div>
    
    <!-- Heatmap Section -->
    <div class="mb-10">
      <div class="lark-card p-6 bg-bg-container">
        <ReportHeatmap />
      </div>
    </div>

    <!-- Status & Footer -->
    <div class="mt-auto grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="lark-card p-5 flex items-center gap-4 border-l-4 border-l-emerald-500">
        <div class="w-10 h-10 rounded-full bg-emerald-500/10 flex items-center justify-center">
           <el-icon class="text-emerald-500" :size="20"><Monitor /></el-icon>
        </div>
        <div class="flex-1">
          <span class="text-[11px] text-text-placeholder font-bold uppercase tracking-wide block">环境状态</span>
          <p class="text-text-title font-semibold text-[14px]">Git 环境已就绪</p>
        </div>
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></div>
          <span class="text-[12px] text-emerald-500 font-medium">Running</span>
        </div>
      </div>
      
      <div 
        v-if="repoStore.repositories.length === 0"
        class="lark-card p-5 flex items-center gap-4 cursor-pointer hover:bg-bg-base transition-colors border-l-4 border-l-amber-500"
        @click="router.push('/repos')"
      >
        <div class="w-10 h-10 rounded-full bg-amber-500/10 flex items-center justify-center text-amber-500">
           <el-icon :size="20"><FolderOpened /></el-icon>
        </div>
        <div class="flex-1">
          <span class="text-[11px] text-amber-500 font-bold uppercase tracking-wide block">待配置</span>
          <p class="text-text-title font-semibold text-[14px]">尚未添加代码仓库</p>
        </div>
        <el-icon class="text-amber-500"><ArrowRight /></el-icon>
      </div>
    </div>
  </div>
</template>

<style scoped>

.custom-scrollbar::-webkit-scrollbar {
  display: none;
}

@keyframes bounce-x {
  0%, 100% { transform: translateX(0); }
  50% { transform: translateX(5px); }
}

.animate-bounce-x {
  animation: bounce-x 1s infinite;
}
</style>
