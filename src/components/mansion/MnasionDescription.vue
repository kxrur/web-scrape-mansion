<template>
  <v-sheet class="pa-4 ma-2" color="transparent">
    <template v-if="!mansion">
      <v-skeleton-loader
        type="list-item-avatar-two-line, list-item-two-line, list-item-two-line, list-item-two-line, list-item-two-line"
      ></v-skeleton-loader>
    </template>

    <v-list v-else rounded="xl" bg-color="primary-darken-1">
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
  </v-sheet>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { commands, type Mansionee } from '@/bindings'

const props = defineProps<{
  id: String
}>()

const mansion = ref<Mansionee>()

onMounted(async () => {
  //await new Promise((resolve) => setTimeout(resolve, 2000)) // Simulate loading delay

  try {
    const result = await commands.getMansionById(Number(props.id))

    if (result.status === 'ok') {
      mansion.value = result.data.mansion
    } else {
      console.error(
        'Error loading mansion:',
        props.id,
        'with error: ',
        result.error
      )
    }
  } catch (error) {
    console.error('Unexpected error:', error)
  }
})
</script>
