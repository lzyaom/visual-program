import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', () => {
  // 定义状态
  const userInfo = ref({})

  return {
    userInfo
  }
})
