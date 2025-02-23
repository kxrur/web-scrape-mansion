<template>
  <v-col sm="4" md="6" lg="8">
    <v-skeleton-loader
      v-if="!images.length"
      type="image"
      class="carousel-skeleton"
    ></v-skeleton-loader>

    <v-carousel v-else show-arrows="hover">
      <v-carousel-item
        v-for="(image, index) in images"
        :key="index"
        :src="image"
        cover
      />
    </v-carousel>
  </v-col>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { homeDir, join } from '@tauri-apps/api/path'
import { convertFileSrc } from '@tauri-apps/api/core'

const images = ref<string[]>([])

onMounted(async () => {
  // Simulate a delay for demonstration purposes
  await new Promise((resolve) => setTimeout(resolve, 2000))

  const homeDirPath = await homeDir()
  const filePath = await join(
    homeDirPath,
    'Desktop/images/Blairlogie_Castle/Aerial(10).jpg'
  )
  const assetUrl = convertFileSrc(filePath)
  console.log(assetUrl)

  images.value = [
    'https://cdn.vuetifyjs.com/images/cards/docks.jpg',
    assetUrl,
    'https://cdn.vuetifyjs.com/images/cards/hotel.jpg',
    'https://cdn.vuetifyjs.com/images/cards/sunshine.jpg',
  ]
})
</script>

<style scoped>
.carousel-skeleton {
  width: 100%;
  height: 300px; /* Adjust height as needed */
}
</style>
