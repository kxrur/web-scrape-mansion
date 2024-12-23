<template>
  <v-app id="inspire">
    <v-navigation-drawer v-model="drawer" expand-on-hover rail>
      <v-list>
        <v-list-item title="Mansions">
          <template #prepend>
            <v-icon color="primary">mdi-castle</v-icon>
          </template>
        </v-list-item>
      </v-list>

      <v-divider />

      <v-list density="compact" nav>
        <v-list-item
          prepend-icon="mdi-folder"
          title="My Files"
          value="myfiles"
          to="/mansion"
        />
        <v-list-item
          prepend-icon="mdi-account-multiple"
          title="Shared with me"
          value="shared"
          to="/about"
        />
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <!--  -->
      <slot />
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import { ref } from "vue";

const drawer = ref(null);
</script>
