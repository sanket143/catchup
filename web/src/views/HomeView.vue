<script setup lang="ts">
import StatCard from '../components/StatCard.vue'
import Dashboard from '../components/Dashboard.vue'
import QuickLink from '../components/QuickLinks.vue'
import problemTagGroupsRequest from '@/client/problemTagGroup/list'
import userDashboardRequest from '@/client/user/dashboard'
import { ref, computed } from 'vue'

const state = ref({
  problemTagGroups: [],
  user: {},
})

const computedState = computed(() => {
  const user = state.value.user
  const contests = user?.contests
  const isSolvedProblem = (p) => p.verdict == 'OK'

  return {
    overallLevel: (user.level || 0).toString(),
    totalContests: (contests.length || 0).toString(),
    totalProblemsSolved: contests
      ?.reduce((result, { problems }) => result + problems?.filter(isSolvedProblem).length, 0)
      .toString(),
  }
})

function fetchProblemTagGroupDetails() {
  problemTagGroupsRequest()
    .then((resp) => {
      state.value.problemTagGroups = resp.problemTagGroups
    })
    .catch((err) => {
      console.error(err)
    })
}

function fetchUserDashboardDetails() {
  userDashboardRequest()
    .then((resp) => {
      state.value.user = resp.user
    })
    .catch((err) => {
      console.error(err)
    })
}

fetchProblemTagGroupDetails()
fetchUserDashboardDetails()
</script>

<template>
  <div class="stat-cards">
    <StatCard title="Total contests" :count="computedState.totalContests" />
    <StatCard title="Overall level" :count="computedState.overallLevel" />
    <StatCard title="Total problems solved" :count="computedState.totalProblemsSolved" />
    <StatCard title="Codeforces profile" :count="state.user.username" />
    <div class="dashboard">
      <Dashboard :problemTagGroups="state.problemTagGroups" />
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
