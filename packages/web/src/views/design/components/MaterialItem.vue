<script setup lang="ts">
interface MaterialItem {
  type: string
  name: string
}
defineProps<MaterialItem>()
</script>

<template>
  <div
    class="material-item flex items-center justify-center w-100% h-100% bg-f0f0f0 rounded-8px cursor-pointer transition-all-3s-ease-in-out hover:bg-e0e0e0"
  >
    <slot>
      <div class="material-icon">
        <i :class="`iconfont icon-${type}`"></i>
      </div>
      <div class="material-name">{{ name }}</div>
    </slot>
  </div>
</template>
