import type { RouteRecordRaw } from 'vue-router'

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('@/views/Home.vue'),
    meta: { label: '首页' }
  },
  {
    path: '/program',
    name: 'program',
    meta: { hidden: true },
    children: [
      {
        path: '',
        name: 'program-list',
        component: () => import('@/views/program/index.vue'),
        meta: { label: '编程' }
      },
      {
        path: ':id',
        name: 'program-detail',
        component: () => import('@/views/program/page/Editor.vue'),
        meta: { label: '详情', hidden: true }
      }
    ]
  },
  {
    path: '/plugin',
    name: 'plugin',
    component: () => import('@/views/plugin/index.vue'),
    meta: { label: '插件' }
  },
  {
    path: '/design',
    name: 'design',
    component: () => import('@/views/design/index.vue'),
    meta: { label: '设计' }
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
    component: () => import('@/views/About.vue'),
    meta: { label: '关于' }
  }
]
