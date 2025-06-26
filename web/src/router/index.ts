import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/local-contest',
      name: 'local-contest',
      component: () => import('../views/LocalContest.vue'),
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/Settings.vue'),
    },
  ],
  linkActiveClass: 'active-link',
  linkExactActiveClass: 'exact-active-link',
})

export default router
