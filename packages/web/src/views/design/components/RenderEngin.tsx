import { defineComponent, h, resolveComponent, type PropType, type VNode } from 'vue'
import { cn } from '@lib'
import type { ComponentSchema } from '#/schema'

export default defineComponent({
  name: 'RenderEngin',
  props: {
    modelValue: {
      type: Array as PropType<Array<ComponentSchema>>,
      default: () => [],
      required: true
    }
  },
  emits: {
    'update:modelValue': (schema: ComponentSchema[]) => Array.isArray(schema)
  },
  setup(props) {
    const renderChild = (children: ComponentSchema[]): VNode[] => {
      return children.map((child) =>
        h(
          resolveComponent(child.name, true),
          {
            ...child.props,
            class: cn('view-item absolute', child.props?.class),
            key: child.id
          },
          child.children && child.children.length > 0
            ? renderChild(child.children)
            : child.text
              ? () => child.text
              : () => child.name
        )
      )
    }
    return () => renderChild(props.modelValue)
  }
})
