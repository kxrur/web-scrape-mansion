<script setup lang="ts">
import Entry from './info-helper/Entry.vue';
import bedIcon from '@a/bed.svg';
import chairIcon from '@a/chair.svg';
import bathIcon from '@a/bath.svg';

const addresses: string[] = ["address1", "address2"];
const rooms_nmb: string[] = ["1", "2", "3"];
const rooms_icons: string[] = [bedIcon, chairIcon, bathIcon];
const price_size: string[] = ["price: 3000", "size: 323234 sq m"];
const type: string[] = ["Mansion"];
</script>

<template>
  <div class="bg-secondary rounded-[36px] p-20 text-xl inline-block space-y-3">
    <Entry :datas="addresses"></Entry>
    <Entry :datas="price_size" class="flex space-x-12"></Entry>
    <Entry :datas="rooms_nmb" :icons="rooms_icons" class="flex space-x-8"></Entry>
    <Entry :datas="type"></Entry>
  </div>
</template>
