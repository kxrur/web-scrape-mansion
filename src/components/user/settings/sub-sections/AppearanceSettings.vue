<template>
  <div>
    <div class="text-h6 mb-4">Appearance Settings</div>

    <v-select
      label="Theme"
      :items="themeOptions"
      variant="outlined"
      class="mb-4"
      color="primary"
      v-model="selectedTheme"
      @update:modelValue="changeTheme"
    ></v-select>

    <v-switch
      label="Enable Animations"
      color="primary"
      hide-details
      class="mb-4"
    ></v-switch>
  </div>
</template>

<script setup lang="ts">
import { useTheme } from 'vuetify'
import { customThemes } from '@/plugins/CustomThemes'

const theme = useTheme()
const selectedTheme = ref('System')

interface ThemeOption {
  title: string
  value: string
}

const customThemeNames: ThemeOption[] = Object.keys(customThemes).map(
  (name) => ({
    title: name.replace(/([A-Z])/g, ' $1').trim(),
    value: name,
  })
)

// Available theme options
const themeOptions: ThemeOption[] = [
  { title: 'Light', value: 'light' },
  { title: 'Dark', value: 'dark' },
  { title: 'System', value: 'system' },
  ...customThemeNames,
]

console.log(themeOptions)

function changeTheme(newTheme: string) {
  switch (newTheme) {
    case 'light':
    case 'dark':
      theme.global.name.value = newTheme
      break
    case 'system':
      const prefersDark = window.matchMedia(
        '(prefers-color-scheme: dark)'
      ).matches
      theme.global.name.value = prefersDark ? 'dark' : 'light'
      break
    default:
      if (customThemes[newTheme]) {
        theme.global.name.value = newTheme
      }
  }
}

onMounted(() => {
  selectedTheme.value = 'system'
  changeTheme('system')
})
</script>
