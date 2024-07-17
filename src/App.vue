<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import Home from "@/views/Home.vue"

invoke('greet', { name: 'World' })
  .then((response) => console.log(response))
</script>

<template>
  <div>
    <Home></Home>
  </div>
</template>
