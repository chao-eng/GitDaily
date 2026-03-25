<script setup lang="ts">
import SideNav from "../components/SideNav.vue";
</script>

<template>
  <div class="flex h-screen w-screen overflow-hidden bg-bg-base font-body text-text-body relative p-1.5">
    <!-- Simplified Lark-style Background Decor -->
    <div class="absolute inset-0 pointer-events-none overflow-hidden rounded-xl border border-border-base/50">
      <div class="absolute top-0 right-0 w-[40%] h-[40%] rounded-full bg-primary/5 blur-[100px]"></div>
    </div>

    <!-- Sidebar -->
    <SideNav />
    
    <!-- Main Content -->
    <main class="flex-1 relative overflow-hidden flex flex-col z-10">
      <router-view v-slot="{ Component }">
        <transition 
          name="page-reveal" 
          mode="out-in"
          appear
        >
          <div :key="$route.path" class="h-full w-full overflow-y-auto custom-scrollbar">
            <component :is="Component" />
          </div>
        </transition>
      </router-view>
    </main>
  </div>
</template>

<style>
.page-reveal-enter-active {
  transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
}
.page-reveal-enter-from {
  opacity: 0;
  transform: translateY(20px) scale(0.98);
}
.page-reveal-leave-active {
  transition: all 0.4s cubic-bezier(0.7, 0, 0.84, 0);
}
.page-reveal-leave-to {
  opacity: 0;
  transform: translateY(-10px) scale(1.02);
}

/* Global Transition for staggered elements */
.stagger-item {
  animation: reveal 0.8s cubic-bezier(0.16, 1, 0.3, 1) both;
}

@keyframes reveal {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
