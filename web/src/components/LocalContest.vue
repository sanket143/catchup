<script setup lang="ts">
defineProps<{
  msg: string
}>()
</script>

<template>
  <nav>
    <router-link to="/" active-class="active-link" class="nav-item">Dashboard</router-link>
    <router-link to="/local-contest" active-class="active-link" class="nav-item"
      >Local contest</router-link
    >
    <router-link to="/settings" active-class="active-link" class="nav-item">Settings</router-link>
  </nav>
</template>

<style scoped>
nav {
  display: flex;
  align-items: center;
  padding: 20px;
  height: 50px;
  background: var(--color-background-soft);
  gap: 24px;
}

nav a {
  color: var(--color-text);
  text-decoration: none;
}

.active-link {
  color: var(--color-text);
  font-weight: bold;
}
</style>
