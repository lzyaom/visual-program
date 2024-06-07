import { defineComponent, type PropType } from 'vue'
import { default as Select } from './Select.vue'
import { default as SelectContent } from './SelectContent.vue'
import { default as SelectGroup } from './SelectGroup.vue'
import { default as SelectValue } from './SelectValue.vue'
import { default as SelectItem } from './SelectItem.vue'
import { default as SelectLabel } from './SelectLabel.vue'
import { default as SelectTrigger } from './SelectTrigger.vue'
import type { SelectOption, SelectGroupOption } from './type'

export default defineComponent({
  props: {
    options: {
      type: Array as PropType<Array<SelectOption | SelectGroupOption>>,
      required: true
    },
    placeholder: String
  },
  setup(props, { slots }) {
    const { options, placeholder } = props
    const renderOption = (options: Array<SelectOption | SelectGroupOption>) =>
      options.map((item) => {})

    return () => (
      <Select>
        <SelectTrigger class="w-[180px] h-9 focus:ring-1 focus:ring-offset-0">
          <SelectValue placeholder={placeholder} />
        </SelectTrigger>
        <SelectContent>{renderOption(options)}</SelectContent>
      </Select>
    )
  }
})
