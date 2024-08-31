<script setup lang="ts">
import Entry from './info-helper/Entry.vue';
const addresses: string[] = ["address1", "address2"];

</script>

<template>
  <div class="bg-secondary rounded-[36px] p-20 text-xl">
    <h1>Should work</h1>
    <Entry :datas="addresses" class="inline-grid"></Entry>
  </div>
</template>
