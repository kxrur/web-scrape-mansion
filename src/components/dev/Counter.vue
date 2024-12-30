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
  <v-btn
    variant="flat"
    rounded="LG"
    text="hello world"
    color="primary"
    @click="helloWorld"
  />
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
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

import { commands } from '../../bindings' // This should point to the file we export from Rust

async function helloWorld() {
  console.log(
    'your house: ',
    await commands.helloWorld({
      name: 'myname',
      dream_floors: 3,
      dream_rooms: 5,
    })
  )
}
</script>
