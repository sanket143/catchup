<script setup lang="ts">
const props = defineProps<{
  title: string
  count: string
}>()
</script>

<template>
  <div class="stat">
    <div class="title">{{ props.title }}</div>
    <div class="count">{{ props.count }}</div>
  </div>
</template>

<style scoped>
.stat {
  flex: 1;
  height: 110px;
  border-radius: 6px;
  background: var(--color-background-soft);
  flex-direction: column;
  padding: 10px;
}

.title {
  font-weight: bold;
}

.count {
  font-weight: bolder;
  font-size: 46px;
}
</style>
