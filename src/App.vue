<template>
  <div>
    <Navbar :items="items"></Navbar>
    <Home></Home>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import Home from "@/views/Home.vue"
import Navbar from "@/components/Navbar.vue"
import { MenuItem } from "@/types/types";




invoke('greet', { name: 'World' })
  .then((response) => console.log(response))

const items: MenuItem[] = [
  { link: "/", name: "Services" },
  { link: "/services", name: "Services" },
]
</script>
