<script setup lang="ts">
import axios from 'axios'

axios({
  method: 'post',
  url: '/api/contest/problems',
  data: {
    contestId: 1,
  },
})
</script>

<template>
  <main></main>
</template>
