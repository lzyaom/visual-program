<script setup lang="ts">
import { toRefs } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NavigationMenu, NavigationMenuLink } from '@/components/ui'
import { cn } from '@/lib/utils'
const { path } = toRefs(useRoute())
const routes = useRouter()
  .getRoutes()
  .filter((item) => !item.meta.hidden)
  .map((item) => ({
    name: item.name,
    path: item.path,
    label: item.meta?.label
  }))
</script>
<template>
  <header
    class="app-menu sticky top-0 flex items-center justify-between h-14 px-8 z-10 border-b border-border bg-background/80 backdrop-blur-lg"
  >
    <NavigationMenu class="h-14 px-2 max-lg:space-x-4 space-x-6 text-sm font-medium" dir="ltr">
      <RouterLink
        v-for="item in routes"
        :key="item.name"
        :to="item.path"
        :class="
          cn(
            'transition-colors hover:text-foreground/80 text-foreground/60',
            path === item.path ? 'font-bold text-primary' : 'font-medium text-muted-foreground'
          )
        "
        >{{ item.label }}</RouterLink
      >
    </NavigationMenu>
  </header>
</template>
