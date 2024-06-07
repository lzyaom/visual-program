export interface SelectOption {
  label: string
  value: any
  disabled?: boolean
}

export interface SelectItemSlot {
  default: Function
}

export interface SelectGroupOption {
  label: string
  options: Array<SelectOption>
  disabled?: boolean
}

export type OptionType = SelectOption | SelectGroupOption
