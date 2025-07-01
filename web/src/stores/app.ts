import { ref, reactive } from 'vue'
import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', {
  state: () => {
    return {
      _syncingProblemsInProgress: false,
    }
  },
  getters: {
    syncingProblemsInProgress(state) {
      return state._syncingProblemsInProgress
    },
  },
  actions: {
    updateSyncingProblemStatus(status) {
      this._syncingProblemsInProgress = status
    },
  },
})
