<script setup lang="ts">
import { 
  HomeFilled, 
  Document, 
  Files, 
  FolderOpened, 
  Edit, 
  Setting,
  Sunny,
  Moon
} from '@element-plus/icons-vue'
import { useRoute } from 'vue-router'
import { useSettingsStore } from '../stores/settingsStore'
import { openUrl } from '@tauri-apps/plugin-opener'
import { getVersion } from '@tauri-apps/api/app'
import { ref, onMounted } from 'vue'

const route = useRoute()
const settingsStore = useSettingsStore()

const toggleTheme = () => {
  const newTheme = settingsStore.theme === 'dark' ? 'light' : 'dark'
  settingsStore.saveTheme(newTheme)
}

const openGithub = async () => {
  await openUrl('https://github.com/chao-eng/GitDaily')
}

const version = ref('...')

onMounted(async () => {
  try {
    version.value = await getVersion()
  } catch (e) {
    version.value = '1.0.0'
  }
})

const navItems = [
  { name: '仪表盘', path: '/dashboard', icon: HomeFilled },
  { name: '生成日报', path: '/generate', icon: Edit },
  { name: '历史记录', path: '/history', icon: Files },
  { name: '仓库管理', path: '/repos', icon: FolderOpened },
  { name: '提示词模板', path: '/prompts', icon: Document },
  { name: '设置', path: '/settings', icon: Setting },
]
</script>

<template>
  <div class="h-full flex flex-col bg-bg-container border-r border-border-base p-3">
    <div class="flex items-center gap-2.5 px-3 py-5 mb-6">
      <div class="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
        <el-icon :size="20" color="white"><Document /></el-icon>
      </div>
      <span class="text-lg font-display font-bold tracking-tight text-text-title">GitDaily</span>
    </div>

    <nav class="flex-1 flex flex-col gap-1">
      <router-link
        v-for="item in navItems"
        :key="item.path"
        :to="item.path"
        class="lark-sidebar-item"
        :class="[route.path === item.path ? 'active' : '']"
      >
        <el-icon :size="18">
          <component :is="item.icon" />
        </el-icon>
        <span class="text-[14px]">{{ item.name }}</span>
      </router-link>
    </nav>

    <!-- Theme Toggle & Info -->
    <div class="mt-auto border-t border-border-base pt-4 pb-2 px-1">
      <div 
        class="flex items-center gap-2.5 px-3 py-2 mb-1 rounded-lg cursor-pointer transition-colors duration-200 hover:bg-black/5 dark:hover:bg-white/5 text-text-secondary hover:text-text-title"
        @click="openGithub"
      >
        <el-icon :size="16">
          <svg viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" class="css-i6dzq1"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path></svg>
        </el-icon>
        <span class="text-[13px] font-medium">GitHub 源码</span>
      </div>
      <div 
        class="flex items-center gap-2.5 px-3 py-2 rounded-lg cursor-pointer transition-colors duration-200 hover:bg-black/5 dark:hover:bg-white/5 text-text-secondary hover:text-text-title"
        @click="toggleTheme"
      >
        <el-icon :size="16"><component :is="settingsStore.theme === 'dark' ? Moon : Sunny" /></el-icon>
        <span class="text-[13px] font-medium">{{ settingsStore.theme === 'dark' ? '深色' : '浅色' }}</span>
        <span class="ml-auto text-[11px] opacity-40 font-mono">v{{ version }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>

.router-link-active {
  color: var(--primary);
  background-color: rgba(51, 112, 255, 0.1);
}
</style>
