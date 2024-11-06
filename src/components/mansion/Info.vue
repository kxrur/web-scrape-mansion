<script setup lang="ts">
import { ref } from 'vue';
import Entry from './info-helper/Entry.vue';
import bedIcon from '@a/bed.svg';
import chairIcon from '@a/chair.svg';
import bathIcon from '@a/bath.svg';

// Define reactive data arrays
const addresses = ref(["address1", "address2"]);
const rooms_nmb = ref(["1", "2", "3"]);
const rooms_icons = ref([bedIcon, chairIcon, bathIcon]);
const price_size = ref(["price: 3000", "size: 323234 sq m"]);
const type = ref(["Mansion"]);
</script>

<template>
  <div class="bg-secondary rounded-[36px] p-4 text-xl inline-block space-y-4">
    <!-- Render each entry dynamically -->
    <Entry :datas="addresses" />
    <Entry :datas="price_size" class="flex space-x-12" />
    <Entry :datas="rooms_nmb" :icons="rooms_icons" class="flex space-x-8" />
    <Entry :datas="type" />
  </div>
</template>
