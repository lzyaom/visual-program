import { defineComponent } from 'vue'
import {
  DownloadIcon,
  UploadIcon,
  SettingsIcon,
  Undo2Icon,
  Redo2Icon,
  RefreshCcwIcon,
  Code2Icon,
  ScissorsIcon,
  ClipboardIcon,
  CopyIcon
} from 'lucide-vue-next'

import { Button } from '@/components/ui'

export default defineComponent({
  name: 'ToolBar',
  setup() {
    return () => (
      <div class="toolbar flex gap-4">
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <Undo2Icon size={16} />
        </Button>
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <Redo2Icon size={16} />
        </Button>
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <ScissorsIcon size={16} />
        </Button>
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <CopyIcon size={16} />
        </Button>
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <ClipboardIcon size={16} />
        </Button>
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <RefreshCcwIcon size={16} />
        </Button>
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <Code2Icon size={16} />
        </Button>
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <DownloadIcon size={16} />
        </Button>
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <UploadIcon size={16} />
        </Button>
        <Button class="bg-gray-600 hover:bg-gray-600" size="xs">
          <SettingsIcon size={16} />
        </Button>
      </div>
    )
  }
})
