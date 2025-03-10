import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

const colorMap = {
  0: 'yellow',  // UP
  1: 'gainsboro',   // DOWN
  2: 'blue',   // FRONT
  3: 'green',    // BACK
  4: 'orange',  // LEFT
  5: 'red'      // RIGHT
}
const cubeSize = 90;
const spacing = cubeSize * 4;

export const useCubeStore = defineStore('cube', {
  state: () => ({
    cubeState: null as number[][][] | null,
  }),

  getters: {
    cubes: (state) => {
      if (!state.cubeState) return []

      return [
        {
          x: 0,
          y: spacing * 0.1,
          size: cubeSize * 0.8,
          rotateX: -45,
          rotateY: 45,
          topColor: colorMap[state.cubeState[0][1][1]],
          bottomColor: colorMap[state.cubeState[1][1][1]],
          frontColor: colorMap[state.cubeState[3][1][1]],
          backColor: colorMap[state.cubeState[2][1][1]],
          leftColor: colorMap[state.cubeState[4][1][1]],
          rightColor: colorMap[state.cubeState[5][1][1]],
        },
        {
          x: spacing,
          y: spacing * 0.1,
          size: cubeSize * 0.85,
          rotateX: -45,
          rotateY: 45,
          topColor: colorMap[state.cubeState[0][1][1]],
          bottomColor: colorMap[state.cubeState[1][1][1]],
          frontColor: colorMap[state.cubeState[2][1][1]],
          backColor: colorMap[state.cubeState[3][1][1]],
          leftColor: colorMap[state.cubeState[5][1][1]],
          rightColor: colorMap[state.cubeState[4][1][1]],
        },
        {
          x: spacing / 2,
          y: spacing / 2,
          size: cubeSize,
          rotateX: -45,
          rotateY: 45,
          topColor: colorMap[state.cubeState[0][1][1]],
          bottomColor: colorMap[state.cubeState[1][1][1]],
          frontColor: colorMap[state.cubeState[2][1][1]],
          backColor: colorMap[state.cubeState[3][1][1]],
          leftColor: colorMap[state.cubeState[4][1][1]],
          rightColor: colorMap[state.cubeState[5][1][1]],
        },
        {
          x: spacing * 0.53,
          y: spacing * 0.99,
          size: cubeSize * 0.75,
          rotateX: -45,
          rotateY: 45,
          topColor: colorMap[state.cubeState[1][1][1]],
          bottomColor: colorMap[state.cubeState[0][1][1]],
          frontColor: colorMap[state.cubeState[2][1][1]],
          backColor: colorMap[state.cubeState[3][1][1]],
          leftColor: colorMap[state.cubeState[4][1][1]],
          rightColor: colorMap[state.cubeState[5][1][1]],
        },

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