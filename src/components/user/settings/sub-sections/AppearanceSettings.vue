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
      @update:modelValue="handleThemeChange"
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
import { customThemes, changeTheme } from '@/plugins/CustomThemes'
import { type Setting } from '@/bindings'
import { useTheme } from 'vuetify'

const props = defineProps<{
  setting: Setting
}>()

const emit = defineEmits<{
  (e: 'update:theme', value: string): void
}>()

const selectedTheme = ref<string | null>('System')
const theme = useTheme()

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
})

function handleThemeChange(newTheme: string) {
  changeTheme(theme, newTheme)
  emit('update:theme', newTheme)
}
</script>
