<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { useSettingsStore } from '../stores/settingsStore'
import { useRepoStore } from '../stores/repoStore'


const settingsStore = useSettingsStore()
const repoStore = useRepoStore()
const loading = ref(false)
const testing = ref(false)
const triggering = ref(false)
const prompts = ref<Array<{ id: number; title: string }>>([])

const loadSettings = async () => {
  loading.value = true
  try {
    await settingsStore.loadSettings()
    
    // 直接加载仓库数据，避免 Store 热更新冲突
    const repos = await invoke<any[]>('list_repositories')
    repoStore.setRepositories(repos.map(r => ({
      id: r.id,
      name: r.name,
      path: r.path,
      isActive: r.isActive ?? true,
      createdAt: r.createdAt || ''
    })))

    // 加载提示词模板
    const promptList = await invoke<any[]>('list_prompts')
    console.log('Loaded prompts for settings:', promptList)
    
    if (Array.isArray(promptList)) {
      prompts.value = promptList.map(p => ({ 
        id: p.id, 
        title: p.title 
      }))
    }
  } catch (err) {
    console.error('Failed to load settings or prompts:', err)
    ElMessage.error('初始化设置失败: ' + err)
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
    // 保存 scheduler 配置
    await invoke('update_scheduler_config', { config: settingsStore.scheduler })
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
  loading.value = true
  try {
    const name = await invoke<string>('get_git_user_name', { repoPath: null })
    if (name) {
      settingsStore.gitUserName = name
      ElMessage.success('已自动获取 Git 用户名: ' + name)
    } else {
      ElMessage.warning('未能从 Git 配置中获取到用户名，请手动填写')
    }
  } catch (err) {
    console.warn('Could not auto-fetch git user:', err)
    ElMessage.error('获取失败，请确保已安装 Git 环境')
  } finally {
    loading.value = false
  }
}

const triggerNow = async () => {
  triggering.value = true
  try {
    const result = await invoke<string>('trigger_auto_generation')
    ElMessage.success('自动生成完成: ' + (result.length > 50 ? result.substring(0, 50) + '...' : result))
  } catch (err) {
    ElMessage.error('生成失败: ' + err)
  } finally {
    triggering.value = false
  }
}

const weekDays = [
  { label: '周日', value: 0 },
  { label: '周一', value: 1 },
  { label: '周二', value: 2 },
  { label: '周三', value: 3 },
  { label: '周四', value: 4 },
  { label: '周五', value: 5 },
  { label: '周六', value: 6 },
]

onMounted(() => {
  loadSettings()
})
</script>

<template>
  <div class="p-10 max-w-4xl mx-auto flex flex-col">
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
                type="button"
                class="industrial-btn-primary px-8 py-2" 
                @click.prevent="saveSettings" 
                :disabled="loading"
              >
                <el-icon v-if="loading" class="animate-spin mr-1"><Loading /></el-icon>
                保存配置
              </button>
              <button 
                type="button"
                class="industrial-btn-secondary px-6 py-2"
                @click.prevent="testConnection" 
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
          <el-form-item label="Git 用户名 (用于筛选提交记录，为空则不限制)">
            <div class="flex gap-4 w-full">
              <el-input 
                v-model="settingsStore.gitUserName" 
                placeholder="例如: 张三"
              />
              <button 
                type="button"
                @click.prevent="fetchGitUser" 
                class="industrial-btn-secondary shrink-0"
                :disabled="loading"
              >
                <el-icon v-if="loading" class="animate-spin mr-1"><Loading /></el-icon>
                自动获取
              </button>
            </div>
          </el-form-item>
          <div class="mt-6">
            <button 
              type="button"
              class="industrial-btn-primary px-8 py-2" 
              @click.prevent="saveSettings" 
              :disabled="loading"
            >
              <el-icon v-if="loading" class="animate-spin mr-1"><Loading /></el-icon>
              保存更改
            </button>
          </div>
        </el-form>
      </section>

      <!-- Scheduler Configuration -->
      <section class="lark-card bg-bg-container p-8 shadow-lark">
        <div class="flex items-center gap-3 mb-8">
          <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center">
            <el-icon class="text-primary" :size="20"><Timer /></el-icon>
          </div>
          <h2 class="text-lg font-bold text-text-title">定时自动生成</h2>
        </div>

        <el-form :model="settingsStore.scheduler" label-position="top" class="lark-form">
          <el-form-item label="启用自动生成">
            <el-switch v-model="settingsStore.scheduler.enabled" />
            <div class="text-text-placeholder text-xs mt-1">启用后，GitDaily 将在后台常驻，到时间自动生成日报/周报</div>
          </el-form-item>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-10 gap-y-2">
            <el-form-item label="执行频率">
              <el-select v-model="settingsStore.scheduler.frequency" placeholder="选择执行频率">
                <el-option label="每天" value="daily" />
                <el-option label="每周某一天" value="weekly" />
                <el-option label="仅工作日（周一至周五）" value="workdays" />
              </el-select>
            </el-form-item>

            <el-form-item v-if="settingsStore.scheduler.frequency === 'weekly'" label="执行星期">
              <el-select v-model="settingsStore.scheduler.dayOfWeek" placeholder="选择星期几">
                <el-option v-for="day in weekDays" :key="day.value" :label="day.label" :value="day.value" />
              </el-select>
            </el-form-item>

            <el-form-item label="执行时间">
              <div class="flex gap-2">
                <el-input-number
                  v-model="settingsStore.scheduler.hour"
                  :min="0" :max="23"
                  class="flex-1"
                  placeholder="小时"
                />
                <span class="flex items-center text-text-secondary">:</span>
                <el-input-number
                  v-model="settingsStore.scheduler.minute"
                  :min="0" :max="59"
                  class="flex-1"
                  placeholder="分钟"
                />
              </div>
            </el-form-item>
          </div>

          <el-form-item label="选择仓库">
            <el-checkbox-group v-model="settingsStore.scheduler.repoIds">
              <div class="flex flex-wrap gap-2">
                <el-checkbox v-for="repo in repoStore.repositories.filter(r => r.isActive)" :key="repo.id" :label="repo.id">
                  {{ repo.name }}
                </el-checkbox>
              </div>
              <div v-if="repoStore.repositories.filter(r => r.isActive).length === 0" class="text-text-placeholder text-xs">
                没有启用的仓库，请先在仓库管理中添加
              </div>
            </el-checkbox-group>
            <div class="text-text-placeholder text-xs mt-1">不选择则自动包含所有启用的仓库</div>
          </el-form-item>

          <el-form-item label="提示词模板">
            <el-select v-model="settingsStore.scheduler.promptId" placeholder="选择提示词模板" clearable>
              <el-option v-for="prompt in prompts" :key="prompt.id" :label="prompt.title" :value="prompt.id" />
            </el-select>
            <div class="text-text-placeholder text-xs mt-1">不选择则使用默认模板</div>
          </el-form-item>

          <div class="mt-8 flex gap-4">
            <button
              type="button"
              class="industrial-btn-primary px-8 py-2"
              @click.prevent="saveSettings"
              :disabled="loading"
            >
              <el-icon v-if="loading" class="animate-spin mr-1"><Loading /></el-icon>
              保存配置
            </button>
            <button
              type="button"
              class="industrial-btn-secondary px-6 py-2"
              @click.prevent="triggerNow"
              :disabled="triggering"
            >
              <el-icon v-if="triggering" class="animate-spin mr-1"><Loading /></el-icon>
              立即测试
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
