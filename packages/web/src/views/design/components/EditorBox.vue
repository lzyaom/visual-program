<script setup lang="ts">
import { useDrop } from 'vue3-dnd'
import type { ComponentSchema, DropResult } from '../types'
import { ref } from 'vue'
import { installComponent } from '@/lib/component'
import RenderEngin from './RenderEngin'

const jsonschemas = ref<ComponentSchema[]>([])

const [, drop] = useDrop(() => {
  return {
    accept: 'box',
    collect: (monitor) => ({
      isOver: monitor.isOver(),
      canDrop: monitor.canDrop(),
      item: monitor.getItem<DropResult>()
    }),
    drop: async (item, monitor) => {
      if (!monitor.canDrop()) {
        return
      }
      if (monitor.didDrop()) {
        return
      }

      if (monitor.isOver()) {
        const { name } = item as DropResult
        await installComponent(name)
        jsonschemas.value.push({
          name,
          text: '1111'
        })
      }
    }
  }
})
</script>

<template>
  <div class="editor-container relative h-full" :ref="drop">
    <RenderEngin :schema="jsonschemas"></RenderEngin>
  </div>
</template>
