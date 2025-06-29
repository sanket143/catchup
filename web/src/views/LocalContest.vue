<script setup lang="ts">
import axios from 'axios'
import { ref, reactive, computed } from 'vue'

let timer = reactive({ timeLeft: null })
let contestCount = reactive({ value: -1 })
let contestName = computed(() => `Local Contest #${contestCount.value}`)
let currentContest = reactive({ contest: null })

function updateDisplay() {
  const contest = currentContest.contest
  const currentTime =
    contest.duration * 1000000 > Date.now() - contest.started_on * 1000
      ? contest.duration * 1000000 - (Date.now() - contest.started_on * 1000)
      : 0

  const milliseconds = Math.floor((currentTime % 1000) / 10)
  const seconds = Math.floor((currentTime / 1000) % 60)
  const minutes = Math.floor((currentTime / (1000 * 60)) % 60)
  const hours = Math.floor(currentTime / (1000 * 60 * 60))

  // Show milliseconds if less than 1 hour, otherwise show hours
  if (hours < 1) {
    timer.timeLeft = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}:${milliseconds.toString().padStart(2, '0')}`
  } else {
    timer.timeLeft = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
  }

  if (timer.timeLeft !== null) {
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
    console.log(contestCount.value)
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
      console.log(timer)
    }
  })
}

function createNewContest() {
  axios({
    method: 'post',
    url: '/api/contest/create',
    data: {
      name: contestName.value,
    },
  }).catch((err) => {
    console.error(err)
  })
}

getCurrentContest()
getContestCount()
</script>

<template>
  <div class="container">
    <div v-if="currentContest.contest != null">
      <div class="section">
        <h3>{{ currentContest.contest.name }}</h3>
        <div>
          <span class="label">Time left: </span> <span class="value">{{ timer.timeLeft }}</span>
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
    </div>
    <div v-else-if="contestCount.value > -1">
      <h3>{{ contestName }}</h3>
      <div>
        <button @click="createNewContest">Start contest</button>
      </div>
    </div>
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
</style>
