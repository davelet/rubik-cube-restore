import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import HelloWorld from '../components/HelloWorld.vue'
import DraggableCube from '../components/DraggableCube.vue'

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
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router