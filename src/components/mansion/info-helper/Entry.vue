<script setup lang="ts">

const props = defineProps<{
  datas: string[],
  icons?: string[]
}>()

validatePropsLength(props.datas, props.icons)

function validatePropsLength(datas: string[], icons?: string[]) {
  if (icons && datas.length !== icons.length) {
    throw new Error("The 'datas' and 'icons' arrays must be of the same length.");
  }
}
</script>

<template>
  <div class="rounded-3xl bg-highlight text-foreground px-3 py-2">
    <span v-for="(data, index) in datas">
      <img v-if="icons && icons[index]" :src="icons[index]" alt="data-icon">
      {{ data }}
    </span>
  </div>
</template>
