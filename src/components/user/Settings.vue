<template>
  <div>
    <v-dialog activator="parent" max-width="800">
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
            ></v-btn>
          </v-card-title>

          <v-divider class="mb-4"></v-divider>

          <v-card-text>
            <v-row>
              <v-col cols="3">
                <v-list color="surface">
                  <v-list-item
                    value="Data"
                    :active="selectedSection === 'Data'"
                    color="primary"
                    base-color=""
                    @click="selectedSection = 'Data'"
                  >
                    <v-list-item-title>Data</v-list-item-title>
                  </v-list-item>

                  <v-list-item
                    value="Appearance"
                    :active="selectedSection === 'Appearance'"
                    color="primary"
                    base-color=""
                    @click="selectedSection = 'Appearance'"
                  >
                    <v-list-item-title>Appearance</v-list-item-title>
                  </v-list-item>
                </v-list>
              </v-col>

              <!-- Content Area -->
              <v-col cols="9">
                <DataSettings v-if="selectedSection === 'Data'" />
                <AppearanceSettings
                  v-else-if="selectedSection === 'Appearance'"
                />
              </v-col>
            </v-row>
            <v-divider class="mt-2"></v-divider>

            <v-card-actions class="my-2 d-flex justify-end">
              <v-btn
                class="text-none"
                rounded="xl"
                text="Cancel"
                @click="isActive.value = false"
              ></v-btn>

              <v-btn
                class="text-none"
                color="primary"
                rounded="xl"
                text="Apply"
                variant="flat"
                @click="isActive.value = false"
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

const selectedSection = ref('Data')
</script>
