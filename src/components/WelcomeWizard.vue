<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { 
  ArrowRight, 
  ArrowLeft, 
  Pointer, 
  Monitor, 
  MagicStick,
  Check
} from '@element-plus/icons-vue'

const show = ref(false)
const step = ref(1)

const checkFirstTime = () => {
  const isFirst = localStorage.getItem('gd_first_time') === null
  if (isFirst) {
    show.value = true
  }
}

const finish = () => {
  localStorage.setItem('gd_first_time', 'false')
  show.value = false
}

onMounted(() => {
  checkFirstTime()
})
</script>

<template>
  <transition name="fade">
    <div v-if="show" class="fixed inset-0 z-[9999] bg-bg-base flex items-center justify-center p-10 overflow-hidden">
      <div class="lark-card max-w-4xl w-full max-h-[700px] h-full flex flex-col items-center p-12 relative overflow-hidden text-center shadow-lark bg-bg-container">
        <!-- Progress Bar -->
        <div class="flex gap-2 mb-12">
          <div v-for="i in 4" :key="i" class="h-1.5 rounded-full transition-all duration-500" :class="[step >= i ? 'w-12 bg-primary' : 'w-4 bg-border-base']"></div>
        </div>

        <transition name="slide-fade" mode="out-in">
          <!-- Step 1: Welcome -->
          <div v-if="step === 1" key="1" class="flex flex-col items-center">
            <div class="w-20 h-20 mb-6 rounded-2xl bg-primary/10 flex items-center justify-center">
              <el-icon :size="40" class="text-primary"><MagicStick /></el-icon>
            </div>
            <h1 class="text-4xl font-bold text-text-title mb-6">欢迎使用 GitDaily</h1>
            <p class="text-text-body text-[15px] max-w-md leading-relaxed">
              这是您的专属开发者助手。我们将通过分析 Git 提交记录，利用 AI 为您自动生成专业、深刻的项目日报。
            </p>
          </div>

          <!-- Step 2: How it works -->
          <div v-else-if="step === 2" key="2" class="flex flex-col items-center">
            <h2 class="text-2xl font-bold text-text-title mb-10">仅需三步，告别周报烦恼</h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
              <div v-for="(item, idx) in [
                { icon: Monitor, title: '添加仓库', desc: '关联您的本地 Git 项目' },
                { icon: Pointer, title: '一键生成', desc: '自动抓取并整理提交信息' },
                { icon: Check, title: '成果展示', desc: 'AI 生成 MD 文档供直接使用' }
              ]" :key="idx" class="flex flex-col items-center">
                <div class="w-14 h-14 rounded-xl bg-primary/5 flex items-center justify-center mb-4 text-primary">
                  <el-icon :size="20"><component :is="item.icon" /></el-icon>
                </div>
                <h4 class="font-bold text-text-title mb-2">{{ item.title }}</h4>
                <p class="text-[12px] text-text-secondary">{{ item.desc }}</p>
              </div>
            </div>
          </div>

          <!-- Step 3: Privacy -->
          <div v-else-if="step === 3" key="3" class="flex flex-col items-center">
            <div class="w-16 h-16 mb-6 rounded-full bg-emerald-50 flex items-center justify-center">
               <el-icon :size="32" class="text-emerald-500"><Check /></el-icon>
            </div>
            <h2 class="text-2xl font-bold text-text-title mb-6">隐私第一</h2>
            <p class="text-text-body text-[15px] max-w-md leading-relaxed mb-10">
              所有代码分析都在本地进行。我们仅会将整理后的提交摘要发送至您指定的 AI 节点（如 OpenAI 或 DeepSeek），绝不会泄露任何物理代码文件。
            </p>
          </div>

          <!-- Step 4: Ready -->
          <div v-else-if="step === 4" key="4" class="flex flex-col items-center">
            <h2 class="text-3xl font-bold text-text-title mb-6">准备就绪！</h2>
            <p class="text-text-body text-[15px] mb-10">建议您首先前往“设置”配置您的 API Key。</p>
            <button @click="finish" class="industrial-btn-primary !px-12 !py-3">开启我的高效工作</button>
          </div>
        </transition>

        <div class="mt-auto w-full flex justify-between items-center" v-if="step < 4">
          <button v-if="step > 1" @click="step--" class="industrial-btn-secondary text-[13px] border-none !bg-transparent !text-text-placeholder hover:!text-text-title">
            <el-icon class="mr-1"><ArrowLeft /></el-icon>
            上一步
          </button>
          <div v-else></div>
          <button @click="step++" class="industrial-btn-primary text-[13px]">
            继续
            <el-icon class="ml-1"><ArrowRight /></el-icon>
          </button>
        </div>
      </div>
    </div>
  </transition>
</template>

<style scoped>
@reference "../styles/main.css";

.animate-bounce.duration-slow {
  animation-duration: 3s;
}

.fade-enter-active, .fade-leave-active {
  transition: opacity 1s ease;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}

.slide-fade-enter-active {
  transition: all 0.6s cubic-bezier(0.16, 1, 0.3, 1);
}
.slide-fade-leave-active {
  transition: all 0.4s cubic-bezier(0.7, 0, 0.84, 0);
}
.slide-fade-enter-from {
  transform: translateX(30px);
  opacity: 0;
}
.slide-fade-leave-to {
  transform: translateX(-30px);
  opacity: 0;
}
</style>
