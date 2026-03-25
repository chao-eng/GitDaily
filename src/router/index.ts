import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/dashboard'
    },
    {
      path: '/dashboard',
      name: 'dashboard',
      component: () => import('../views/DashboardPage.vue')
    },
    {
      path: '/generate',
      name: 'generate',
      component: () => import('../views/GeneratePage.vue')
    },
    {
      path: '/history',
      name: 'history',
      component: () => import('../views/HistoryPage.vue')
    },
    {
      path: '/repos',
      name: 'repos',
      component: () => import('../views/RepoManagePage.vue')
    },
    {
      path: '/prompts',
      name: 'prompts',
      component: () => import('../views/PromptManagePage.vue')
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/SettingsPage.vue')
    }
  ]
})

export default router
