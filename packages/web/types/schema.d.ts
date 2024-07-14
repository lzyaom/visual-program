export interface Schema {
  // 标识
  id?: string
  name: string
  type?: string
  text?: any
  description?: string
  properties?: Record<string, Schema>
  items?: Schema
  required?: string[]
}

export interface ComponentSchema extends Schema {
  // 属性
  props?: Record<string, any>
  data?: Record<string, Schema>
  computed?: Record<string, Schema>
  watch?: Record<string, Schema>
  events?: Record<string, Schema>
  methods?: Record<string, Schema>

  //
  directives?: Record<string, Schema>
  lifecycle?: Record<string, Schema>
  slots?: Record<string, Schema>
  children?: Schema[]

  // 样式
  style?: Record<string, Schema>
  class?: Record<string, Schema>
}
