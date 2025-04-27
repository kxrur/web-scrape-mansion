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
          :disabled="isDialogOpen"
        >
          <v-list-item-title>Data</v-list-item-title>
        </v-list-item>

        <v-list-item
          value="Appearance"
          :active="selectedSection === 'Appearance'"
          color="primary"
          base-color=""
          @click="$emit('update:selectedSection', 'Appearance')"
          :disabled="isDialogOpen"
        >
          <v-list-item-title>Appearance</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-col>

    <v-col cols="9">
      <DataSettings
        v-if="selectedSection === 'Data'"
        @update:isDialogOpen="
          (value) => {
            isDialogOpen = value
            $emit('update:isDialogOpen', value)
          }
        "
        @update:dataFolder="(value) => $emit('update:dataFolder', value)"
      />
      <AppearanceSettings v-else-if="selectedSection === 'Appearance'" />
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
defineEmits<{
  (e: 'update:isDialogOpen', value: boolean): void
  (e: 'update:selectedSection', value: string): void
  (e: 'update:dataFolder', value: string): void
}>()

const isDialogOpen = ref<boolean>(false)

defineProps({
  selectedSection: {
    type: String,
    required: true,
  },
})
</script>
