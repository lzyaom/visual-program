<script setup lang="ts">
import { ref } from 'vue'
import {
  SquarePlusIcon,
  TrashIcon,
  EllipsisIcon,
  Share2Icon,
  PencilIcon,
  CopyIcon,
  StarIcon
  // WatchIcon,
  // CheckCircleIcon,
  // PencilLineIcon
} from 'lucide-vue-next'
import {
  Input,
  Table,
  Select,
  type OptionType,
  Button,
  Pagination,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator,
  DropdownMenuShortcut
} from '@/components/ui'
import type { Columns } from '@/components/ui'

defineOptions({
  name: 'ProgramListPage'
})

const programList = [
  {
    task: 'INV001',
    title: 'Paid',
    status: 'Done',
    descript: 'Credit Card'
  },
  {
    task: 'INV002',
    title: 'Pending',
    status: 'Draft',
    descript: 'PayPal'
  },
  {
    task: 'INV003',
    title: 'Unpaid',
    status: 'Done',
    descript: 'Bank Transfer'
  },
  {
    task: 'INV004',
    title: 'Paid',
    status: 'In Process',
    descript: 'Credit Card'
  },
  {
    task: 'INV005',
    title: 'Paid',
    status: 'Draft',
    descript: 'PayPal'
  },
  {
    task: 'INV006',
    title: 'Pending',
    status: 'Done',
    descript: 'Bank Transfer'
  },
  {
    task: 'INV007',
    title: 'Unpaid',
    status: 'In Process',
    descript: 'Credit Card'
  }
]

const statusOptions: OptionType[] = [
  { label: 'Apple', value: 'apple' },
  { label: 'Banana', value: 'banana' },
  { label: 'Blueberry', value: 'blueberry' },
  { label: 'Grapes', value: 'grapes' },
  { label: 'Pineapple', value: 'pineapple' }
]
const pageSizeOption: OptionType[] = [
  { label: '10', value: 10 },
  { label: '20', value: 20 },
  { label: '30', value: 30 },
  { label: '40', value: 40 },
  { label: '50', value: 50 }
]

const filterItem = ref({
  keyword: '',
  status: ''
})

const columns: Columns[] = [
  {
    prop: 'selection',
    attrs: {
      class: 'w-[60px]'
    }
  },
  {
    prop: 'task',
    label: 'Task'
  },
  {
    prop: 'title',
    label: 'Title'
  },
  {
    prop: 'descript',
    label: 'Descript'
  },
  {
    prop: 'status',
    label: 'Status',
    icon: 'CheckCircleIcon'
  },
  {
    prop: 'action',
    isSlot: !0,
    attrs: {
      class: 'w-[90px]'
    }
  }
]

const pages = ref({
  current: 1,
  total: 20,
  size: '10'
})
const selectRows = ref<any[]>([])
const createProgram = () => {}
const selctionChange = (rows: any[]) => {
  selectRows.value = rows
}
const changePageSize = () => {
  pages.value.current = 1
}
const handleEdit = () => {}
const handleCopy = () => {}
const handleFavorite = () => {}
const handleShare = () => {}
const handleDelete = () => {}
</script>
<template>
  <div class="page-program px-8 pt-4">
    <div class="flex items-center justify-between space-y-2">
      <div class="descript">
        <h2 class="text-2xl font-bold tracking-tighter">Welcome</h2>
        <p class="text-muted-foreground">这里是当前已创建的程序！</p>
      </div>
      <div class="operate space-x-2">
        <Button class="border-dashed h-8" variant="outline" size="sm" @click="createProgram">
          <SquarePlusIcon class="mr-2 w-4 h-4" />
          添加
        </Button>
        <Button class="border-dashed h-8" variant="outline" size="sm">
          <TrashIcon class="mr-2 w-4 h-4" />
          删除
        </Button>
      </div>
    </div>
    <div class="space-y-4">
      <div class="filter flex items-center mt-8 space-x-2">
        <Input
          class="w-[150px] lg:w-[250px]"
          placeholder="Filter tasks..."
          :model-value="filterItem.keyword"
        />
        <Select
          :options="statusOptions"
          name="fiter-item"
          v-model:model-value="filterItem.status"
          placeholder="Select a Status"
        >
        </Select>
      </div>
      <div class="border rounded-md">
        <Table :columns="columns" :data="programList" @selection-change="selctionChange">
          <template #action>
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button variant="ghost" class="flex h-8 w-8 p-0 data-[state=open]:bg-muted">
                  <EllipsisIcon class="h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end" class="w-[160px]">
                <DropdownMenuItem class="cursor-pointer" @click="handleEdit">
                  Edit
                  <DropdownMenuShortcut><PencilIcon class="w-3 h-3" /></DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem class="cursor-pointer" @click="handleCopy">
                  Copy
                  <DropdownMenuShortcut><CopyIcon class="w-3 h-3" /></DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem class="cursor-pointer" @click="handleFavorite">
                  Favorite
                  <DropdownMenuShortcut><StarIcon class="w-3 h-3" /></DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem class="cursor-pointer" @click="handleShare">
                  Share
                  <DropdownMenuShortcut><Share2Icon class="w-3 h-3" /></DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem class="cursor-pointer" @click="handleDelete">
                  Delete
                  <DropdownMenuShortcut>⌘⌫</DropdownMenuShortcut>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </template>
        </Table>
      </div>
      <Pagination v-model:page="pages.current" :total="pages.total" :size="+pages.size">
        <template #select>
          <div class="flex-1 text-sm text-muted-foreground">
            {{ selectRows.length }} of {{ pages.size }} row(s) selected.
          </div>
        </template>
        <template #size>
          <div class="flex items-center space-x-2">
            <p class="text-sm font-medium">Rows per page</p>
            <Select
              name="page-size"
              placeholder="Select a size"
              :options="pageSizeOption"
              v-model:model-value="pages.size"
              @change="changePageSize"
            />
          </div>
        </template>
        <template #page>
          <div class="flex w-[100px] items-center justify-center text-sm font-medium">
            Page {{ pages.current }} of {{ Math.ceil(pages.total / +pages.size) }}
          </div>
        </template>
      </Pagination>
    </div>
  </div>
</template>
