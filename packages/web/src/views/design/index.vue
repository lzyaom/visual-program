<script setup lang="ts">
import { SaveIcon, MonitorPlayIcon, WrenchIcon, PlusIcon } from 'lucide-vue-next'
import {
  ResizablePanelGroup,
  ResizablePanel,
  ResizableHandle,
  Button,
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
  ScrollArea
} from '@components'
import { ToolBar, DraggableBox, EditorBox, AttrsConfigure } from './components'
import { useData } from './hooks'
defineOptions({
  name: 'Design'
})
const { project, materials } = useData()
</script>

<template>
  <div class="design h-full bg-slate-50">
    <div class="design-header flex items-center justify-between h-12 px-4">
      <div class="project-info flex items-center space-x-4">
        <div class="project-cover flex flex-col">
          <img class="w-6 h-6 rounded-full" :src="project.cover" alt="" />
        </div>
        <div class="project-profile flex flex-col">
          <div class="title text-lg font-bold text-gray-700">
            <span class="">{{ project.name }}</span>
            <span class="">{{ project.type === 'command' ? '指令' : '插件' }}</span>
          </div>
          <span
            class="project-description max-w-64 overflow-hidden text-xs text-gray-400 text-ellipsis"
            >{{ project.description }}</span
          >
        </div>
      </div>
      <div class="actions flex items-center space-x-16">
        <ToolBar />
        <div class="space-x-4">
          <Button class="bg-red-600 hover:bg-red-600" size="xs">
            <PlusIcon :size="16" />
          </Button>
          <Button class="bg-yellow-600 hover:bg-yellow-600" size="xs">
            <SaveIcon :size="16" />
          </Button>
          <Button class="bg-blue-600 hover:bg-blue-600" size="xs">
            <WrenchIcon :size="16" />
          </Button>
          <Button class="bg-green-600 hover:bg-green-600" size="xs">
            <MonitorPlayIcon :size="16" />
          </Button>
        </div>
      </div>
    </div>
    <div class="design-container">
      <ResizablePanelGroup direction="horizontal" class="h-full">
        <ResizablePanel :defaultSize="25">
          <ScrollArea class="h-full">
            <Accordion type="single" class="h-full px-4" collapsible>
              <AccordionItem
                v-for="material in materials"
                :key="material.name"
                :value="material.name"
              >
                <AccordionTrigger>{{ material.titile }}</AccordionTrigger>
                <AccordionContent class="space-y-2 pb-2">
                  <DraggableBox
                    v-for="item in material.items"
                    :key="item.id"
                    :name="item.name"
                  ></DraggableBox>
                </AccordionContent>
              </AccordionItem>
            </Accordion>
          </ScrollArea>
        </ResizablePanel>
        <ResizableHandle with-handle />
        <ResizablePanel :defaultSize="50">
          <EditorBox />
        </ResizablePanel>
        <ResizableHandle with-handle />
        <ResizablePanel>
          <AttrsConfigure />
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  </div>
</template>

<style lang="css">
.design-container {
  height: calc(100% - 3rem);
}
</style>
