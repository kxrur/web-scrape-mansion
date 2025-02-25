<template>
  <v-container class="fill-height">
    <v-responsive class="align-center fill-height">
      <v-row no-gutters>
        <MansionImages :id="id"></MansionImages>
        <MnasionDescription :id="id"></MnasionDescription>
      </v-row>
    </v-responsive>
  </v-container>
</template>

<script setup lang="ts">
const route = useRoute('/mansions/[id]')
const id = ref(route.params.id)
</script>
