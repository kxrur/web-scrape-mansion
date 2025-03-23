<template>
  <v-app id="inspire">
    <v-main>
      <v-container class="v-fill">
        <v-row>
          <template v-if="!mansions">
            <v-col v-for="n in 4" :key="n" cols="12 mt-0" sm="6" md="4" lg="3">
              <v-skeleton-loader
                type="image, heading"
                height="200"
              ></v-skeleton-loader>
            </v-col>
          </template>

          <template v-else>
            <v-col
              v-for="(mansion, index) in mansions"
              :key="index"
              cols="12 mt-0"
              sm="6"
              md="4"
              lg="3"
            >
              <v-card :to="`/mansions/${index}`">
                <v-img :src="first_pictures[index]" height="200" cover />
                <v-card-title>
                  {{ mansion.address }}
                </v-card-title>
              </v-card>
            </v-col>
          </template>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { commands, type Mansionee } from '@/bindings'
import { convertFileSrc } from '@tauri-apps/api/core'

// Define the structure of a Picture object
interface Picture {
  name: string
  path: string
}

const mansions = ref<Mansionee[]>()
const first_pictures = ref<string[]>([])

onMounted(async () => {
  loadMansions()
})

async function loadMansions() {
  try {
    console.log('start loading mansions')
    const result = await commands.getStoreMansions()
    console.log('finish loading mansions')

    if (result.status === 'ok') {
      mansions.value = result.data

      first_pictures.value = []

      mansions.value.forEach((mansion, index) => {
        if (mansion.pictures) {
          const pictures = mansion.pictures as unknown as Picture[]
          if (pictures.length > 0) {
            const firstPicturePath = pictures[0].path
            console.log('success mansion pic', firstPicturePath)
            const filepath = firstPicturePath
            console.log(filepath)
            first_pictures.value[index] = convertFileSrc(filepath)
          } else {
            console.log('failed mansion pic')
            first_pictures.value[index] = '' // or a placeholder image URL
          }
        } else {
          // If pictures is null or undefined, set a placeholder or leave it empty
          first_pictures.value[index] = '' // or a placeholder image URL
        }
      })
    } else {
      console.error('Error loading mansions:', result.error)
    }
    console.log('mansions:', mansions.value)
    console.log('first_pictures:', first_pictures.value)
  } catch (error) {
    console.error('Unexpected error:', error)
  }
}
</script>
