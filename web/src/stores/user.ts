import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
  persist: true,
  state: () => {
    return {
      _username: '',
      _isLoggedIn: false,
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
    updateUsername(username: string) {
      this._username = username
      this._isLoggedIn = username?.length > 0
    },
  },
})
