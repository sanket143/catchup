<script setup lang="ts">
import axios from 'axios'
import { ref, reactive, computed } from 'vue'
import { useAppStore } from '@/stores/app'
import { useUserStore } from '@/stores/user'

import listContestRequest from '@/client/contest/list'
import createContestRequest from '@/client/contest/create'
import evaluateContestRequest from '@/client/contest/evaluate'

const appStore = useAppStore()
const userStore = useUserStore()

let timer = reactive({ timeLeftLabel: null, timeLeft: -1 })

const state = ref({
  contests: [],
  recentContest: null,
  fetchingContestList: true,
})

const contestName = computed(() => `Local Contest #${state?.value?.recentContest?.id || 0}`)
const isRecentContestCompleted = computed(() => timer?.timeLeft == 0)

function updateTimeRemaining() {
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
    requestAnimationFrame(updateTimeRemaining)
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
  state.value.fetchingContestList = true

  listContestRequest()
    .then((resp) => {
      state.value.fetchingContestList = false
      state.value.recentContest = resp?.user?.recentContest
      state.value.contests = resp?.user?.contests.slice(1)

      state.value.contests = state.value.contests.map((contest) => {
        contest.totalProblems = contest.problems.length
        contest.noOfSolvedProblems = contest.problems.map((p) => p.verdict == 'OK').length

        return contest
      })

      updateTimeRemaining()
    })
    .catch((err) => {
      console.error(err)
    })
}

getRecentContest()
</script>

<template>
  <div>
    <div class="container">
      <div v-if="!userStore.isLoggedIn">
        <router-link to="/login">Login first, it's just a Codeforces username anyways</router-link>
      </div>
      <div v-else-if="state.fetchingContestList">
        <h3>Loading contest details...</h3>
      </div>
      <div v-else-if="state.fetchingContestList == false">
        <div v-if="state.recentContest != null">
          <div class="section">
            <h3>{{ state.recentContest.name }}</h3>
            <div>
              <span class="label">Time left: </span>
              <span class="value" v-if="timer.timeLeft > 0">
                {{ timer.timeLeftLabel }}
              </span>
              <span class="value" v-else-if="isRecentContestCompleted"> Completed </span>
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

          <div v-if="timer.timeLeft == 0 || true" class="flex">
            <div>
              <button @click="evaluteContestSubmissions">Evaluate submissions</button>
            </div>
            <div>
              <button @click="createNewContest">Start new contest</button>
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
    <div class="container" :v-if="isRecentContestCompleted && state.contests?.length > 0">
      <div>
        <h3>Past contests</h3>
      </div>

      <div v-for="ct in state.contests" class="contest">
        <div class="col-1">
          <span>{{ ct.name }}</span>
        </div>
        <div class="col-2">
          <span>{{ ct.noOfSolvedProblems }} / {{ ct.totalProblems }}</span>
        </div>
      </div>
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

.problem,
.contest {
  max-width: 600px;
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: var(--section-gap);
}

.contest {
  max-width: 100%;
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
  border-radius: 6px;
  margin-bottom: var(--section-gap);
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
