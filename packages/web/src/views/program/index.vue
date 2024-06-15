<script setup lang="ts">
import { onBeforeMount, ref, toRaw } from 'vue'
import {
  SquarePlusIcon,
  TrashIcon,
  EllipsisIcon,
  Share2Icon,
  PencilIcon,
  CopyIcon,
  StarIcon,
  EyeIcon
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
  DropdownMenuShortcut,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  Label,
  Textarea
} from '@/components/ui'
import type { Columns } from '@/components/ui'
import { getProgram, createProgram, deleteProgram, updateProgram } from '@/api'

interface Program {
  _id?: any
  title: string
  descript: string
  status: string
}
defineOptions({
  name: 'ProgramListPage'
})

const programList = ref<Array<Program>>([])
const newFile = ref<Program>({
  title: '',
  status: '',
  descript: ''
})
const statusOptions: OptionType[] = [
  { label: 'Draft', value: 'Draft' },
  { label: 'In Process', value: 'InProcess' },
  { label: 'Done', value: 'Done' }
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
const selectRows = ref<Program[]>([])
const selctionChange = (rows: Program[]) => {
  selectRows.value = rows
}
const changePageSize = () => {
  pages.value.current = 1
}
const isShowCreate = ref<boolean>(false)
const toggleShow = (value: boolean) => {
  isShowCreate.value = value
}
const handleView = (row: Program) => {}
const handleEdit = (row: Program) => {
  newFile.value = { ...toRaw(row) }
  toggleShow(true)
}
const handleCopy = (row: Program) => {}
const handleFavorite = (row: Program) => {}
const handleShare = (row: Program) => {}
const handleDelete = async (row: Program) => {
  try {
    await deleteProgram(row._id.$oid)
    await getProgramList()
  } catch (error) {
    console.error(error)
  }
}
/**
 * 获取程序列表
 */
const getProgramList = async () => {
  try {
    const { data } = await getProgram({
      page: pages.value.current,
      size: pages.value.size,
      ...filterItem.value
    })
    programList.value = data
  } catch (error) {
    programList.value = []
  }
}

/**
 * 保存新建或编辑的程序文件
 */
const saveProgram = async () => {
  try {
    let id = newFile.value._id
    if (id) {
      id = id.$oid
      const params = newFile.value
      delete params._id
      await updateProgram(id, params)
    } else {
      await createProgram({
        ...newFile.value,
        create_time: new Date().toString()
      })
    }
    newFile.value = {
      title: '',
      descript: '',
      status: ''
    }
    await getProgramList()
    toggleShow(false)
  } catch (error) {
    console.error(error)
  }
}
onBeforeMount(getProgramList)
</script>
<template>
  <div class="page-program px-8 pt-4">
    <div class="flex items-center justify-between space-y-2">
      <div class="descript">
        <h2 class="text-2xl font-bold tracking-tighter">Welcome</h2>
        <p class="text-muted-foreground">这里是当前已创建的程序！</p>
      </div>
      <div class="operate space-x-2">
        <Button class="border-dashed h-8" variant="outline" size="sm" @click="toggleShow(true)">
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
          placeholder="Filter title..."
          v-model:model-value="filterItem.keyword"
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
          <template #action="row">
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button variant="ghost" class="flex h-8 w-8 p-0 data-[state=open]:bg-muted">
                  <EllipsisIcon class="h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end" class="w-[160px]">
                <DropdownMenuItem class="cursor-pointer" @click="handleView(row)">
                  View
                  <DropdownMenuShortcut><EyeIcon class="w-3 h-3" /></DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem class="cursor-pointer" @click="handleEdit(row)">
                  Edit
                  <DropdownMenuShortcut><PencilIcon class="w-3 h-3" /></DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem class="cursor-pointer" @click="handleCopy(row)">
                  Copy
                  <DropdownMenuShortcut><CopyIcon class="w-3 h-3" /></DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem class="cursor-pointer" @click="handleFavorite(row)">
                  Favorite
                  <DropdownMenuShortcut><StarIcon class="w-3 h-3" /></DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem class="cursor-pointer" @click="handleShare(row)">
                  Share
                  <DropdownMenuShortcut><Share2Icon class="w-3 h-3" /></DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem class="cursor-pointer" @click="handleDelete(row)">
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
    <Dialog v-model:open="isShowCreate">
      <DialogContent class="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Create File</DialogTitle>
          <DialogDescription>
            To create a new program file, fill in the following fields
          </DialogDescription>
        </DialogHeader>
        <div class="grid gap-4 py-4">
          <div class="grid grid-cols-4 items-center gap-4">
            <Label for="title" class="text-right"> Title </Label>
            <Input
              class="col-span-3"
              id="title"
              placeholder="Please enter a title"
              v-model:model-value="newFile.title"
            />
          </div>
          <div class="grid grid-cols-4 items-center gap-4">
            <Label for="status" class="text-right"> Status </Label>
            <Select
              class="col-span-3"
              :options="statusOptions"
              name="fiter-item"
              id="status"
              v-model:model-value="newFile.status"
              placeholder="Select a Status"
            />
          </div>
          <div class="grid grid-cols-4 items-center gap-4">
            <Label for="descript" class="text-right"> Descript </Label>
            <Textarea
              class="col-span-3"
              id="descript"
              placeholder="please enter a descript"
              v-model:model-value="newFile.descript"
            ></Textarea>
          </div>
        </div>
        <DialogFooter>
          <Button variant="outline" @click="toggleShow(false)"> Cancel </Button>
          <Button type="submit" @click="saveProgram"> Save </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>
