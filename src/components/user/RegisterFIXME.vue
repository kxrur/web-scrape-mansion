<template>
  <div class="align-center mt-8">
    <v-card
      class="mx-auto pa-12 pb-8"
      elevation="8"
      max-width="448"
      rounded="lg"
    >
      <v-card-text class="text-center text-h5 text-high-emphasis text-primary">
        Sign up
      </v-card-text>
      <div class="text-subtitle-1 text-medium-emphasis">Username</div>

      <v-text-field
        density="compact"
        placeholder="Username"
        prepend-inner-icon="mdi-account"
        variant="outlined"
      />
      <div class="text-subtitle-1 text-medium-emphasis">Email</div>

      <v-text-field
        density="compact"
        placeholder="Email address"
        prepend-inner-icon="mdi-email-outline"
        variant="outlined"
      />

      <div class="text-subtitle-1 text-medium-emphasis">Password</div>

      <v-text-field
        :append-inner-icon="visible ? 'mdi-eye-off' : 'mdi-eye'"
        :type="visible ? 'text' : 'password'"
        density="compact"
        placeholder="Enter password"
        prepend-inner-icon="mdi-lock-outline"
        variant="outlined"
        @click:append-inner="visible = !visible"
      />
      <div class="text-subtitle-1 text-medium-emphasis">Confirm Password</div>
      <v-text-field
        :append-inner-icon="visible2 ? 'mdi-eye-off' : 'mdi-eye'"
        :type="visible2 ? 'text' : 'password'"
        density="compact"
        placeholder="Confirm password"
        prepend-inner-icon="mdi-lock-outline"
        variant="outlined"
        @click:append-inner="visible2 = !visible2"
      />

      <v-btn class="mb-8" color="primary" size="large" variant="tonal" block>
        Register
      </v-btn>

      <v-card-text class="text-center">
        <v-btn
          class="text-decoration-none"
          rel="noopener noreferrer"
          to="/user/login"
          variant="plain"
        >
          Sign in now
          <v-icon icon="mdi-chevron-right" />
        </v-btn>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
const visible = ref(null)
const visible2 = ref(null)
</script>
