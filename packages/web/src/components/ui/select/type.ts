export interface SelectOption {
  lable: string
  value: any
  disabled: boolean
}

export interface SelectItemSlot {
  default: Function
}

export interface SelectGroupOption {
  label: string
  options: Array<SelectOption>
  disabled: boolean
}
