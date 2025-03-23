<template>
  <v-col sm="4" md="6" lg="8">
    <v-skeleton-loader
      v-if="!images.length"
      type="image"
      class="carousel-skeleton"
    ></v-skeleton-loader>

    <v-carousel v-else show-arrows="hover" :hide-delimiters="true">
      <v-carousel-item
        rounded="xl"
        v-for="(image, index) in images"
        :key="index"
        :src="image"
        cover
      ></v-carousel-item>
    </v-carousel>
  </v-col>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { commands } from '@/bindings'

const props = defineProps<{
  id: string
}>()

interface Picture {
  name: string
  path: string
}

const images = ref<string[]>([])

onMounted(async () => {
  //await new Promise((resolve) => setTimeout(resolve, 2000)) // Simulate loading delay

  const result = await commands.getMansionById(Number(props.id))
  if (result.status === 'ok') {
    const mansion = result.data
    if (mansion.pictures) {
      const pictures = mansion.pictures as unknown as Picture[]
      if (pictures.length > 0) {
        const imageUrls = pictures.map((picture) => {
          const filepath = picture.path
          console.log(filepath)
          return convertFileSrc(filepath)
        })
        images.value = imageUrls.filter((url) => url !== '')
      }
    }
  }
})
</script>

<style scoped>
.carousel-skeleton {
  width: 100%;
  height: 300px; /* Adjust height as needed */
}
</style>
