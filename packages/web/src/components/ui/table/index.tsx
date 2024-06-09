/* @jsxImportSource vue */
import { defineComponent, ref } from 'vue'
import type { PropType } from 'vue'
import { default as Table } from './Table.vue'
import { default as TableCell } from './TableCell.vue'
import { default as TableEmpty } from './TableEmpty.vue'
import { default as TableHead } from './TableHead.vue'
import { default as TableHeader } from './TableHeader.vue'
import { default as TableBody } from './TableBody.vue'
import { default as TableRow } from './TableRow.vue'
import { Checkbox } from '..'

export interface Columns {
  prop: string
  label?: string
  isSlot?: boolean
}

export const TableRoot = defineComponent({
  name: 'TableRoot',
  props: {
    columns: {
      type: Array as PropType<Array<Columns>>,
      required: true
    },
    data: {
      type: Array as PropType<Array<any>>,
      required: true
    },
    emptyText: {
      type: String,
      default: 'Empty'
    }
  },
  emits: {
    'selection-change': (rows: any[]) => Array.isArray(rows)
  },
  setup(props, { slots, emit }) {
    const { columns, data } = props
    const isAllCheck = ref(false)
    const selectedRows = ref<Set<any>>(new Set())
    const selectAll = (value: boolean) => {
      if (value) {
        for (const item of data) {
          selectedRows.value.add(item)
        }
      } else {
        selectedRows.value.clear()
      }
      isAllCheck.value = value
      emit('selection-change', Array.from(selectedRows.value))
    }

    const selectItem = (value: boolean, row: any) => {
      if (value) {
        selectedRows.value.add(row)
        if (!isAllCheck.value) {
          isAllCheck.value = true
        }
      } else {
        selectedRows.value.delete(row)
        if (selectedRows.value.size === 0) {
          isAllCheck.value = false
        }
      }
      emit('selection-change', Array.from(selectedRows.value))
    }

    const renderRow = (row: any, index: number, columns: Columns[]) => (
      <TableRow>
        {columns.map((col) => {
          if (col.prop === 'selection') {
            return (
              <TableCell>
                <Checkbox
                  name="check-item"
                  checked={selectedRows.value.has(row)}
                  onUpdate:checked={(value) => selectItem(value, row)}
                ></Checkbox>
              </TableCell>
            )
          }
          return <TableCell>{col.isSlot ? slots[col.prop]!(row) : row[col.prop]}</TableCell>
        })}
      </TableRow>
    )

    return () => (
      <Table>
        <TableHeader>
          <TableRow>
            {columns.map((item) => {
              if (item.prop === 'selection') {
                return (
                  <TableHead>
                    <Checkbox
                      name="select-all"
                      checked={isAllCheck.value}
                      onUpdate:checked={selectAll}
                    ></Checkbox>
                  </TableHead>
                )
              }
              return <TableHead>{item.label}</TableHead>
            })}
          </TableRow>
        </TableHeader>
        <TableBody>
          {data.length === 0 ? (
            <TableEmpty>{slots.empty ? slots.empty() : props.emptyText}</TableEmpty>
          ) : (
            data.map((row, index) => renderRow(row, index, columns))
          )}
        </TableBody>
      </Table>
    )
  }
})
