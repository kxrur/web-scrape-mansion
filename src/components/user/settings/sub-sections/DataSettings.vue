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
import { type Setting } from '@/bindings'

const props = defineProps<{
  setting: Setting
}>()

const emit = defineEmits<{
  (e: 'update:isDialogOpen', value: boolean): void
  (e: 'update:dataFolder', value: string): void
}>()

const dataFolder = ref<string | null>(null)

const isDialogOpen = ref<boolean>(false)

onMounted(() => {
  dataFolder.value = props.setting.db_path
})

const openDirectoryPicker = async () => {
  try {
    isDialogOpen.value = true
    emit('update:isDialogOpen', isDialogOpen.value)

    // Open a directory picker
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Data Folder',
    })

    if (selected) {
      dataFolder.value = selected as string
      emit('update:dataFolder', dataFolder.value)
    }
  } catch (error) {
    console.error('Error selecting directory:', error)
  } finally {
    isDialogOpen.value = false
    emit('update:isDialogOpen', isDialogOpen.value)
  }
}
</script>
