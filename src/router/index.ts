import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import RubiksCubeOnTauri from '../components/RubiksCubeOnTauri.vue'
import GithubRubiksPage from '../components/GithubRubiksPage.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/cube'
  },
  {
    path: '/cube',
    name: 'cube',
    component: RubiksCubeOnTauri
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