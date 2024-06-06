<script setup lang="ts">
import { RouterView } from 'vue-router'
import Layout from '@/layout/index.vue'
</script>

<template>
  <Layout></Layout>
  <Teleport to="main">
    <RouterView />
  </Teleport>
</template>
