import { defineComponent, type PropType } from 'vue'
import Select from './Select.vue'
import SelectContent from './SelectContent.vue'
import SelectGroup from './SelectGroup.vue'
import SelectValue from './SelectValue.vue'
import SelectItem from './SelectItem.vue'
import SelectLabel from './SelectLabel.vue'
import SelectTrigger from './SelectTrigger.vue'
import type { OptionType } from './type'

export default defineComponent({
  name: 'SelectWrapper',
  props: {
    options: {
      type: Array as PropType<Array<OptionType>>,
      required: true
    },
    placeholder: String,
    modelValue: {
      type: String
    }
  },
  emits: {
    change: (value: string) => value !== null,
    'update:model-value': (value: any) => value !== null
  },
  setup(props, { emit }) {
    const { options, placeholder } = props
    const renderOption = (options: Array<OptionType>) =>
      options.map((option) => {
        if ('options' in option) {
          return (
            <SelectGroup>
              <SelectLabel>{option.label}</SelectLabel>
              {option.options.map((item) => (
                <SelectItem value={`${item.value}`} key={item.value} disabled={item.disabled}>
                  {item.label}
                </SelectItem>
              ))}
            </SelectGroup>
          )
        }
        return (
          <SelectItem value={`${option.value}`} key={option.value} disabled={option.disabled}>
            {option.label}
          </SelectItem>
        )
      })

    const handleChange = (value: string) => {
      if (value !== props.modelValue) {
        emit('change', value)
      }
      emit('update:model-value', value)
    }
    return () => (
      <Select modelValue={props.modelValue} onUpdate:modelValue={handleChange}>
        <SelectTrigger class="w-[180px] h-9 focus:ring-1 focus:ring-offset-0">
          <SelectValue placeholder={placeholder} />
        </SelectTrigger>
        <SelectContent>{renderOption(options)}</SelectContent>
      </Select>
    )
  }
})
