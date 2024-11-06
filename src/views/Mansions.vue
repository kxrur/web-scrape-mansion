<script setup lang="ts">
import Pictures from '@c/mansion/Pictures.vue'
import Info from '@c/mansion/Info.vue'

const images = [
  "src/assets/images/Blairlogie_Castle/Aerial(3).jpg"
]

</script>

<template>
  <div class="inline-flex  space-x-20">
    <Pictures :pictures="images"></Pictures>
    <Info></Info>
  </div>
</template>
