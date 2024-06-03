<script setup lang="ts">
import { toRefs } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NavigationMenu, NavigationMenuLink } from '@/components/ui/navigation-menu'
import { cn } from '@/lib/utils'
defineOptions({
  name: 'AppLayout'
})
const { path } = toRefs(useRoute())
const routes = useRouter()
  .getRoutes()
  .map((item) => ({
    name: item.name,
    path: item.path,
    label: item.meta?.label
  }))
</script>

<template>
  <NavigationMenu class="h-14 px-2 max-lg:space-x-4 space-x-6 text-sm font-medium" dir="ltr">
    <NavigationMenuLink
      v-for="item in routes"
      :key="item.name"
      :href="item.path"
      :active="path === item.path"
      :class="
        cn(
          'transition-colors hover:text-foreground/80 text-foreground/60',
          path === item.path ? 'font-bold text-primary' : 'font-medium text-muted-foreground'
        )
      "
      >{{ item.label }}</NavigationMenuLink
    >
  </NavigationMenu>
</template>
