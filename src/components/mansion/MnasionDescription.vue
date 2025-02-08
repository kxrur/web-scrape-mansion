<template>
  <v-col cols="4">
    <v-sheet class="pa-4 ma-2">
      <v-list v-if="mansion">
        <v-list-item>
          <v-list-item-title class="text-h6">Mansion Details</v-list-item-title>
        </v-list-item>
        <v-divider></v-divider>
        <v-list-item>
          <v-list-item-title>Address</v-list-item-title>
          <v-list-item-subtitle>{{ mansion.address }}</v-list-item-subtitle>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Price</v-list-item-title>
          <v-list-item-subtitle>
            ${{ mansion.price?.toLocaleString() }}
          </v-list-item-subtitle>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Size</v-list-item-title>
          <v-list-item-subtitle>
            {{ mansion.size?.toLocaleString() }} sq. ft.
          </v-list-item-subtitle>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Rooms</v-list-item-title>
          <v-list-item-subtitle>
            {{ mansion.bedrooms }} Bedrooms, {{ mansion.bathrooms }} Bathrooms
          </v-list-item-subtitle>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>House Type</v-list-item-title>
          <v-list-item-subtitle>{{ mansion.house_type }}</v-list-item-subtitle>
        </v-list-item>
      </v-list>
      <v-alert v-else type="info">Loading mansion details...</v-alert>
    </v-sheet>
  </v-col>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { commands } from '@/bindings'

const mansion = ref()

onMounted(async () => {
  try {
    const mansions = await commands.loadMansions()
    mansion.value = mansions.length > 0 ? mansions[0] : null
  } catch (error) {
    console.error('Error loading mansions:', error)
  }
})
</script>
