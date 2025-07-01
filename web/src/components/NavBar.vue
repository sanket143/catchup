<script setup lang="ts">
import { useUserStore } from '@/stores/user'
const userStore = useUserStore()
</script>

<template>
  <nav>
    <router-link to="/" active-class="active-link" class="nav-item">Dashboard</router-link>
    <router-link to="/local-contest" active-class="active-link" class="nav-item"
      >Local contest</router-link
    >
    <router-link to="/settings" active-class="active-link" class="nav-item">Settings</router-link>
    <div class="spacer"></div>
    <router-link to="/login" active-class="active-link" class="nav-item">{{
      userStore.isLoggedIn ? userStore.username : 'Login'
    }}</router-link>
  </nav>
</template>

<style scoped>
nav {
  border-radius: 6px;
  width: 100%;
  display: flex;
  align-items: center;
  padding: 20px;
  height: 50px;
  background: var(--color-background-soft);
  gap: var(--section-gap);
}

nav a {
  color: var(--color-text);
  text-decoration: none;
}

nav a.active-link {
  color: var(--color-text);
  font-weight: bold;
}
</style>
