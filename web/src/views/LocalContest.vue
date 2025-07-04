<script setup lang="ts">
import axios from 'axios'
import { ref, reactive, computed } from 'vue'
import { useAppStore } from '@/stores/app'
import { useUserStore } from '@/stores/user'
import recentContestRequest from '@/client/contest/recent'
import createContestRequest from '@/client/contest/create'
import evaluateContestRequest from '@/client/contest/evaluate'

const appStore = useAppStore()
const userStore = useUserStore()

let timer = reactive({ timeLeftLabel: null, timeLeft: -1 })

const state = ref({
  fetchingRecentContest: true,
  recentContest: null,
})

const contestName = computed(() => `Local Contest #${state?.value?.recentContest?.id || 0}`)

function updateDisplay() {
  const contest = state.value.recentContest

  timer.timeLeft =
    contest?.duration * 60 * 1000 > Date.now() - contest?.started_on * 1000
      ? contest?.duration * 60 * 1000 - (Date.now() - contest?.started_on * 1000)
      : 0

  const milliseconds = Math.floor((timer.timeLeft % 1000) / 10)
  const seconds = Math.floor((timer.timeLeft / 1000) % 60)
  const minutes = Math.floor((timer.timeLeft / (1000 * 60)) % 60)
  const hours = Math.floor(timer.timeLeft / (1000 * 60 * 60))

  // Show milliseconds if less than 1 hour, otherwise show hours
  if (hours < 1) {
    timer.timeLeftLabel = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}:${milliseconds.toString().padStart(2, '0')}`
  } else {
    timer.timeLeftLabel = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
  }

  if (timer.timeLeftLabel !== null) {
    requestAnimationFrame(updateDisplay)
  }
}

function createNewContest() {
  createContestRequest({
    input: {
      name: contestName.value,
    },
  }).then(() => {
    getRecentContest()
  })
}

function evaluteContestSubmissions() {
  if (!state.value.recentContest) {
    console.error('No contest to evaluate')
    return
  }

  evaluateContestRequest({
    input: {
      contestId: state.value.recentContest.id,
    },
  })
    .then((resp) => {
      getRecentContest()
    })
    .catch((err) => {
      console.error(err)
    })
}

function getRecentContest() {
  state.value.fetchingRecentContest = true
  recentContestRequest()
    .then((resp) => {
      state.value.fetchingRecentContest = false
      state.value.recentContest = resp?.user?.recentContest
      updateDisplay()
    })
    .catch((err) => {
      console.error(err)
    })
}

getRecentContest()
</script>

<template>
  <div class="container">
    <div v-if="!userStore.isLoggedIn">
      <router-link to="/login">Login first, it's just a Codeforces username anyways</router-link>
    </div>
    <div v-else-if="state.fetchingRecentContest">
      <h3>Loading contest details...</h3>
    </div>
    <div v-else-if="state.fetchingRecentContest == false">
      <div v-if="state.recentContest != null">
        <div class="section">
          <h3>{{ state.recentContest.name }}</h3>
          <div>
            <span class="label">Time left: </span>
            <span class="value" v-if="timer.timeLeft > 0">
              {{ timer.timeLeftLabel }}
            </span>
            <span class="value" v-else-if="timer.timeLeft == 0"> Completed </span>
          </div>
        </div>
        <div class="problems">
          <div class="problem" v-for="p in state.recentContest.problems">
            <div class="col-1">
              <a :href="p.problem.url" target="_blank">{{ p.problem.uid }}</a>
            </div>
            <div class="col-2">
              <a :href="p.problem.url" target="_blank">{{ p.problem.title }}</a>
            </div>
            <div class="col-3" v-if="p.isEvaluated">
              <span>{{ p.verdict }}</span>
            </div>
          </div>
        </div>

        <div v-if="timer.timeLeft == 0 || true">
          <div>
            <button @click="evaluteContestSubmissions">Evaluate submissions</button>
          </div>
        </div>
      </div>
      <div v-else>
        <h3>{{ contestName }}</h3>
        <div>
          <button @click="createNewContest">Start contest</button>
        </div>
      </div>
    </div>
    <!-- I'm aware, this will not make any difference if we refresh the page -->
    <div v-else-if="appStore.syncingProblemsInProgress">
      <h3>Codeforces problem set syncing in progress...</h3>
    </div>
  </div>
</template>

<style scoped>
h3 {
  font-weight: bold;
}

div.section {
  margin-bottom: 10px;
}

.problem {
  max-width: 600px;
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: var(--section-gap);
}

.col-1 {
  grid-column: 1;
}

.col-2 {
  grid-column: 2 / span 2;
}

.col-3 {
  grid-column: 4 / span 2;
}

.col-1 > a,
.col-2 > a {
  color: var(--color-text);
  text-decoration: none;
}

.col-1 > a {
  font-weight: bold;
}

.col-1 > a:hover,
.col-2 > a:hover {
  text-decoration: underline;
}

div.container {
  background: var(--color-background-soft);
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

span.label {
  font-weight: bold;
}

span.value {
  color: var(--color-text-primary);
  font-weight: bold;
}

button {
  margin-top: 10px;
}
</style>
