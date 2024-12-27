<template>
  <v-app id="inspire">
    <v-main>
      <v-container class="v-fill">
        <v-row>
          <v-col v-for="n in 8" :key="n" cols="3 mt-0">
            <v-card
              height="200"
              image="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRrqkIH_ebGgP-M-3KswfjypW-Uw5MdVsxN8w&s"
            />
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>
