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

        <v-col cols="4">
          <v-sheet class="pa-4 ma-2">
            <v-list>
              <v-list-item>
                <v-list-item-title class="text-h6">
                  Mansion Details
                </v-list-item-title>
              </v-list-item>
              <v-divider></v-divider>
              <v-list-item>
                <v-list-item-title>Address</v-list-item-title>
                <v-list-item-subtitle>
                  123 Luxury Lane, Beverly Hills, CA 90210
                </v-list-item-subtitle>
              </v-list-item>
              <v-list-item>
                <v-list-item-title>Price</v-list-item-title>
                <v-list-item-subtitle>$5,000,000</v-list-item-subtitle>
              </v-list-item>
              <v-list-item>
                <v-list-item-title>Size</v-list-item-title>
                <v-list-item-subtitle>10,000 sq. ft.</v-list-item-subtitle>
              </v-list-item>
              <v-list-item>
                <v-list-item-title>Rooms</v-list-item-title>
                <v-list-item-subtitle>
                  6 Bedrooms, 8 Bathrooms
                </v-list-item-subtitle>
              </v-list-item>
              <v-list-item>
                <v-list-item-title>House Type</v-list-item-title>
                <v-list-item-subtitle>Modern Mansion</v-list-item-subtitle>
              </v-list-item>
            </v-list>
          </v-sheet>
        </v-col>
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
