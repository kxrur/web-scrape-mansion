<template>
  <div>
    <v-snackbar
      v-model="alert.show"
      :color="alert.type"
      :timeout="3000"
      location="top right"
    >
      {{ alert.message }}

      <template v-slot:actions>
        <v-btn
          color="white"
          variant="text"
          @click="alert.show = false"
          icon="mdi-close"
        ></v-btn>
      </template>
    </v-snackbar>
    <v-card class="mx-auto mt-10" color="surface-light" max-width="400">
      <v-card-text>
        <v-text-field
          v-model="url"
          :loading="loading"
          append-inner-icon="mdi-magnify"
          density="compact"
          label="Search templates"
          variant="solo"
          hide-details
          single-line
          @click:append-inner="search"
        ></v-text-field>
      </v-card-text>
    </v-card>
    <div v-if="mansion && id">
      <v-container class="fill-height">
        <v-responsive class="align-center fill-height">
          <!-- <v-row no-gutters> -->
          <!--   <MansionImages :id="id.toString()"></MansionImages> -->
          <!--   <MnasionDescription :id="id.toString()"></MnasionDescription> -->
          <!-- </v-row> -->
          <Mansion :id="id.toString()"></Mansion>
        </v-responsive>
      </v-container>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { commands, type Mansion } from '@/bindings'
import { ref } from 'vue'

const loading = ref(false)
const loaded = ref(false)
const url = ref('')
const mansion = ref<Mansion>()
const id = ref<number>()

const alert = ref<{
  show: boolean
  type: 'success' | 'error' | 'info' | 'warning' | undefined
  message: string
}>({
  show: false,
  type: undefined,
  message: '',
})

async function search() {
  if (!url.value) {
    alert.value.message = 'Please enter a URL!'
    alert.value.type = 'warning'
    alert.value.show = true
    return
  }

  mansion.value = undefined
  id.value = undefined

  loading.value = true

  try {
    const new_mansion = await commands.addMansion(url.value)
    if (new_mansion.status === 'ok') {
      let idee = await commands.getMansionStateIdByDatabaseId(
        new_mansion.data.mansion
      )
      if (idee.status === 'ok') {
        mansion.value = new_mansion.data
        id.value = idee.data

        alert.value.message = 'Success scraping mansion!'
        alert.value.type = 'success'
        alert.value.show = true
      } else {
        throw Error
      }
    }
  } catch (e) {
    alert.value.message =
      e instanceof Error ? e.message : 'An unexpected error occurred!'
    alert.value.type = 'error'
    alert.value.show = true

    mansion.value = undefined
    id.value = undefined
  } finally {
    loading.value = false
    loaded.value = true
  }
}
</script>
