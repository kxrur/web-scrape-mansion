<template>
  <div>
    <div class="text-h6 mb-4">Data Settings</div>

    <div class="text-h7 mb-2">Storage Folder</div>
    <v-text-field
      :model-value="dataFolder"
      label="Data Folder"
      variant="solo-inverted"
      readonly
      @click="openDirectoryPicker"
    ></v-text-field>

    <div class="text-h7 mb-2">Sync</div>
    <v-switch label="Enable Data Sync" color="primary" hide-details></v-switch>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'

const dataFolder = ref<string | null>(null)

const openDirectoryPicker = async () => {
  try {
    // Open a directory picker
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Data Folder',
    })

    if (selected) {
      // Store the selected directory path
      dataFolder.value = selected as string
      console.log(dataFolder.value)
    }
  } catch (error) {
    console.error('Error selecting directory:', error)
  }
}

defineExpose({ dataFolder })
</script>
