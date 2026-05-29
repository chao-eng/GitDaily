<script setup lang="ts">
import { watchEffect, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import MainLayout from "./layouts/MainLayout.vue"
import { useSettingsStore } from "./stores/settingsStore"
import { useGenerateStore } from "./stores/generateStore"

const settingsStore = useSettingsStore()
const generateStore = useGenerateStore()

onMounted(() => {
  settingsStore.loadSettings()
  generateStore.initializeListeners()
})

watchEffect(async () => {
  const theme = settingsStore.theme
  const isDark = theme === 'dark' || (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
  
  if (isDark) {
    document.documentElement.classList.add('dark')
    await getCurrentWindow().setTheme('dark')
  } else {
    document.documentElement.classList.remove('dark')
    await getCurrentWindow().setTheme('light')
  }
})
</script>

<template>
  <el-config-provider>
    <MainLayout />
    <WelcomeWizard />
  </el-config-provider>
</template>

<style>
/* Global styles in main.css */
</style>