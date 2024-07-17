<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { invoke } from '@tauri-apps/api'

invoke('greet', { name: 'World' })
  // `invoke` returns a Promise
  .then((response) => console.log(response))
</script>

<template>
  <div>
    <Button>Click me</Button>
  </div>
</template>
