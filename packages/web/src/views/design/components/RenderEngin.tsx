import { defineComponent, h, resolveComponent, type PropType, type VNode } from 'vue'

import type { ComponentSchema } from '../types'

export default defineComponent({
  name: 'RenderEngin',
  props: {
    schema: {
      type: Array as PropType<Array<ComponentSchema>>,
      default: () => [],
      required: true
    }
  },
  setup(props) {
    const renderChild = (children: ComponentSchema[]): VNode[] => {
      return children.map((child) =>
        h(
          resolveComponent(child.name, true),
          {
            ...child.props,
            key: child.id
          },
          child.children && child.children.length > 0
            ? renderChild(child.children)
            : () => child.text
        )
      )
    }
    return () => renderChild(props.schema)
  }
})
