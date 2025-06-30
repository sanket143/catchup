<script setup lang="ts">
import graphqlRequest from '@/client/graphqlRequest'
import { ref } from 'vue'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()

const state = ref({
  loginInfoText: '',
  disableLogin: false,
  username: userStore.getUsername,
})

const query = `
  mutation ($input: UserInput!) {
    createOrLoginUser(input: $input) {
      id
      username
    }
  }
`

function disableLogin(status = true) {
  state.value.disableLogin = status
}

function updateLoginInfo(status = true) {
  state.value.loginInfoText = status
    ? 'Logged in!'
    : 'Something went wrong, check network request or console!'
}

function updateUsernameInStore(username: String) {
  userStore.updateUsername(username)
}

function onUsernameChange(_event) {
  disableLogin(false)
}

async function login() {
  disableLogin()
  const username = state.value.username
  const variables = {
    input: {
      username: username,
    },
  }

  await graphqlRequest({ query, variables }).then(({ data: { data } }) => {
    if (data?.createOrLoginUser?.username) {
      updateUsernameInStore(username)
      updateLoginInfo(true)
    } else {
      updateLoginInfo(false)
      console.error('Invalid data format or reading data from incorrect path', data)
    }
  })
}
</script>

<template>
  <div class="container">
    <div class="settings-section">
      <div class="title">Login</div>
      <div class="settings">
        <div class="col-1">Codeforces username:</div>
        <div class="col-2">
          <input
            v-model="state.username"
            @input="onUsernameChange"
            placeholder="Your username (e.g. sankxt143)"
          />
        </div>
      </div>
    </div>
    <div class="flex button-section">
      <button :class="{ disabled: state.disableLogin }" @click="login">Submit</button>
      <div class="login-info" v-if="state.loginInfoText?.length > 0 && state.disableLogin">
        {{ state.loginInfoText }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  padding: 10px;
  background: var(--color-background-soft);
}

.title {
  font-weight: bold;
}

.settings {
  display: flex;
  max-width: 550px;
  gap: 10px;
}

input {
  border: none;
  width: 100%;
  background: var(--color-background);
}

.settings-section {
  margin-bottom: 10px;
}

.button-section {
  align-items: center;
}
</style>
