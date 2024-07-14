<script setup lang="ts">
import { useDrag } from 'vue3-dnd'
import type { DropResult, MaterialItem } from '#/drag'

const props = defineProps<MaterialItem>()

const [, drag] = useDrag({
  type: 'box',
  item: () => ({ name: props.name }),
  collect: (monitor) => ({
    isDragging: monitor.isDragging(),
    handlerId: monitor.getHandlerId()
  }),
  end: (item, monitor) => {
    const dropResult = monitor.getDropResult<DropResult>()
    if (item && dropResult) {
      console.log(dropResult)
      console.log(`You dropped ${item.name} into ${dropResult.name}`)
    }
  }
})
</script>

<template>
  <div
    class="material-item flex items-center h-8 bg-orange-200 rounded-8px cursor-pointer transition-all-3s-ease-in-out hover:bg-e0e0e0"
    :ref="drag"
    draggable="false"
  >
    <div class="material-icon">
      <i :class="`iconfont icon-${type}`"></i>
    </div>
    <div class="material-name">{{ name }}</div>
  </div>
</template>
