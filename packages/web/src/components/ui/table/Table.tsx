import { defineComponent, ref } from 'vue'
import type { PropType } from 'vue'
import Table from './Table.vue'
import TableCell from './TableCell.vue'
import TableEmpty from './TableEmpty.vue'
import TableHead from './TableHead.vue'
import TableHeader from './TableHeader.vue'
import TableBody from './TableBody.vue'
import TableRow from './TableRow.vue'
import { Checkbox } from '..'
import type { Columns } from './type'

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
      default: 'No results'
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
                  <TableHead class={item.attrs?.class}>
                    <Checkbox
                      name="select-all"
                      checked={isAllCheck.value}
                      onUpdate:checked={selectAll}
                    ></Checkbox>
                  </TableHead>
                )
              }
              return <TableHead class={item.attrs?.class}>{item.label}</TableHead>
            })}
          </TableRow>
        </TableHeader>
        <TableBody>
          {data.length === 0 ? (
            <TableEmpty class={`text-center`} colspan={columns.length}>
              {slots.empty ? slots.empty() : props.emptyText}
            </TableEmpty>
          ) : (
            data.map((row, index) => renderRow(row, index, columns))
          )}
        </TableBody>
      </Table>
    )
  }
})
