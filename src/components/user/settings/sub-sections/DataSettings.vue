<template>
  <div>
    <div class="text-h6 mb-4">Data Settings</div>

    <div class="text-h7 mb-2">Storage Folder</div>
    <!-- FIXME: display the Data Folder path for the current user -->
    <v-text-field
      :model-value="dataFolder"
      label="Data Folder"
      variant="solo-inverted"
      readonly
      :disabled="isDialogOpen"
      @click="openDirectoryPicker"
    ></v-text-field>

    <div class="text-h7 mb-2">Sync</div>
    <v-switch
      label="Enable Data Sync"
      color="primary"
      hide-details
      :disabled="isDialogOpen"
    ></v-switch>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { commands } from '@/bindings'

defineProps<{
  isDialogOpen: boolean
}>()

const emit = defineEmits<{
  (e: 'update:isDialogOpen', value: boolean): void
  (e: 'update:dataFolder', value: string): void
}>()

const dataFolder = ref<string | null>(null)

commands
  .getSettingById(0)
  .then((settings) => {
    console.log(settings)
    if (settings.status == 'ok') {
      dataFolder.value = settings.data.db_path
    }
  })
  .catch((error) => {
    console.error('Promise rejected with error: ' + error)
  })

const openDirectoryPicker = async () => {
  try {
    emit('update:isDialogOpen', true) // Notify parent that dialog is open

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
      emit('update:dataFolder', dataFolder.value)
    }
  } catch (error) {
    console.error('Error selecting directory:', error)
  } finally {
    emit('update:isDialogOpen', false) // Notify parent that dialog is closed
  }
}
</script>
