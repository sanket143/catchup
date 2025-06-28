<script setup lang="ts">
import axios from 'axios'
import { ref, watch } from 'vue'

let isSyncingProblems = ref(false)
let username = ref('')

watch(username, (val) => {
  document.cookie = 'username=' + val + ';'
})

async function syncProblems() {
  if (isSyncingProblems.value) {
    return
  }

  isSyncingProblems.value = true

  await axios({
    method: 'post',
    url: '/api/sync-problems',
  })

  setTimeout(() => {
    isSyncingProblems.value = false
  }, 1000)
}
</script>

<template>
  <div class="container">
    <div class="settings-section">
      <div class="title">Accounts</div>
      <div class="settings">
        <div class="col-1">Codeforces:</div>
        <div class="col-2">
          <input v-model="username" placeholder="Your username (e.g. sankxt143)" />
        </div>
      </div>
    </div>
    <div class="settings-section">
      <div class="title">Codeforces</div>
      <div>
        <a :class="{ disabled: isSyncingProblems }" @click="syncProblems">Sync problems</a>
      </div>
      <div>
        <a
          >Sync user solved submissions (for
          {{ username ? username : '<codeforces-username>' }})</a
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
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: var(--section-gap);
}

.col-1 {
  grid-column: 1;
}

.col-2 {
  grid-column: 2 / span 3;
}

input {
  border: none;
  width: 100%;
}

.settings-section {
  margin-bottom: 10px;
}
</style>
