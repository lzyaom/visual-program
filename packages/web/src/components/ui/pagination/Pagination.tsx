import { defineComponent, toRefs, type FunctionalComponent, type PropType } from 'vue'
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
    },
    onChange: {
      type: Function as PropType<(page: number) => any>
    }
  },
  emits: {
    'update:page': (value: number) => Number.isInteger(value),
    change: (page: number) => Number.isInteger(page)
  },
  setup(props, { slots, emit }) {
    const { page, size, total } = toRefs(props)

    return () => (
      <div class="flex items-center justify-between">
        <div class="flex-1 text-sm text-muted-foreground">0 of {size} row(s) selected.</div>
        <div class="flex items-center space-x-6 lg:space-x-8">
          <div class="flex items-center space-x-2">{slots.size && slots.size()}</div>
          <div class="flex w-[100px] items-center justify-center text-sm font-medium">
            Page {page.value} of {Math.ceil(total.value / size.value)}
          </div>
          <PaginationRoot
            page={page.value}
            total={total.value}
            items-per-page={size.value}
            sibling-count={1}
            default-page={1}
            show-edges
            onUpdate:page={(value: number) => emit('update:page', value)}
          >
            <PaginationListRoot page={page.value}></PaginationListRoot>
          </PaginationRoot>
        </div>
      </div>
    )
  }
})
