import { ref } from 'vue'

export const useData = () => {
  const project = ref({
    name: 'Move',
    id: '1234567890',
    type: 'command',
    description: '项目描述',
    cover: 'https://via.placeholder.com/150'
  })

  const materials = [
    {
      name: 'layout',
      titile: '布局',
      items: [{ id: '1', name: '图片' }]
    },
    {
      name: 'component',
      titile: '组件',
      items: [
        { id: '1', name: 'Input' },
        { id: '2', name: 'Button' },
        { id: '3', name: 'Switch' },
        { id: '4', name: 'Select' },
        { id: '5', name: 'Text' },
        { id: '6', name: 'Radio' },
        { id: '7', name: 'Checkbox' },
        { id: '8', name: 'TimePicker' },
        { id: '9', name: 'DatePicker' },
        { id: '10', name: 'ColorPicker' },
        { id: '11', name: 'Slider' },
        { id: '12', name: 'Rate' },
        { id: '13', name: 'Upload' },
        { id: '14', name: 'Form' },
        { id: '15', name: 'Table' },
        { id: '16', name: 'Tree' },
        { id: '17', name: 'Pagination' },
        { id: '18', name: 'Breadcrumb' },
        { id: '19', name: 'Tabs' },
        { id: '20', name: 'Dialog' },
        { id: '21', name: 'Notification' },
        { id: '22', name: 'Loading' },
        { id: '23', name: 'Tooltip' },
        { id: '24', name: 'Popover' },
        { id: '25', name: 'Dropdown' },
        { id: '26', name: 'Steps' },
        { id: '27', name: 'Card' },
        { id: '28', name: 'Tag' },
        { id: '29', name: 'Avatar' },
        { id: '30', name: 'Badge' },
        { id: '31', name: 'Timeline' },
        { id: '32', name: 'Calendar' }
      ]
    },
    {
      name: 'chart',
      titile: '图表',
      items: [
        { id: '1', name: '柱状图' },
        { id: '2', name: '折线图' },
        { id: '3', name: '饼图' },
        { id: '4', name: '散点图' },
        { id: '5', name: '雷达图' },
        { id: '6', name: 'K线图' },
        { id: '7', name: '热力图' },
        { id: '8', name: '地图' },
        { id: '9', name: '仪表盘' },
        { id: '10', name: '漏斗图' },
        { id: '11', name: '桑基图' },
        { id: '12', name: '树图' },
        { id: '13', name: '旭日图' },
        { id: '14', name: '平行坐标' }
      ]
    }
  ]
  return {
    project,
    materials
  }
}
