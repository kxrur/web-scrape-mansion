<script setup lang="ts">
import { ref, defineProps, computed } from 'vue';

interface Props {
  pictures: string[];
}

const props = defineProps<Props>();
const currentIndex = ref(0);

const currentImage = computed(() => props.pictures[currentIndex.value]);

const nextImage = () => {
  currentIndex.value = (currentIndex.value + 1) % props.pictures.length;
};

const prevImage = () => {
  currentIndex.value = (currentIndex.value - 1 + props.pictures.length) % props.pictures.length;
};
</script>

<template>
  <div class="relative w-full max-w-full sm:max-w-md md:max-w-lg lg:max-w-xl xl:max-w-2xl">
    <div class="relative w-full h-40 sm:h-64 md:h-80 lg:h-96 xl:h-[40vh] rounded-lg overflow-hidden shadow-lg">
      <img :src="currentImage" alt="Gallery Image" class="w-full h-full object-contain" />

      <button @click="prevImage"
        class="absolute left-4 top-1/2 transform -translate-y-1/2 bg-secondary text-foreground rounded-full w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 flex items-center justify-center opacity-75 hover:opacity-100 transition"
        aria-label="Previous Image">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-foreground" fill="none" viewBox="0 0 24 24"
          stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
        </svg>
      </button>

      <!-- Next Button with SVG Icon -->
      <button @click="nextImage"
        class="absolute right-4 top-1/2 transform -translate-y-1/2 bg-secondary text-foreground rounded-full w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 flex items-center justify-center opacity-75 hover:opacity-100 transition"
        aria-label="Next Image">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-foreground" fill="none" viewBox="0 0 24 24"
          stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
        </svg>
      </button>
    </div>

    <!-- Picture Count -->
    <div class="text-center mt-2 text-foreground">
      {{ currentIndex + 1 }}/{{ props.pictures.length }}
    </div>
  </div>
</template>
