<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { useSettingsStore } from '../stores/settingsStore'

const settingsStore = useSettingsStore()
const loading = ref(false)
const testing = ref(false)

const loadSettings = async () => {
  loading.value = true
  try {
    await settingsStore.loadSettings()
  } catch (err) {
    console.error('Failed to load settings:', err)
  } finally {
    loading.value = false
  }
}

const saveSettings = async () => {
  loading.value = true
  try {
    const settingsObj = {
      'ai.api_base_url': settingsStore.aiConfig.apiBaseUrl,
      'ai.api_key': settingsStore.aiConfig.apiKey,
      'ai.model_name': settingsStore.aiConfig.modelName,
      'ai.temperature': settingsStore.aiConfig.temperature.toString(),
      'ai.max_tokens': settingsStore.aiConfig.maxTokens.toString(),
      'git.user_name': settingsStore.gitUserName,
      'app.theme': settingsStore.theme,
    }
    await invoke('update_settings', { settings: settingsObj })
    ElMessage.success('配置已保存')
  } catch (err) {
    ElMessage.error('保存失败: ' + err)
  } finally {
    loading.value = false
  }
}

const testConnection = async () => {
  testing.value = true
  try {
    const result = await invoke<{ success: boolean; message: string }>('test_ai_connection', {
      config: settingsStore.aiConfig
    })
    if (result.success) {
      ElMessage.success(result.message)
    } else {
      ElMessage.error(result.message)
    }
  } catch (err) {
    ElMessage.error('连接异常: ' + err)
  } finally {
    testing.value = false
  }
}

const fetchGitUser = async () => {
  try {
    const name = await invoke<string>('get_git_user_name', { repoPath: null })
    if (name) settingsStore.gitUserName = name
  } catch (err) {
     console.warn('Could not auto-fetch git user')
  }
}

onMounted(() => {
  loadSettings()
})
</script>

<template>
  <div class="p-10 max-w-4xl mx-auto flex flex-col h-full overflow-y-auto custom-scrollbar bg-bg-base">
    <header class="mb-10">
      <h1 class="text-3xl font-bold text-text-title mb-2">系统设置</h1>
      <p class="text-[14px] text-text-secondary">管理您的 AI 后端服务与全局偏好。</p>
    </header>

    <div class="space-y-8">
      <!-- AI Configuration -->
      <section class="lark-card bg-bg-container p-8 shadow-lark">
        <div class="flex items-center gap-3 mb-8">
          <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center">
            <el-icon class="text-primary" :size="20"><Connection /></el-icon>
          </div>
          <h2 class="text-lg font-bold text-text-title">AI 模型配置</h2>
        </div>

        <el-form :model="settingsStore.aiConfig" label-position="top" class="lark-form">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-10 gap-y-2">
            <el-form-item label="API 基础地址">
              <el-input 
                v-model="settingsStore.aiConfig.apiBaseUrl" 
                placeholder="https://api.openai.com/v1"
              />
            </el-form-item>
              
              <el-form-item label="模型名称">
                <el-input 
                  v-model="settingsStore.aiConfig.modelName" 
                  placeholder="gpt-4o-mini"
                />
              </el-form-item>

              <el-form-item label="API Key" class="md:col-span-2">
                <el-input 
                  v-model="settingsStore.aiConfig.apiKey" 
                  type="password" 
                  show-password
                  placeholder="sk-..."
                />
              </el-form-item>

              <el-form-item>
                <template #label>
                  <div class="flex items-center gap-1.5">
                    <span>生成温度 (Temperature)</span>
                    <el-tooltip content="控制生成内容的随机性。较低的温度（如 0.2）使结果更严谨、稳定；较高的温度（如 0.8）使内容更具创造力和多样性。建议保持在 0.5 - 0.7 之间。" placement="top">
                      <el-icon class="text-text-placeholder cursor-help" :size="14"><InfoFilled /></el-icon>
                    </el-tooltip>
                  </div>
                </template>
                <el-slider 
                  v-model="settingsStore.aiConfig.temperature" 
                  :min="0" :max="1" :step="0.1" 
                />
              </el-form-item>

              <el-form-item>
                <template #label>
                  <div class="flex items-center gap-1.5">
                    <span>最大 Token (Max Tokens)</span>
                    <el-tooltip content="限制 AI 一次性生成的最大字符长度。如果内容被截断，可以适当调高此参数。" placement="top">
                      <el-icon class="text-text-placeholder cursor-help" :size="14"><InfoFilled /></el-icon>
                    </el-tooltip>
                  </div>
                </template>
                <el-input-number 
                  v-model="settingsStore.aiConfig.maxTokens" 
                  :min="100" :max="10000" 
                  class="!w-full"
                />
              </el-form-item>
            </div>

            <div class="mt-8 flex gap-4">
              <button 
                class="industrial-btn-primary px-8 py-2" 
                @click="saveSettings" 
                :disabled="loading"
              >
                <el-icon v-if="loading" class="animate-spin mr-1"><Loading /></el-icon>
                保存配置
              </button>
              <button 
                class="industrial-btn-secondary px-6 py-2"
                @click="testConnection" 
                :disabled="testing"
              >
                <el-icon v-if="testing" class="animate-spin mr-1"><Loading /></el-icon>
                测试连通性
              </button>
            </div>
          </el-form>
      </section>

      <!-- Git User Config -->
      <section class="lark-card bg-bg-container p-8 shadow-lark">
        <div class="flex items-center gap-3 mb-6">
          <div class="w-8 h-8 rounded-lg bg-emerald-500/10 flex items-center justify-center">
            <el-icon class="text-emerald-500" :size="18"><InfoFilled /></el-icon>
          </div>
          <h2 class="text-lg font-bold text-text-title">Git 统计偏好</h2>
        </div>

        <el-form label-position="top" class="lark-form">
          <el-form-item label="Git 用户名 (用于筛选提交记录)">
            <div class="flex gap-4 w-full">
              <el-input 
                v-model="settingsStore.gitUserName" 
                placeholder="例如: 张三"
              />
              <button @click="fetchGitUser" class="industrial-btn-secondary shrink-0">
                自动获取
              </button>
            </div>
          </el-form-item>
          <div class="mt-6">
            <button class="industrial-btn-primary px-8 py-2" @click="saveSettings" :disabled="loading">
              保存更改
            </button>
          </div>
        </el-form>
      </section>
    </div>
  </div>
</template>

<style scoped>

.lark-form :deep(.el-form-item__label) {
  color: var(--text-secondary);
  font-weight: 500;
  margin-bottom: 6px;
  font-size: 13px;
}
</style>
