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
import { type Setting } from '@/bindings'

const props = defineProps<{
  setting: Setting
}>()

const emit = defineEmits<{
  (e: 'update:theme', value: string): void
}>()

const theme = useTheme()
const selectedTheme = ref<string | null>('System')

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

const themeOptions: ThemeOption[] = [
  { title: 'Light', value: 'light' },
  { title: 'Dark', value: 'dark' },
  { title: 'System', value: 'system' },
  ...customThemeNames,
]

onMounted(() => {
  selectedTheme.value = props.setting.theme
  console.log(themeOptions)
  changeTheme(selectedTheme.value ? selectedTheme.value : 'System')
})

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
  emit('update:theme', newTheme)
}
</script>
