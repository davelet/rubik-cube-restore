import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import RubiksCube2D from '../components/RubiksCube2D.vue'
import GithubRubiksPage from '../components/GithubRubiksPage.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/cube'
  },
  {
    path: '/cube',
    name: 'cube',
    component: RubiksCube2D
  },
  {
    path: '/rubiks',
    name: 'rubiks',
    component: GithubRubiksPage
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router