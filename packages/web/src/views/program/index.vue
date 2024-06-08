<script setup lang="ts">
import { ref } from 'vue'
import { SquarePlus, Trash } from 'lucide-vue-next'
import { Input, Table, Select, type OptionType, Button, Pagination } from '@/components/ui'
import type { Columns } from '@/components/ui'
const invoices = [
  {
    task: 'INV001',
    title: 'Paid',
    status: '$250.00',
    descript: 'Credit Card'
  },
  {
    task: 'INV002',
    title: 'Pending',
    status: '$150.00',
    descript: 'PayPal'
  },
  {
    task: 'INV003',
    title: 'Unpaid',
    status: '$350.00',
    descript: 'Bank Transfer'
  },
  {
    task: 'INV004',
    title: 'Paid',
    status: '$450.00',
    descript: 'Credit Card'
  },
  {
    task: 'INV005',
    title: 'Paid',
    status: '$550.00',
    descript: 'PayPal'
  },
  {
    task: 'INV006',
    title: 'Pending',
    status: '$200.00',
    descript: 'Bank Transfer'
  },
  {
    task: 'INV007',
    title: 'Unpaid',
    status: '$300.00',
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
    prop: 'checkout'
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
    label: 'Status'
  },
  {
    prop: 'action',
    label: ''
  }
]

const pages = ref({
  current: 1,
  total: 20,
  size: '10'
})

const createProgram = () => {}
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
          <SquarePlus class="mr-2 w-4 h-4" />
          添加
        </Button>
        <Button class="border-dashed h-8" variant="outline" size="sm">
          <Trash class="mr-2 w-4 h-4" />
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
        <Table :columns="columns" :data="invoices"></Table>
      </div>
      <Pagination v-model:page="pages.current" :total="pages.total" :size="+pages.size">
        <template #size>
          <p class="text-sm font-medium">Rows per page</p>
          <Select
            name="page-size"
            placeholder="Select a size"
            :options="pageSizeOption"
            v-model:model-value="pages.size"
          >
          </Select>
        </template>
      </Pagination>
    </div>
  </div>
</template>
