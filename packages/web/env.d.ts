/// <reference types="vite/client" />

import type { App } from 'vue'

declare global {
  interface Window {
    __APP__: App
  }
}
