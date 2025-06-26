<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <header>
    <NavBar />
  </header>

  <main>
    <RouterView />
  </main>
</template>

<style scoped>
main {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--section-gap);
}
</style>
