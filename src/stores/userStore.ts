// Utilities
import { defineStore } from "pinia";

export const useUserStore = defineStore("user", () => {
  const username = ref("");
  const isLoggedIn = ref(false);
  const getUsername = computed(() => username.value);
  function login() {
    console.log("attempt login on user:", getUsername);
    if (true) {
      isLoggedIn.value = true;
    } else {
      isLoggedIn.value = false;
    }
  }
  return { username, isLoggedIn, login };
});
