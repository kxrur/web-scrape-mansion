<script setup lang="ts">
defineProps<{
  pictures: string[],
}>()
</script>

<template>
  <img :src="pictures[0]" alt="some image">
</template>
