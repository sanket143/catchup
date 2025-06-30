<script setup lang="ts">
import graphqlRequest from '@/client/graphqlRequest'
import { ref, watch, reactive } from 'vue'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()

let loginDisabled = reactive({ value: false })
let infoText = reactive({ value: '' })
let username = ref(userStore.getUsername)

const query = `
  mutation ($input: UserInput!) {
    createOrLoginUser(input: $input) {
      id
      username
    }
  }
`

async function login() {
  loginDisabled.value = true

  const variables = {
    input: {
      username: username.value,
    },
  }

  await graphqlRequest({ query, variables })
  userStore.updateUsername(username.value)
  infoText.value = 'Logged in!'
}
</script>

<template>
  <div class="container">
    <div class="settings-section">
      <div class="title">Login</div>
      <div class="settings">
        <div class="col-1">Codeforces username:</div>
        <div class="col-2">
          <input v-model="username" placeholder="Your username (e.g. sankxt143)" />
        </div>
      </div>
    </div>
    <div class="flex button-section">
      <button :class="{ disabled: loginDisabled.value }" @click="login">Submit</button>
      <div class="login-info" v-if="infoText.value.length > 0">
        {{ infoText.value }}
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
