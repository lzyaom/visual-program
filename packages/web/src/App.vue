<script setup lang="ts">
import { DndProvider } from 'vue3-dnd'
import { HTML5Backend } from 'react-dnd-html5-backend'

import { RouterView } from 'vue-router'
import Layout from '@/layout/index.vue'
</script>

<template>
  <DndProvider :backend="HTML5Backend">
    <Layout></Layout>
    <Teleport to="main">
      <RouterView />
    </Teleport>
  </DndProvider>
</template>
