import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import HelloWorld from '../components/HelloWorld.vue'
import DraggableCube from '../components/DraggableCube.vue'
import RubiksCube from '../../rubiks-cube/src/components/RubiksCube.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/cube'
  },
  {
    path: '/hello',
    name: 'hello',
    component: HelloWorld,
    props: { msg: 'Hello Vue 3 + Vite' }
  },
  {
    path: '/cube',
    name: 'cube',
    component: DraggableCube
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