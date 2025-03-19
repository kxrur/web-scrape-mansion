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
        @update:dataFolder="
          (value) => () => {
            console.log('Data folder updated:', value)
            $emit('update:dataFolder', value)
          }
        "
      />
      <AppearanceSettings v-else-if="selectedSection === 'Appearance'" />
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'update:selectedSection', value: string): void
  (e: 'update:dataFolder', value: string | null): void
}>()

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
</script>
