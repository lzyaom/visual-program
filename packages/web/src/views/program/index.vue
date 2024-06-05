<script setup lang="ts">
import { ref } from 'vue'
import { SquarePlus, Trash } from 'lucide-vue-next'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
  Button,
  Pagination,
  PaginationEllipsis,
  PaginationFirst,
  PaginationLast,
  PaginationList,
  PaginationListItem,
  PaginationNext,
  PaginationPrev
} from '@/components/ui'
const invoices = [
  {
    invoice: 'INV001',
    paymentStatus: 'Paid',
    totalAmount: '$250.00',
    paymentMethod: 'Credit Card'
  },
  {
    invoice: 'INV002',
    paymentStatus: 'Pending',
    totalAmount: '$150.00',
    paymentMethod: 'PayPal'
  },
  {
    invoice: 'INV003',
    paymentStatus: 'Unpaid',
    totalAmount: '$350.00',
    paymentMethod: 'Bank Transfer'
  },
  {
    invoice: 'INV004',
    paymentStatus: 'Paid',
    totalAmount: '$450.00',
    paymentMethod: 'Credit Card'
  },
  {
    invoice: 'INV005',
    paymentStatus: 'Paid',
    totalAmount: '$550.00',
    paymentMethod: 'PayPal'
  },
  {
    invoice: 'INV006',
    paymentStatus: 'Pending',
    totalAmount: '$200.00',
    paymentMethod: 'Bank Transfer'
  },
  {
    invoice: 'INV007',
    paymentStatus: 'Unpaid',
    totalAmount: '$300.00',
    paymentMethod: 'Credit Card'
  }
]

const statusOptions = [
  { label: 'Apple', value: 'apple' },
  { label: 'Banana', value: 'banana' },
  { label: 'Blueberry', value: 'blueberry' },
  { label: 'Grapes', value: 'grapes' },
  { label: 'Pineapple', value: 'pineapple' }
]

const pages = ref({
  current: 1,
  total: 20,
  size: '10'
})
const changePageSize = (size: string) => {
  pages.value.size = size
}

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
      <div class="filter mt-8">
        <Select>
          <SelectTrigger class="w-[180px]">
            <SelectValue placeholder="Select a status" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectLabel>Fruits</SelectLabel>
              <SelectItem v-for="item in statusOptions" :key="item.label" :value="item.value">{{
                item.label
              }}</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>
      <div class="border rounded-md">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead></TableHead>
              <TableHead>Task</TableHead>
              <TableHead>Title</TableHead>
              <TableHead>Descript</TableHead>
              <TableHead>Status</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="invoice in invoices" :key="invoice.invoice">
              <TableCell></TableCell>
              <TableCell>
                {{ invoice.invoice }}
              </TableCell>
              <TableCell>{{ invoice.paymentStatus }}</TableCell>
              <TableCell>{{ invoice.paymentMethod }}</TableCell>
              <TableCell>
                {{ invoice.totalAmount }}
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </div>
      <div class="flex items-center justify-between">
        <div class="flex-1 text-sm text-muted-foreground">
          0 of {{ pages.size }} row(s) selected.
        </div>
        <div class="flex items-center space-x-6 lg:space-x-8">
          <div class="flex items-center space-x-2">
            <p class="text-sm font-medium">Rows per page</p>
            <Select :model-value="pages.size" @update:model-value="changePageSize">
              <SelectTrigger class="h-8 w-[70px]">
                <SelectValue :placeholder="pages.size" />
              </SelectTrigger>
              <SelectContent side="top">
                <SelectItem
                  v-for="pageSize in [10, 20, 30, 40, 50]"
                  :key="pageSize"
                  :value="`${pageSize}`"
                >
                  {{ pageSize }}
                </SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div class="flex w-[100px] items-center justify-center text-sm font-medium">
            Page {{ pages.current }} of
            {{ Math.ceil(pages.total / +pages.size) }}
          </div>
          <Pagination
            v-slot="{ page }"
            :total="pages.total"
            :sibling-count="1"
            show-edges
            :default-page="pages.current"
          >
            <PaginationList v-slot="{ items }" class="flex items-center gap-1">
              <PaginationFirst />
              <PaginationPrev />
              <template v-for="(item, index) in items">
                <PaginationListItem
                  v-if="item.type === 'page'"
                  :key="index"
                  :value="item.value"
                  as-child
                >
                  <Button
                    class="w-10 h-10 p-0"
                    :variant="item.value === page ? 'default' : 'outline'"
                  >
                    {{ item.value }}
                  </Button>
                </PaginationListItem>
                <PaginationEllipsis v-else :key="item.type" :index="index" />
              </template>
              <PaginationNext />
              <PaginationLast />
            </PaginationList>
          </Pagination>
        </div>
      </div>
    </div>
  </div>
</template>
