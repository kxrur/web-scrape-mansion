<template>
  <v-card class="mx-auto mt-10" color="surface-light" max-width="400">
    <v-card-text>
      <v-text-field
        v-model="url"
        :loading="loading"
        append-inner-icon="mdi-magnify"
        density="compact"
        label="Search templates"
        variant="solo"
        hide-details
        single-line
        @click:append-inner="onClick"
      ></v-text-field>
      <v-alert v-if="error" type="error" class="mt-3">
        {{ error }}
      </v-alert>
      <v-alert v-if="result" type="success" class="mt-3">
        Successfully scraped: {{ result }}
      </v-alert>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import { commands } from '@/bindings'
import { ref } from 'vue'

let loading = ref(false)
let loaded = ref(false)
let url = ref('')
let error = ref<string | null>(null)
let result = ref<any>(null)

async function onClick() {
  if (!url.value) {
    error.value = 'Please enter a URL.'
    return
  }

  loading.value = true
  error.value = null
  result.value = null

  try {
    const response = await commands.scrapeOneMansion(url.value)
    if (response.status === 'ok') {
      result.value = response.data
    }
  } catch (e) {
    error.value =
      e instanceof Error ? e.message : 'An unexpected error occurred.'
  } finally {
    loading.value = false
    loaded.value = true
  }
}
</script>
