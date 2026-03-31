<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

defineOptions({
  name: 'ReportHeatmap'
})

const activityData = ref<Record<string, number>>({})
const loading = ref(true)

const loadActivity = async () => {
  try {
    const data = await invoke<Record<string, number>>('get_activity_data')
    activityData.value = data
  } catch (err) {
    console.error('Failed to load activity data:', err)
  } finally {
    loading.value = false
  }
}

// Generate data for the last 365 days
const days = computed(() => {
  const result = []
  const today = new Date()
  for (let i = 364; i >= 0; i--) {
    const d = new Date(today)
    d.setDate(today.getDate() - i)
    const dateStr = d.toISOString().split('T')[0]
    result.push({
      date: dateStr,
      count: activityData.value[dateStr] || 0,
      dayOfWeek: d.getDay()
    })
  }
  return result
})

const getLevelClass = (count: number) => {
  if (count === 0) return 'bg-bg-base border border-border-base/30 hover:border-primary/30 hover:bg-primary/5'
  if (count < 3) return 'bg-primary/20'
  if (count < 6) return 'bg-primary/45'
  if (count < 10) return 'bg-primary/75'
  return 'bg-primary'
}

// Group by weeks for the grid
const weeks = computed(() => {
// ... same logic
  const result = []
  let currentWeek = []
  
  if (days.value.length === 0) return []

  // Padding for the first week
  const firstDay = new Date(days.value[0].date).getDay()
  for (let i = 0; i < firstDay; i++) {
    currentWeek.push(null)
  }

  for (const day of days.value) {
    currentWeek.push(day)
    if (currentWeek.length === 7) {
      result.push(currentWeek)
      currentWeek = []
    }
  }
  
  if (currentWeek.length > 0) {
    while (currentWeek.length < 7) {
      currentWeek.push(null)
    }
    result.push(currentWeek)
  }
  
  return result
})

onMounted(() => {
  loadActivity()
})
</script>

<template>
  <div class="lark-card p-8 shadow-lark bg-bg-container">
    <div class="flex items-center justify-between mb-8">
      <div>
        <h3 class="text-lg font-bold text-text-title flex items-center gap-2">
          <div class="w-1.5 h-1.5 rounded-full bg-primary animate-pulse"></div>
          活跃度图谱
        </h3>
        <p class="text-[12px] text-text-secondary mt-1 font-medium">展示您在过去一年中生成日报的频率与分布。</p>
      </div>
      <div class="flex items-center gap-2 text-[10px] text-text-placeholder uppercase font-bold tracking-widest">
        <span>Less</span>
        <div class="flex gap-1">
          <div class="w-3 h-3 rounded-sm bg-bg-base border border-border-base/50"></div>
          <div class="w-3 h-3 rounded-sm bg-primary/10"></div>
          <div class="w-3 h-3 rounded-sm bg-primary/30"></div>
          <div class="w-3 h-3 rounded-sm bg-primary/60"></div>
          <div class="w-3 h-3 rounded-sm bg-primary"></div>
        </div>
        <span>More</span>
      </div>
    </div>

    <div class="overflow-x-auto custom-scrollbar pb-4">
      <div class="flex gap-1.5 min-w-max">
        <div v-for="(week, wIdx) in weeks" :key="wIdx" class="flex flex-col gap-1.5">
          <el-tooltip
            v-for="(day, dIdx) in week"
            :key="dIdx"
            :content="day ? `${day.date}: ${day.count} 次提交` : ''"
            :disabled="!day"
            placement="top"
          >
            <div 
              class="w-3.5 h-3.5 rounded-sm transition-all duration-300"
              :class="[
                !day ? 'invisible' : getLevelClass(day.count)
              ]"
            ></div>
          </el-tooltip>
        </div>
      </div>
    </div>
    
    <div v-if="days && days.length > 0" class="mt-6 flex justify-between text-[11px] text-text-secondary font-medium tracking-wide px-1">
      <span>{{ new Date(days[0].date).toLocaleDateString('zh-CN', { month: 'short', year: 'numeric' }) }}</span>
      <span class="font-bold">今天</span>
    </div>
  </div>
</template>

<style scoped>
</style>
