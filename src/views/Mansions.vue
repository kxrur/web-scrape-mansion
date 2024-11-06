<script setup lang="ts">
import Pictures from '@c/mansion/Pictures.vue'
import Info from '@c/mansion/Info.vue'

const images = [
  "src/assets/images/Blairlogie_Castle/Aerial(3).jpg",
  "src/assets/images/Blairlogie_Castle/Aerial(1).jpg",
  "src/assets/images/Blairlogie_Castle/Aerial(2).jpg"
]

</script>

<template>
  <div class="flex flex-col items-center space-y-16 pt-20">
    <Pictures :pictures="images"></Pictures>
    <Info></Info>
  </div>
</template>
