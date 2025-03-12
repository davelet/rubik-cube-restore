import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import RubiksCubeApp from '../components/RubiksCubeApp.vue'
import GithubRubiksPage from '../components/github/GithubRubiksPage.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/cube'
  },
  {
    path: '/cube',
    name: 'cube',
    component: RubiksCubeApp
  },
  {
    path: '/github-rubiks',
    name: 'github',
    component: GithubRubiksPage
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router