import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('@/views/HomeView.vue'),
      meta: { label: '首页' }
    },
    {
      path: '/program',
      name: 'program',
      component: () => import('@/views/program/index.vue'),
      meta: { label: '编程' }
    },
    {
      path: '/plugin',
      name: 'plugin',
      component: () => import('@/views/plugin/index.vue'),
      meta: { label: '插件' }
    },
    {
      path: '/system',
      name: 'system',
      component: () => import('@/views/system/index.vue'),
      meta: { label: '系统' }
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('@/views/AboutView.vue'),
      meta: { label: '关于' }
    }
  ]
})

export default router
