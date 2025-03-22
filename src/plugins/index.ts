/**
 * plugins/index.ts
 *
 * Automatically included in `./src/main.ts`
 */

// Plugins
import vuetify from './vuetify'
import pinia from '../stores'
import router from '../router'
import { devtools } from '@vue/devtools'

if (process.env.NODE_ENV === 'development') {
  devtools.connect()
}

// Types
import type { App } from 'vue'

export function registerPlugins(app: App) {
  app.use(vuetify).use(router).use(pinia)
}
