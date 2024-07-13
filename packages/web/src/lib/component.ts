import type { App } from 'vue'

/**
 * get app instance
 * @returns
 */
const getApp = (): App => {
  // @ts-ignore
  return window.__APP__
}

/**
 * load component from local
 * @param componentName
 * @returns
 */
export const installComponent = async (componentName: string) => {
  const app = getApp()
  const isComponentInstalled = app.component(componentName)
  console.log(`Installing component ${componentName}`)

  if (isComponentInstalled) return

  try {
    const components = await import(`@/components/ui`)
    app.component(componentName, (components as any)[componentName])
  } catch (error) {
    console.error(`Failed to install component ${componentName}`, error)
  }
}

/**
 * any component can be installed from remote
 * @param componentName
 * @param url
 * @returns
 */
export const installRemoteComponent = async (componentName: string, url: string) => {
  const app = getApp()
  const isComponentInstalled = app.component(componentName)
  console.log(`Installing remote component ${componentName}`)

  if (isComponentInstalled) return

  try {
    const components = await import(url)
    app.component(componentName, (components as any)[componentName])
  } catch (error) {
    console.error(`Failed to install remote component ${componentName}`, error)
  }
}
