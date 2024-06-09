import { defineComponent, type FunctionalComponent } from 'vue'
import { PaginationRoot, PaginationList, PaginationListItem } from 'radix-vue'
import PaginationEllipsis from './PaginationEllipsis.vue'
import PaginationFirst from './PaginationFirst.vue'
import PaginationLast from './PaginationLast.vue'
import PaginationNext from './PaginationNext.vue'
import PaginationPrev from './PaginationPrev.vue'
import { Button } from '../button'

const PaginationListRoot: FunctionalComponent<{ page: number }> = ({ page }) => {
  const slots = {
    default: ({ items }: { items: any[] }) => (
      <>
        <PaginationFirst />
        <PaginationPrev />
        {items.map((item) => {
          if (item.type === 'page') {
            return (
              <PaginationListItem key={'page-' + item.value} value={item.value} as-child>
                <Button class="w-10 h-10 p-0" variant={item.value === page ? 'default' : 'outline'}>
                  {item.value}
                </Button>
              </PaginationListItem>
            )
          }
          return <PaginationEllipsis key={item.type} />
        })}
        <PaginationNext />
        <PaginationLast />
      </>
    )
  }
  return <PaginationList class="flex items-center gap-1" v-slots={slots}></PaginationList>
}

export const Pagination = defineComponent({
  name: 'Pagination',
  props: {
    page: {
      type: Number,
      required: true,
      default: 1
    },
    size: {
      type: Number,
      default: 10
    },
    total: {
      type: Number,
      default: 0
    }
  },
  emits: {
    'update:page': (value: number) => Number.isInteger(value)
  },
  setup(props, { slots, emit }) {
    return () => (
      <div class="flex items-center justify-between">
        {slots.select && slots.select()}
        <div class="flex items-center space-x-6 lg:space-x-8">
          {slots.size && slots.size()}
          {slots.page && slots.page()}
          <PaginationRoot
            class={'pagination'}
            as={'div'}
            page={props.page}
            total={props.total}
            items-per-page={props.size}
            sibling-count={1}
            default-page={1}
            show-edges
            onUpdate:page={(value: number) => emit('update:page', value)}
          >
            <PaginationListRoot page={props.page}></PaginationListRoot>
          </PaginationRoot>
        </div>
      </div>
    )
  }
})
