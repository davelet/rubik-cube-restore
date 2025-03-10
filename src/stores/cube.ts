import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

const cubeSize = 90;
const spacing = cubeSize * 4;

export const useCubeStore = defineStore('cube', {
  state: () => ({
    cubeState: null as number[][][] | null,
  }),

  getters: {
    cubes: (state) => {
      if (!state.cubeState) return []

      const cubeFaces = {
        top: 0,
        bottom: 1,
        front: 2,
        back: 3,
        left: 4,
        right: 5,
      }
      return [
        {
          x: 0,
          y: spacing * 0.1,
          size: cubeSize * 0.8,
          rotateX: -45,
          rotateY: 45,
          cubeState: state.cubeState,
          coveredFaces: {
            ...cubeFaces,
            front: 3,
            back: 2
          }
        },
        {
          x: spacing,
          y: spacing * 0.1,
          size: cubeSize * 0.85,
          rotateX: -45,
          rotateY: 45,
          cubeState: state.cubeState,
          coveredFaces: {
            ...cubeFaces,
            left: 5,
            right: 4
          }
        },
        {
          x: spacing / 2,
          y: spacing / 2,
          size: cubeSize,
          rotateX: -45,
          rotateY: 45,
          cubeState: state.cubeState,
          coveredFaces: { ...cubeFaces, }
        },
        {
          x: spacing * 0.53,
          y: spacing * 0.99,
          size: cubeSize * 0.75,
          rotateX: -45,
          rotateY: 45,
          cubeState: state.cubeState,
          coveredFaces: {
            ...cubeFaces,
            top: 1,
            bottom: 0
          }
        }
      ]
    },
  },

  actions: {
    async initCubeState() {
      try {
        this.cubeState = await invoke('init_get_get_state')
        return { success: true, result: this.cubeState }
      } catch (error) {
        console.error('Error fetching cube state:', error)
        return { success: false, error }
      }
    },
    async handleRotation({ face, direction }) {
      const params = {
        state: this.cubeState,
        face,
        direction: direction === 0
      }

      try {
        const result = await invoke<number[][][]>('turn', params)
        this.cubeState = result
        return { success: true, result }
      } catch (error) {
        return { success: false, error }
      }
    },
  },
})