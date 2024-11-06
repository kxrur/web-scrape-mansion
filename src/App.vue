<script setup lang="ts">
import Mansion from '@v/Mansions.vue'
import Pictures from './components/mansion/Pictures.vue';

const images = [
  "src/assets/images/Blairlogie_Castle/Aerial(3).jpg"
]
</script>

<template>
  <div class="bg-background h-screen">
    <Pictures :pictures="images"></Pictures>
  </div>
</template>
