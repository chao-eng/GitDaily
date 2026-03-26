<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'

interface Prompt {
  id: number;
  title: string;
  content: string;
  is_builtin: boolean;
  is_default: boolean;
  created_at: string;
}

const prompts = ref<Prompt[]>([])
const loading = ref(false)
const dialogVisible = ref(false)
const editingPrompt = ref<Partial<Prompt>>({
  id: 0,
  title: '',
  content: ''
})

const loadPrompts = async () => {
  loading.value = true
  try {
    const result = await invoke<Prompt[]>('list_prompts')
    prompts.value = result
  } catch (err) {
    ElMessage.error('加载模板出错: ' + err)
  } finally {
    loading.value = false
  }
}

const openDialog = (prompt?: Prompt) => {
  if (prompt) {
    editingPrompt.value = { ...prompt }
  } else {
    editingPrompt.value = { id: 0, title: '', content: '', is_builtin: false }
  }
  dialogVisible.value = true
}

const savePrompt = async () => {
  if (!editingPrompt.value.title || !editingPrompt.value.content) {
    ElMessage.warning('请填写完整的标题和内容')
    return
  }

  try {
    if (editingPrompt.value.id === 0) {
      const newPrompt = await invoke<Prompt>('create_prompt', { 
        title: editingPrompt.value.title, 
        content: editingPrompt.value.content 
      })
      prompts.value.unshift(newPrompt)
    } else {
      await invoke('update_prompt', { 
        id: editingPrompt.value.id, 
        title: editingPrompt.value.title, 
        content: editingPrompt.value.content 
      })
      const index = prompts.value.findIndex(p => p.id === editingPrompt.value.id)
      if (index !== -1) {
        prompts.value[index].title = editingPrompt.value.title!
        prompts.value[index].content = editingPrompt.value.content!
      }
    }
    ElMessage.success('模板保存成功')
    dialogVisible.value = false
  } catch (err) {
    ElMessage.error('保存失败: ' + err)
  }
}

const deletePrompt = (id: number) => {
  ElMessageBox.confirm('确定要删除此自定义库吗？内置模板无法删除。', '警告', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning',
    customClass: 'gitdaily-message-box'
  }).then(async () => {
    try {
      await invoke('delete_prompt', { id })
      prompts.value = prompts.value.filter(p => p.id !== id)
      ElMessage.success('模板已删除')
    } catch (err) {
      ElMessage.error('删除失败: ' + err)
    }
  })
}

const setDefault = async (id: number) => {
  try {
    await invoke('set_default_prompt', { id })
    prompts.value.forEach(p => p.is_default = (p.id === id))
    ElMessage.success('默认模板已设置')
  } catch (err) {
    ElMessage.error('操作失败: ' + err)
  }
}

onMounted(() => {
  loadPrompts()
})
</script>

<template>
  <div class="p-10 max-w-5xl mx-auto flex flex-col">
    <header class="flex justify-between items-end mb-10">
      <div>
        <h1 class="text-3xl font-bold text-text-title mb-2">模板管理</h1>
        <p class="text-[14px] text-text-secondary">目前拥有 {{ prompts.length }} 套 AI 日报提示词模板。</p>
      </div>
      <button 
        class="industrial-btn-primary !px-8 py-2" 
        @click="openDialog()" 
      >
        <el-icon class="mr-1"><Plus /></el-icon>
        新建模板
      </button>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 pb-10">
      <div 
        v-for="prompt in prompts" 
        :key="prompt.id"
        class="lark-card lark-card-hover bg-bg-container flex flex-col shadow-lark transition-all border-b-2 border-b-transparent"
        :class="{'!border-b-primary': prompt.is_default}"
      >
        <div class="p-6 flex flex-col h-full">
          <div class="flex justify-between items-start mb-4">
            <div class="flex items-center gap-2 min-w-0">
              <h3 class="text-lg font-bold text-text-title truncate">{{ prompt.title }}</h3>
              <div class="flex gap-1 shrink-0">
                <el-tag v-if="prompt.is_builtin" size="small" effect="plain" class="!bg-emerald-50 !text-emerald-600 !border-emerald-200">内置</el-tag>
                <el-tag v-if="prompt.is_default" size="small" effect="plain" class="!bg-primary/10 !text-primary !border-primary/20">默认</el-tag>
              </div>
            </div>
            
            <button 
              class="w-7 h-7 rounded-md hover:bg-black/5 dark:hover:bg-white/5 flex items-center justify-center transition-colors text-text-placeholder"
              :class="{'text-primary': prompt.is_default}"
              @click="setDefault(prompt.id)"
            >
              <el-icon :size="16"><component :is="prompt.is_default ? 'StarFilled' : 'Star'" /></el-icon>
            </button>
          </div>
          
          <div class="bg-bg-base/50 rounded-lg p-4 font-mono text-[11px] text-text-secondary mb-6 h-32 overflow-hidden relative fade-bottom leading-relaxed border border-border-base/50">
            {{ prompt.content }}
          </div>
          
          <div class="mt-auto flex justify-end gap-1.5 pt-4 border-t border-border-base">
            <button 
              class="text-[12px] font-bold text-text-secondary hover:text-primary px-3 py-1.5 rounded-lg hover:bg-primary/5 transition-all flex items-center"
              @click="openDialog(prompt)"
            >
              <el-icon class="mr-1"><Edit /></el-icon>
              查看详情
            </button>
            <button 
              v-if="!prompt.is_builtin"
              class="text-[12px] font-bold text-text-secondary hover:text-red-500 px-3 py-1.5 rounded-lg hover:bg-red-50 transition-all flex items-center"
              @click="deletePrompt(prompt.id)"
            >
              <el-icon class="mr-1"><Delete /></el-icon>
              移除
            </button>
          </div>
        </div>
      </div>
    </div>

    <el-dialog
      v-model="dialogVisible"
      :title="editingPrompt.id === 0 ? '新建日报模板' : '编辑模板详情'"
      width="680px"
    >
      <div class="space-y-6 py-2">
        <el-form label-position="top" class="lark-form">
          <el-form-item label="模板标题">
            <el-input 
              v-model="editingPrompt.title" 
              placeholder="例如: 技术深度版"
            />
          </el-form-item>
          <el-form-item label="System Prompt (系统提示词)">
            <el-input 
              v-model="editingPrompt.content" 
              type="textarea" 
              :rows="12"
              placeholder="输入指导 AI 生成内容的具体指示词..."
              class="font-mono !text-[13px]"
            />
            <p class="text-[12px] text-text-placeholder mt-2 leading-relaxed">提示: 可以指示 AI 重点关注代码重构、测试覆盖率或业务价值。我们会自动在用户消息中追加提交记录。</p>
          </el-form-item>
        </el-form>
      </div>
      <template #footer>
        <div class="flex justify-end gap-4 p-4 border-t border-border-base">
          <button @click="dialogVisible = false" class="industrial-btn-secondary px-6 py-2">取消</button>
          <button 
            @click="savePrompt" 
            class="industrial-btn-primary px-8 py-2"
          >
            保存模板入库
          </button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>

.fade-bottom {
  mask-image: linear-gradient(to bottom, black 60%, transparent 100%);
}

.lark-form :deep(.el-form-item__label) {
  color: var(--text-secondary);
  font-weight: 500;
  margin-bottom: 6px;
  font-size: 13px;
}
</style>
