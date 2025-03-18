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

    <v-dialog
      activator="parent"
      max-width="800"
      @update:model-value="selectedSection = 'Data'"
    >
      <template v-slot:activator="{ props: activatorProps }">
        <v-btn v-bind="activatorProps" icon="mdi-cog" color="primary"></v-btn>
      </template>

      <template v-slot:default="{ isActive }">
        <v-card rounded="lg">
          <v-card-title class="d-flex justify-space-between align-center">
            <v-toolbar-title class="text-h5 text-primary-darken-1 ps-2">
              Settings
            </v-toolbar-title>
            <v-btn
              icon="mdi-close"
              variant="text"
              color="primary"
              @click="isActive.value = false"
              :disabled="isDialogOpen"
            ></v-btn>
          </v-card-title>

          <v-divider class="mb-4"></v-divider>

          <v-card-text>
            <SettingsBar
              :selected-section="selectedSection"
              v-model="isDialogOpen"
              @update:selectedSection="(value) => (selectedSection = value)"
            ></SettingsBar>
            <v-divider class="mt-2"></v-divider>

            <v-card-actions class="my-2 d-flex justify-end">
              <v-btn
                class="text-none"
                rounded="xl"
                text="Cancel"
                @click="isActive.value = false"
                :disabled="isDialogOpen"
              ></v-btn>

              <v-btn
                class="text-none"
                color="primary"
                rounded="xl"
                text="Apply"
                variant="flat"
                @click="
                  () => {
                    isActive.value = false
                    applySettings()
                  }
                "
                :disabled="isDialogOpen"
              ></v-btn>
            </v-card-actions>
          </v-card-text>
        </v-card>
      </template>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { commands, type Setting } from '@/bindings'

const selectedSection = ref('Data')
const dataSettings = ref()
const isDialogOpen = ref(false) // Track dialog state

const alert = ref<{
  show: boolean
  type: 'success' | 'error' | 'info' | 'warning' | undefined
  message: string
}>({
  show: false,
  type: undefined,
  message: '',
})

function applySettings() {
  if (dataSettings.value) {
    let setting: Setting = {
      id: 0,
      db_path: dataSettings.value.dataFolder,
      profile: 'meh',
      theme: 'light',
    }
    commands
      .saveSetting(setting)
      .then((setting) => {
        if (setting) {
          alert.value = {
            show: true,
            type: 'success',
            message: 'Settings applied successfully!',
          }
        } else {
          alert.value = {
            show: true,
            type: 'error',
            message: 'Failed to apply settings.',
          }
        }
      })
      .catch((error) => {
        console.error('Promise rejected with error: ' + error)
      })
  } else {
    alert.value = {
      show: true,
      type: 'error',
      message: 'Failed to apply settings.',
    }
  }
}
</script>
