<template>
  <v-app>
    <router-view />
  </v-app>
</template>

<script lang="ts" setup>
import { initTheme } from '@/plugins/CustomThemes'
import { useTheme } from 'vuetify'

onMounted(() => {
  const theme = useTheme()
  initTheme(theme)
})
</script>
