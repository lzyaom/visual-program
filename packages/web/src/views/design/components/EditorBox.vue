<script setup lang="ts">
import { nextTick, onMounted, ref } from 'vue'
import { useDrop, type XYCoord } from 'vue3-dnd'
import RenderEngin from './RenderEngin'
import { genId, installComponent } from '@/lib'
import type { ComponentSchema } from '#/schema'

const schema = ref<ComponentSchema[]>([])

const offset = { x: 0, y: 0, h: 0, w: 0 }

const getOffset = ({ x, y }: XYCoord) => {
  x = Math.min(Math.max(x - offset.x, 0), offset.w)
  y = Math.min(Math.max(y - offset.y, 0), offset.h)
  return {
    x,
    y
  }
}
const [, drop] = useDrop(() => ({
  accept: 'box',
  collect: (monitor) => ({
    isOver: monitor.isOver(),
    canDrop: monitor.canDrop(),
    item: monitor.getItem<ComponentSchema>()
  }),
  drop: async (_, monitor) => {
    if (!monitor.canDrop()) {
      return
    }
    if (monitor.didDrop()) {
      return
    }

    const { x, y } = getOffset(monitor.getClientOffset()!)

    const item = monitor.getItem<ComponentSchema>()
    if (monitor.isOver()) {
      const { name } = item
      await installComponent(name)
      schema.value.push({
        id: genId(),
        name,
        props: {
          style: {
            top: `${y}px`,
            left: `${x}px`
          }
        }
      })
    }
  }
}))

onMounted(() => {
  nextTick(() => {
    const { offsetWidth, offsetHeight, offsetLeft, offsetTop } =
      document.getElementById('editor-container')!
    offset.x = offsetLeft
    offset.y = offsetTop
    offset.w = offsetWidth
    offset.h = offsetHeight
  })
})
</script>

<template>
  <div class="editor-wrapper h-full px-4">
    <div class="relative w-full h-full bg-white" id="editor-container" :ref="drop">
      <RenderEngin v-model="schema"></RenderEngin>
    </div>
  </div>
</template>
