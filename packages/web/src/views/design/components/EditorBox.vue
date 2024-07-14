<script setup lang="ts">
import { ref } from 'vue'
import { useDrop } from 'vue3-dnd'
import RenderEngin from './RenderEngin'
import { installComponent } from '@/lib'
import type { DropResult } from '#/drag'
import type { ComponentSchema } from '#/schema'

const jsonschemas = ref<ComponentSchema[]>([])

const [, drop] = useDrop(() => ({
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
    }
  }
}))
</script>

<template>
  <div class="editor-container relative h-full" :ref="drop">
    <RenderEngin :schema="jsonschemas"></RenderEngin>
  </div>
</template>
