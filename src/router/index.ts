import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import DraggableCube from '../components/DraggableCube.vue'
import RubiksCube from '../../rubiks-cube/src/components/RubiksCube.vue'
import RubiksCube2D from '../components/RubiksCube2D.vue'

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
    component: RubiksCube
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router