<template>
  counter:
  <VCounter :active="true" :value="count" />
  <v-btn
    variant="flat"
    rounded="LG"
    text="Increment"
    color="primary"
    @click="increment"
  />
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { commands } from '@/bindings'

const count = ref(0)

function increment() {
  console.log('increment')
  // Invocation from JavaScript
  invoke<number>('increment_counter', {})
    .then((res) => {
      console.log(`counter: ${res}`)
      count.value = res
    })
    .catch((e) => console.error(e))
}
</script>
