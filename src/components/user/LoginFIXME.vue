<template>
  <div class="align-center mt-8">
    <v-card
      class="mx-auto pa-12 pb-8"
      elevation="8"
      max-width="448"
      rounded="lg"
    >
      <v-card-text class="text-center text-h5 text-high-emphasis text-primary">
        Sign in
      </v-card-text>
      <div class="text-subtitle-1 text-medium-emphasis">Account</div>

      <v-text-field
        density="compact"
        placeholder="Email address"
        prepend-inner-icon="mdi-email-outline"
        variant="outlined"
      />

      <div
        class="text-subtitle-1 text-medium-emphasis d-flex align-center justify-space-between"
      >
        Password
        <v-btn
          class="text-caption text-decoration-none pr-0"
          rel="noopener noreferrer"
          color="primary"
          to="/user/reset-passoword"
          variant="plain"
        >
          Forgot password?
        </v-btn>
      </div>

      <v-text-field
        :append-inner-icon="visible ? 'mdi-eye-off' : 'mdi-eye'"
        :type="visible ? 'text' : 'password'"
        density="compact"
        placeholder="Enter your password"
        prepend-inner-icon="mdi-lock-outline"
        variant="outlined"
        @click:append-inner="visible = !visible"
      />

      <v-btn class="mb-8" color="primary" size="large" variant="tonal" block>
        Log In
      </v-btn>

      <v-card-text class="text-center">
        <v-btn
          class="text-decoration-none"
          rel="noopener noreferrer"
          to="/user/register"
          variant="plain"
        >
          Sign up now
          <v-icon icon="mdi-chevron-right" />
        </v-btn>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
const visible = ref<boolean>()
</script>
