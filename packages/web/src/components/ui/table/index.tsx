/* @jsxImportSource vue */
import { defineComponent } from 'vue'
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

export default defineComponent({
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
  setup(props, { slots }) {
    const { columns, data } = props

    const renderRow = (data: Array<any>, columns: Columns[]) => {
      if (data.length === 0) {
        return <TableEmpty>{slots.empty ? slots.empty() : props.emptyText}</TableEmpty>
      }
      return data.map((row) => (
        <TableRow>
          {columns.map((col) => {
            if (col.prop === 'checkout') {
              return (
                <TableCell>
                  <Checkbox></Checkbox>
                </TableCell>
              )
            }
            return <TableCell>{col.isSlot ? slots[col.prop]!(row) : row[col.prop]}</TableCell>
          })}
        </TableRow>
      ))
    }

    return () => (
      <Table>
        <TableHeader>
          <TableRow>
            {columns.map((item) => {
              if (item.prop === 'checkout') {
                return (
                  <TableHead>
                    <Checkbox></Checkbox>
                  </TableHead>
                )
              }
              return <TableHead>{item.label}</TableHead>
            })}
          </TableRow>
        </TableHeader>
        <TableBody>{renderRow(data, columns)}</TableBody>
      </Table>
    )
  }
})
