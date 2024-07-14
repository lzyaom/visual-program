export interface Schema {
  // 标识
  id?: string
  name: string
  type?: string
  description?: string
  properties?: Record<string, Schema>
  items?: Schema
  required?: string[]
}

export interface ComponentSchema {
  id?: string
  name: string
  text?: string
  // 属性
  props?: Record<string, any>
  //
  children?: Schema[]
}
