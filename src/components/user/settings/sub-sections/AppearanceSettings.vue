<template>
  <div>
    <div class="text-h6 mb-4">Appearance Settings</div>

    <v-select
      label="Theme"
      :items="['Light', 'Dark', 'System']"
      variant="outlined"
      class="mb-4"
      color="primary"
    ></v-select>

    <v-switch
      label="Enable Animations"
      color="primary"
      hide-details
      class="mb-4"
    ></v-switch>
  </div>
</template>

<script setup lang="ts"></script>
