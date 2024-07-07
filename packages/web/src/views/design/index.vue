<script setup lang="ts">
import { ref } from 'vue'
import { SaveIcon, MonitorPlayIcon, WrenchIcon, PlusIcon } from 'lucide-vue-next'
import {
  ResizablePanelGroup,
  ResizablePanel,
  ResizableHandle,
  Button,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
  ScrollArea
} from '@/components/ui'
import { ToolBar, ContentBox, EditorBox } from './components'

const project = ref({
  name: 'Move',
  id: '1234567890',
  type: 'command',
  description: '项目描述',
  cover: 'https://via.placeholder.com/150'
})

const materials = [
  {
    name: 'layout',
    titile: '布局',
    items: [{ id: '1', name: '图片' }]
  },
  {
    name: 'component',
    titile: '组件',
    items: [
      { id: '1', name: 'Input' },
      { id: '2', name: 'Button' },
      { id: '3', name: 'Switch' },
      { id: '4', name: 'Select' },
      { id: '5', name: 'Text' },
      { id: '6', name: 'Radio' },
      { id: '7', name: 'Checkbox' },
      { id: '8', name: 'TimePicker' },
      { id: '9', name: 'DatePicker' },
      { id: '10', name: 'ColorPicker' },
      { id: '11', name: 'Slider' },
      { id: '12', name: 'Rate' },
      { id: '13', name: 'Upload' },
      { id: '14', name: 'Form' },
      { id: '15', name: 'Table' },
      { id: '16', name: 'Tree' },
      { id: '17', name: 'Pagination' },
      { id: '18', name: 'Breadcrumb' },
      { id: '19', name: 'Tabs' },
      { id: '20', name: 'Dialog' },
      { id: '21', name: 'Notification' },
      { id: '22', name: 'Loading' },
      { id: '23', name: 'Tooltip' },
      { id: '24', name: 'Popover' },
      { id: '25', name: 'Dropdown' },
      { id: '26', name: 'Steps' },
      { id: '27', name: 'Card' },
      { id: '28', name: 'Tag' },
      { id: '29', name: 'Avatar' },
      { id: '30', name: 'Badge' },
      { id: '31', name: 'Timeline' },
      { id: '32', name: 'Calendar' }
    ]
  },
  {
    name: 'chart',
    titile: '图表',
    items: [
      { id: '1', name: '柱状图' },
      { id: '2', name: '折线图' },
      { id: '3', name: '饼图' },
      { id: '4', name: '散点图' },
      { id: '5', name: '雷达图' },
      { id: '6', name: 'K线图' },
      { id: '7', name: '热力图' },
      { id: '8', name: '地图' },
      { id: '9', name: '仪表盘' },
      { id: '10', name: '漏斗图' },
      { id: '11', name: '桑基图' },
      { id: '12', name: '树图' },
      { id: '13', name: '旭日图' },
      { id: '14', name: '平行坐标' }
    ]
  }
]
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
                  <ContentBox
                    v-for="item in material.items"
                    :key="item.id"
                    :name="item.name"
                  ></ContentBox>
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
          <Tabs default-value="style" class="w-full">
            <TabsList class="grid w-full grid-cols-3 bg-slate-200">
              <TabsTrigger value="style">样式</TabsTrigger>
              <TabsTrigger value="property">属性</TabsTrigger>
              <TabsTrigger value="rule">规则</TabsTrigger>
            </TabsList>
            <TabsContent value="style"> Make changes to your account here. </TabsContent>
            <TabsContent value="property"> Change your password here. </TabsContent>
            <TabsContent value="rule"> Change your password here. </TabsContent>
          </Tabs>
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
