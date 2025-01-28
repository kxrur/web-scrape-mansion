<template>
  <v-app id="inspire">
    <v-main>
      <v-container class="v-fill">
        <v-row>
          <v-col
            v-for="(mansion, index) in mansions"
            :key="index"
            cols="12 mt-0"
            sm="6"
            md="4"
            lg="3"
          >
            <v-card :to="`/mansions/${index}`">
              <v-img :src="mansion.image" height="200" cover />
              <v-card-title>{{ mansion.name }}</v-card-title>
            </v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
const mansions = [
  {
    name: 'Versailles Palace',
    image:
      'https://i.pinimg.com/736x/56/79/19/567919bfe1a3b365a9aa36a09e29dfc7.jpg',
  },
  {
    name: 'Hearst Castle',
    image:
      'https://i.pinimg.com/736x/38/67/9c/38679c10f7c4843331e867c7a3e56401.jpg',
  },
  {
    name: 'Villa Leopolda',
    image:
      'https://i.pinimg.com/736x/58/0a/b5/580ab5044d98e960114626a33352fd50.jpg',
  },
  {
    name: 'Biltmore Estate',
    image:
      'https://i.pinimg.com/736x/9c/1b/66/9c1b66b18530dc0711e0dbbae7ebd721.jpg',
  },
  {
    name: 'The Breakers',
    image:
      'https://i.pinimg.com/736x/cc/36/5e/cc365e5630db30443406e76e426dfaad.jpg',
  },
  {
    name: 'Windsor Castle',
    image:
      'https://i.pinimg.com/736x/33/d7/5e/33d75e11b551c84fd56e3e928d9338c7.jpg',
  },
  {
    name: 'Neuschwanstein Castle',
    image:
      'https://i.pinimg.com/736x/ec/92/9c/ec929c03bf4a706bc0595cdfb120c56b.jpg',
  },
  {
    name: 'Taj Lake Palace',
    image:
      'https://i.pinimg.com/736x/40/da/6f/40da6f2097f1ef8932907922dc447f70.jpg',
  },
]
</script>
