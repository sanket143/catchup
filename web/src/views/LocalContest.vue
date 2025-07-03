<script setup lang="ts">
import axios from 'axios'
import { ref, reactive, computed } from 'vue'
import { useAppStore } from '@/stores/app'
import { useUserStore } from '@/stores/user'
import recentContestRequest from '@/client/contest/recent'
import createContestRequest from '@/client/contest/create'

const appStore = useAppStore()
const userStore = useUserStore()

let timer = reactive({ timeLeftLabel: null, timeLeft: -1 })
let contestCount = reactive({ value: -1 })
let currentContest = reactive({ contest: null })

const state = ref({
  fetchingRecentContest: true,
  recentContest: null,
})
const contestName = computed(() => `Local Contest #${state?.value?.recentContest.id || 0}`)

function updateDisplay() {
  const contest = currentContest.contest

  timer.timeLeft =
    contest.duration * 60 * 1000 > Date.now() - contest.started_on * 1000
      ? contest.duration * 60 * 1000 - (Date.now() - contest.started_on * 1000)
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

// fetch numnber of contests, to have the default name of the contest
async function getContestCount() {
  axios({
    method: 'post',
    url: '/api/contest/count',
    data: {
      contestId: 1,
    },
  }).then((resp) => {
    contestCount.value = resp?.data?.count
  })
}

async function getCurrentContest() {
  axios({
    method: 'post',
    url: '/api/contest/current',
  }).then((resp) => {
    currentContest.contest = resp.data?.contest

    if (currentContest?.contest) {
      currentContest.contest.problems = resp.data?.problems
      updateDisplay()
    }
  })
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
  axios({
    method: 'post',
    url: '/api/contest/evaluate',
    data: {
      contestId: currentContest.contest.id,
    },
  }).catch((err) => {
    console.error(err)
  })
}

function getRecentContest() {
  recentContestRequest()
    .then((resp) => {
      state.value.fetchingRecentContest = false
      state.value.recentContest = resp
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
    <div v-else-if="currentContest.contest != null">
      <div class="section">
        <h3>{{ currentContest.contest.name }}</h3>
        <div>
          <span class="label">Time left: </span>
          <span class="value" v-if="timer.timeLeft > 0">
            {{ timer.timeLeftLabel }}
          </span>
          <span class="value" v-else-if="timer.timeLeft == 0"> Completed </span>
        </div>
      </div>
      <div class="problems">
        <div class="problem" v-for="problem in currentContest.contest?.problems">
          <div class="col-1">
            <a :href="problem.url" target="_blank">{{ problem.uid }}</a>
          </div>
          <div class="col-2">
            <a :href="problem.url" target="_blank">{{ problem.title }}</a>
          </div>
        </div>
      </div>

      <div v-if="timer.timeLeft == 0 || true">
        <div>
          <button @click="evaluteContestSubmissions">Evaluate submissions</button>
        </div>
      </div>
    </div>
    <div v-else-if="appStore.syncingProblemsInProgress">
      <h3>Codeforces problem set syncing in progress...</h3>
    </div>
    <div v-else-if="!state.fetchingRecentContest">
      <h3>{{ contestName }}</h3>
      <div>
        <button @click="createNewContest">Start contest</button>
      </div>
    </div>
    <!-- I'm aware, this will not make any difference if we refresh the page -->
    <div v-else>
      <h3>Loading contest details...</h3>
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
