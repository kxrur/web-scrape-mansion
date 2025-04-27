<template>
  <v-app id="inspire">
    <v-navigation-drawer v-model="drawer">
      <v-list>
        <v-list-item title="Mansions" to="/" :value="selectedLink">
          <template #prepend>
            <v-icon color="primary">mdi-castle</v-icon>
          </template>
        </v-list-item>
      </v-list>

      <v-divider />

      <v-list density="compact" nav slim>
        <v-list-item
          v-for="(link, index) in links"
          :key="index"
          :value="selectedLink"
          :prepend-icon="link.icon"
          :title="link.title"
          :to="link.to"
          color="primary"
          base-color=""
        />
      </v-list>
    </v-navigation-drawer>

    <v-app-bar>
      <v-app-bar-nav-icon color="primary" @click="drawer = !drawer" />

      <!--TODO: Make page title reflect the page's contents -->
      <v-app-bar-title>Web Scrape Mansions</v-app-bar-title>

      <Settings></Settings>
      <v-btn icon="mdi-login" color="primary" to="/user/login" />
    </v-app-bar>

    <v-main>
      <!--  -->
      <slot />
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const drawer = ref<boolean>()

const selectedLink = ref(null)

const links = [
  {
    icon: 'mdi-dots-grid',
    title: 'Browse Mansions',
    value: 'myfiles',
    to: '/mansions',
  },
  {
    icon: 'mdi-plus-box-multiple-outline',
    title: 'Add Mansion',
    value: 'shared',
    to: '/add-mansion',
  },
  {
    icon: 'mdi-information-box-outline',
    title: 'About',
    value: 'shared',
    to: '/about',
  },
]
</script>
