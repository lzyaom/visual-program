<script setup lang="ts">
import AppMenu from './components/AppMenu.vue'
import AppMain from './components/AppMain.vue'

defineOptions({
  name: 'Layout'
})
</script>

<template>
  <AppMenu />
  <AppMain />
</template>
