<script setup lang="ts">
import { useDrop } from 'vue3-dnd'
import type { DropResult } from '../types'
import { ref } from 'vue'

const list = ref<string[]>([])

const [, drop] = useDrop(() => {
  return {
    accept: 'box',
    collect: (monitor) => ({
      isOver: monitor.isOver(),
      canDrop: monitor.canDrop(),
      item: monitor.getItem<DropResult>()
    }),
    drop: (item, monitor) => {
      list.value.push((item as any).name)
    }
  }
})
</script>

<template>
  <div class="editor-container h-full" :ref="drop">
    <div class="item" v-for="item in list" :key="item">{{ item }}</div>
  </div>
</template>
