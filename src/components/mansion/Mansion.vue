<template>
  <v-container class="fill-height">
    <v-responsive class="align-center fill-height">
      <v-row no-gutters>
        <v-col sm="4" md="6" lg="8">
          <v-carousel show-arrows="hover">
            <v-carousel-item
              v-for="(image, index) in images"
              :key="index"
              :src="image"
              cover
            />
          </v-carousel>
        </v-col>
        <MnasionDescription></MnasionDescription>
      </v-row>
    </v-responsive>
  </v-container>
</template>

<script setup lang="ts">
const images = [
  'https://cdn.vuetifyjs.com/images/cards/docks.jpg',
  'https://cdn.vuetifyjs.com/images/cards/hotel.jpg',
  'https://cdn.vuetifyjs.com/images/cards/sunshine.jpg',
]
</script>
