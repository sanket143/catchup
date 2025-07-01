<script setup lang="ts">
import axios from 'axios'
import { ref, watch } from 'vue'
import { useAppStore } from '@/stores/app'
import { useUserStore } from '@/stores/user'

const appStore = useAppStore()
const userStore = useUserStore()

async function syncProblems() {
  if (appStore.syncingProblemsInProgress) {
    return
  }

  appStore.updateSyncingProblemStatus(true)

  setTimeout(() => {
    appStore.updateSyncingProblemStatus(false)
  }, 10000)
}
</script>

<template>
  <div class="container">
    <div class="settings-section">
      <div class="title">Accounts</div>
      <div class="settings flex">
        <div class="col-1">Codeforces username:</div>
        <div class="col-2">
          <router-link to="/login">{{ userStore.username }}</router-link>
        </div>
      </div>
    </div>
    <div class="settings-section">
      <div class="title">Codeforces</div>
      <div>
        <a :class="{ disabled: appStore.syncingProblemsInProgress }" @click="syncProblems"
          >Sync problems</a
        >
      </div>
      <div>
        <a>
          Sync user solved submissions (for
          {{ userStore.username ? userStore.username : '\<codeforces-username>' }})</a
        >
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  padding: 10px;
  background: var(--color-background-soft);
}

.title {
  font-weight: bold;
}

.settings {
  max-width: 550px;
}

input {
  border: none;
  width: 100%;
}

.settings-section {
  margin-bottom: 10px;
}
</style>
