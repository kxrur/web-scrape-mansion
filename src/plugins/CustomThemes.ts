import { type ThemeDefinition } from 'vuetify'

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
      background: '#19323C', // Gunmetal
      surface: '#8C5E58', // Rose taupe
      primary: '#A93F55', // Amaranth purple
      'primary-darken-1': '#AB8476', // Beaver
      secondary: '#F3F7F0', // Mint cream
      'secondary-darken-1': '#6200EE',
      error: '#CF6679',
      info: '#03DAC6',
      success: '#81C784',
      warning: '#FFC107',
    },
  },
}
