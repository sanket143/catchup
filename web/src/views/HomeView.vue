<script setup lang="ts">
import StatCard from '../components/StatCard.vue'
import Dashboard from '../components/Dashboard.vue'
import QuickLink from '../components/QuickLinks.vue'
</script>

<template>
  <div class="stat-cards">
    <StatCard title="Total contests" count="134" />
    <StatCard title="Overall level" count="4" />
    <StatCard title="Total problems solved" count="122" />
    <StatCard title="Codeforces profile" count="sankxt143" />
    <div class="dashboard">
      <Dashboard />
    </div>
    <div class="quick-links">
      <QuickLink />
    </div>
  </div>
</template>

<style scoped>
.stat-cards {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: var(--section-gap);
}

.dashboard {
  grid-column: 1 / span 3;
  background: var(--color-background-soft);
  height: 500px;
  flex: 1;
}

.quick-links {
  grid-column: 4 / 5;
  background: var(--color-background-soft);
  height: 500px;
  flex: 1;
}
</style>
