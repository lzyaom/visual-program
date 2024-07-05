<script setup lang="ts">
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui'
</script>

<template>
  <div class="design">
    <div class="tool-bar">
      <div class="back"></div>
      <h2 class="title">设计</h2>
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
        <button>Build</button>
        <button>Preview</button>
        <button>Deploy</button>
      </div>
    </div>
    <ResizablePanelGroup direction="horizontal">
      <ResizablePanel>
        <div class="material-panel">
          <div class="material-list">
            <div class="material-item">
              <div class="material-icon">
                <i class="iconfont icon-text"></i>
              </div>
              <div class="material-name">文本</div>
            </div>
            <div class="material-item">
              <div class="material-icon">
                <i class="iconfont icon-image"></i>
              </div>
              <div class="material-name">图片</div>
            </div>
            <div class="material-item">
              <div class="material-icon">
                <i class="iconfont icon-video"></i>
              </div>
              <div class="material-name">视频</div>
            </div>
            <div class="material-item">
              <div class="material-icon">
                <i class="iconfont icon-audio"></i>
              </div>
              <div class="material-name">音频</div>
            </div>
            <div class="material-item">
              <div class="material-icon">
                <i class="iconfont icon-map"></i>
              </div>
              <div class="material-name">地图</div>
            </div>
            <div class="material-item">
              <div class="material-icon">
                <i class="iconfont icon-chart"></i>
              </div>
              <div class="material-name">图表</div>
            </div>
            <div class="material-item">
              <div class="material-icon">
                <i class="iconfont icon-form"></i>
              </div>
              <div class="material-name">表单</div>
            </div>
          </div>
        </div>
      </ResizablePanel>
      <ResizableHandle with-handle />
      <ResizablePanel>
        <div class="editor-container"></div>
      </ResizablePanel>
      <ResizableHandle with-handle />
      <ResizablePanel>
        <div class="property-panel"></div>
      </ResizablePanel>
    </ResizablePanelGroup>
  </div>
</template>
