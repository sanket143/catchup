<script setup lang="ts">
import axios from 'axios'

function syncProblems() {
  axios({
    method: 'post',
    url: '/api/sync-problems',
  })
}
</script>

<template>
  <main>
    <div>
      <button @click="syncProblems">Sync problem set</button>
    </div>
  </main>
</template>
