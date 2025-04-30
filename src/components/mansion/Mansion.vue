<template>
  <v-container class="fill-height">
    <v-responsive class="align-center fill-height">
      <v-row>
        <v-col cols="12" md="6">
          <MansionImages :id="id" />
        </v-col>
        <v-col cols="12" md="6">
          <MnasionDescription :id="id" />
        </v-col>
      </v-row>
    </v-responsive>
  </v-container>
</template>

<script setup lang="ts">
const props = defineProps<{
  id: string
}>()
</script>
