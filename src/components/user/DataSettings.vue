<template>
  <div>
    <div class="text-h6 mb-4">Data Settings</div>

    <div class="text-h7 mb-2">Storage Folder</div>
    <v-file-input
      clearable
      label="Data Folder"
      variant="solo-inverted"
      webkitdirectory
      v-model="dataFolder"
    ></v-file-input>

    <div class="text-h7 mb-2">Sync</div>
    <v-switch label="Enable Data Sync" color="primary" hide-details></v-switch>
  </div>
</template>

<script setup lang="ts">
const dataFolder = ref(null)
defineExpose({ dataFolder })
</script>
