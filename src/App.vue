<script setup lang="ts">
import Mansion from '@v/Mansions.vue'


// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'
import { Ref, ref } from 'vue';
const test: Ref<unknown> = ref(null)
invoke('test').then(message => {
  test.value = ref(message)
})

const test_async: Ref<unknown> = ref(null)
// await invoke('test_async').then(message => {
//   test_async.value = ref(message)
// })
</script>

<template>
  <div class="bg-background h-screen">
    <div class="flex flex-col">
      <label class="text-white"> {{ test }}</label>
      <label class="text-white"> {{ test_async }}</label>
    </div>
    <Mansion></Mansion>
  </div>
</template>
