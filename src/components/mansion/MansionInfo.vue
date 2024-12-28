<template>
  <v-container class="bg-surface-variant fill-height">
    <v-responsive
      class="align-centerfill-height"
      max-width="2000"
      min-width="1600"
    >
      <v-row no-gutters>
        <v-col>
          <v-sheet class="pa-2 ma-2"> image </v-sheet>
        </v-col>

        <v-col cols="4">
          <v-sheet class="pa-2 ma-2"> desc </v-sheet>
        </v-col>
      </v-row>
    </v-responsive>
  </v-container>
</template>

<script setup lang="ts"></script>
