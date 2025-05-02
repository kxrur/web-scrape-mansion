import { commands } from '@/bindings'
import { type ThemeDefinition, type ThemeInstance } from 'vuetify'

export const customThemes: { [key: string]: ThemeDefinition } = {
  myCustomLightTheme: {
    dark: false,
    colors: {
      background: '#FFFFFF',
      surface: '#FFFFFF',
      primary: '#6200EE',
      'primary-darken-1': '#3700B3',
      secondary: '#03DAC6',
      'secondary-darken-1': '#018786',
      error: '#B00020',
      info: '#2196F3',
      success: '#4CAF50',
      warning: '#FB8C00',
    },
  },
  mehe: {
    dark: true,
    colors: {
      background: '#12252E', // Rose taupe
      surface: '#19323C', // Gunmetal
      primary: '#A93F55', // Amaranth purple
      'primary-darken-1': '#8C5E58', // Beaver
      secondary: '#F3F7F0', // Mint cream
      'secondary-darken-1': '#6200EE',
      error: '#CF6679',
      info: '#03DAC6',
      success: '#81C784',
      warning: '#FFC107',
    },
  },
}

export function changeTheme(currentTheme: ThemeInstance, newTheme: string) {
  switch (newTheme) {
    case 'light':
    case 'dark':
      currentTheme.global.name.value = newTheme
      break
    case 'system':
      const prefersDark = window.matchMedia(
        '(prefers-color-scheme: dark)'
      ).matches
      currentTheme.global.name.value = prefersDark ? 'dark' : 'light'
      break
    default:
      if (customThemes[newTheme]) {
        currentTheme.global.name.value = newTheme
      }
  }
}

export function initTheme(currentTheme: ThemeInstance) {
  commands
    .getSettingById(0)
    .then((theSettings) => {
      console.log('settings', theSettings)
      if (theSettings.status == 'ok') {
        changeTheme(
          currentTheme,
          theSettings.data.theme ? theSettings.data.theme : 'System'
        )
      }
    })
    .catch((error) => {
      console.error('Promise rejected with error: ' + error)
    })
}
