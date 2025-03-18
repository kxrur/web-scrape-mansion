<template>
  <v-row>
    <v-col cols="3">
      <v-list color="surface">
        <v-list-item
          value="Data"
          :active="selectedSection === 'Data'"
          color="primary"
          base-color=""
          @click="$emit('update:selectedSection', 'Data')"
          :disabled="modelValue"
        >
          <v-list-item-title>Data</v-list-item-title>
        </v-list-item>

        <v-list-item
          value="Appearance"
          :active="selectedSection === 'Appearance'"
          color="primary"
          base-color=""
          @click="$emit('update:selectedSection', 'Appearance')"
          :disabled="modelValue"
        >
          <v-list-item-title>Appearance</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-col>

    <v-col cols="9">
      <DataSettings
        v-if="selectedSection === 'Data'"
        ref="dataSettings"
        :isDialogOpen="modelValue"
        @update:isDialogOpen="(value) => $emit('update:modelValue', value)"
      />
      <AppearanceSettings v-else-if="selectedSection === 'Appearance'" />
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import DataSettings from './DataSettings.vue'
import AppearanceSettings from './AppearanceSettings.vue'

defineProps({
  selectedSection: {
    type: String,
    required: true,
  },
  modelValue: {
    type: Boolean,
    required: true,
  },
})
defineEmits(['update:modelValue', 'update:selectedSection'])
</script>
