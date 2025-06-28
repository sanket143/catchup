import { ref, reactive } from 'vue'
import { defineStore } from 'pinia'
import * as cookie from 'cookie'

export const useUserStore = defineStore('user', {
  state: () => {
    const username = cookie.parse(document.cookie)?.username || ''

    return {
      username,
    }
  },
  getters: {
    getUsername(state) {
      return state.username
    },
  },
  actions: {
    updateUsername(username) {
      this.username = username
      document.cookie = 'username=' + username + ';'
    },
  },
})
