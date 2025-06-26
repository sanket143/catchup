<script setup lang="ts">
const data = []
const hasData = data.length > 0
</script>

<template>
  <div class="container">
    <div class="header">
      <div class="col-1">Topic</div>
      <div class="col-2">Level</div>
      <div class="col-3">No of solved problems</div>
    </div>
    <div class="content" v-if="hasData">
      <div class="col-1">Dynamic Programming</div>
    </div>
    <div class="content" v-else>
      <div class="col-1">No data</div>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px;
}

div.header,
div.content {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: var(--section-gap);
}

div.header > div {
  font-weight: bold;
}

.col-1 {
  grid-column: 1 / span 2;
}

.col-2 {
  grid-column: 3 / 4;
}

.col-3 {
  grid-column: 4 / 5;
}
</style>
