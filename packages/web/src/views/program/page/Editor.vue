<script setup lang="ts">
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui'
</script>

<template>
  <div class="editor-container">
    <div class="tool-bar">
      <div class="back"></div>
      <h2 class="title"></h2>
      <div class="tool-bar">
        <button>New</button>
        <button>Open</button>
        <button>Save</button>
        <button>Forward</button>
        <button>Back</button>
        <button>Import</button>
        <button>Export</button>
        <button>Help</button>
        <button>Settings</button>
      </div>
      <div class="operate">
        <button>Run</button>
        <button>Stop</button>
        <button>Debug</button>
      </div>
    </div>
    <ResizablePanelGroup direction="horizontal">
      <ResizablePanel>
        <div class="command-panel"></div>
      </ResizablePanel>
      <ResizableHandle with-handle />
      <ResizablePanel>
        <div class="command-tree"></div>
      </ResizablePanel>
      <ResizableHandle with-handle />
      <ResizablePanel>
        <div class="property-panel"></div>
      </ResizablePanel>
    </ResizablePanelGroup>
  </div>
</template>
