<script setup lang="ts">
import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useRepoStore, Repository } from '../stores/repoStore'

const repoStore = useRepoStore()

const loadRepos = async () => {
  repoStore.setLoading(true)
  try {
    const repos = await invoke<Repository[]>('list_repositories')
    repoStore.setRepositories(repos)
  } catch (err) {
    ElMessage.error('加载仓库失败: ' + err)
  } finally {
    repoStore.setLoading(false)
  }
}

const addRepo = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择 Git 仓库目录'
    })

    if (selected && !Array.isArray(selected)) {
      const isValid = await invoke<boolean>('validate_repo_path', { path: selected })
      if (!isValid) {
        ElMessage.error('该目录不是有效的 Git 仓库 (.git 目录不存在)')
        return
      }

      const newRepo = await invoke<Repository>('add_repository', { path: selected })
      repoStore.repositories.unshift(newRepo)
      ElMessage.success('仓库已成功添加')
    }
  } catch (err) {
    ElMessage.error('添加失败: ' + err)
  }
}

const toggleRepo = async (repo: Repository) => {
  try {
    await invoke('toggle_repository', { id: repo.id, isActive: !repo.isActive })
    repo.isActive = !repo.isActive
    ElMessage.success(repo.isActive ? '已启用统计 (生成日报时将默认选中)' : '已暂停统计 (生成日报时不再默认勾选)')
  } catch (err) {
    ElMessage.error('操作失败: ' + err)
  }
}

const removeRepo = (id: number) => {
  ElMessageBox.confirm('确定要移除此仓库吗？这不会删除物理文件，只会在 GitDaily 中停止追踪。', '提示', {
    confirmButtonText: '移除',
    cancelButtonText: '取消',
    type: 'warning',
    customClass: 'gitdaily-message-box'
  }).then(async () => {
    try {
      await invoke('remove_repository', { id })
      repoStore.repositories = repoStore.repositories.filter(r => r.id !== id)
      ElMessage.success('已从列表中移除')
    } catch (err) {
      ElMessage.error('移除失败: ' + err)
    }
  })
}

onMounted(() => {
  loadRepos()
})
</script>

<template>
  <div class="p-10 max-w-5xl mx-auto flex flex-col h-full overflow-y-auto custom-scrollbar bg-bg-base">
    <header class="flex justify-between items-end mb-10">
      <div>
        <h1 class="text-3xl font-bold text-text-title mb-2">仓库管理</h1>
        <p class="text-[14px] text-text-secondary">目前追踪 {{ repoStore.repositories.length }} 个代码仓库。</p>
      </div>
      <button 
        class="industrial-btn-primary !px-8 py-2" 
        @click="addRepo" 
      >
        <el-icon class="mr-1"><Plus /></el-icon>
        添加本地仓库
      </button>
    </header>

    <!-- Repo List -->
    <div v-if="repoStore.repositories.length > 0" class="flex flex-col gap-4">
      <div 
        v-for="repo in repoStore.repositories" 
        :key="repo.id"
        class="lark-card lark-card-hover bg-bg-container shadow-lark transition-all"
        :class="{'opacity-50 grayscale-[0.5]': !repo.isActive}"
      >
        <div class="flex items-center p-5 gap-5">
          <div 
            class="w-12 h-12 rounded-xl flex items-center justify-center transition-all duration-300"
            :class="[repo.isActive ? 'bg-primary/10 text-primary' : 'bg-bg-base text-text-placeholder']"
          >
            <el-icon :size="24"><Monitor /></el-icon>
          </div>

          <div class="flex-1 min-w-0">
            <h3 class="text-lg font-bold text-text-title mb-0.5 truncate">{{ repo.name }}</h3>
            <div class="flex items-center gap-1.5 text-text-secondary font-mono text-[12px] truncate">
              <el-icon :size="12"><FolderOpened /></el-icon>
              <span class="truncate">{{ repo.path }}</span>
            </div>
          </div>

          <div class="flex items-center gap-1 pr-1">
            <el-tooltip :content="repo.isActive ? '暂停统计' : '启用统计'" placement="top">
              <button 
                class="w-9 h-9 rounded-lg flex items-center justify-center transition-all"
                :class="[repo.isActive ? 'text-emerald-500 hover:bg-emerald-50' : 'text-text-placeholder hover:bg-black/5 dark:hover:bg-white/5']"
                @click="toggleRepo(repo)"
              >
                <el-icon :size="18"><component :is="repo.isActive ? 'CircleCheckFilled' : 'CircleCloseFilled'" /></el-icon>
              </button>
            </el-tooltip>
            
            <el-tooltip content="移除仓库">
              <button 
                class="w-9 h-9 rounded-lg text-text-placeholder hover:text-red-500 hover:bg-red-50 flex items-center justify-center transition-all"
                @click="removeRepo(repo.id)"
              >
                <el-icon :size="18"><Delete /></el-icon>
              </button>
            </el-tooltip>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="mt-20 flex flex-col items-center justify-center p-20 lark-card bg-bg-container shadow-lark text-center">
      <div class="w-20 h-20 bg-primary/5 rounded-full flex items-center justify-center mb-6">
        <el-icon :size="40" class="text-primary/40"><Plus /></el-icon>
      </div>
      <h3 class="text-xl font-bold text-text-title mb-2">没有任何仓库</h3>
      <p class="text-[14px] text-text-secondary max-w-sm mb-8 leading-relaxed">开始添加您的本地 Git 仓库以自动生成日报，我们将自动扫描其中的提交记录。</p>
      <button @click="addRepo" class="industrial-btn-primary !px-12 py-2">
        选择我的第一个项目
      </button>
    </div>
  </div>
</template>



<style scoped>
</style>
