import { ref, reactive } from 'vue'
import { defineStore } from 'pinia'
import * as cookie from 'cookie'

export const useUserStore = defineStore('user', {
  state: () => {
    const username = cookie.parse(document.cookie)?.username || ''

    return {
      _username: username,
      _isLoggedIn: username?.length > 0,
    }
  },
  getters: {
    isLoggedIn(state) {
      return state._isLoggedIn
    },
    username(state) {
      return state._username
    },
  },
  actions: {
    updateUsername(username) {
      this._username = username
      this._isLoggedIn = username?.length > 0

      let cookies = `username=${username || ''};`

      if (!this._isLoggedIn) {
        // logout by deleting the cookie
        const pastDate = new Date(0).toUTCString()
        let cookieString = `${cookies} expires=${pastDate};`
      }

      document.cookie = cookies
    },
  },
})
