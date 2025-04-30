<template>
  <Mansion :id="id" />
</template>

<script lang="ts" setup>
const route = useRoute('/mansions/[id]')
const id = ref(route.params.id)
</script>
