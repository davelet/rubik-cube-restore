import { defineStore } from 'pinia'
import TauriService from '../services/invoke-utils'

const cubeSize = 90;
const spacing = cubeSize * 4;

export const useCubeStore = defineStore('cube', {
  state: () => ({
    cubeState: undefined as number[][][] | undefined,
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
      const response = await TauriService.initCubeState();
      if (response.success) {
        this.cubeState = response.result;
      }
      return response;
    },
    async handleRotation({ face, direction }) {
      const params = {
        state: this.cubeState as number[][][],
        face,
        direction: direction === 0
      }

      const response = await TauriService.handleRotation(params);
      if (response.success) {
        this.cubeState = response.result;
      }
      return response;
    },
    async handleShuffle(times: number) {
      const params = {
        state: this.cubeState as number[][][],
        times
      };
      const response = await TauriService.handleShuffle(params);
      if (response.success) {
        this.cubeState = response.result;
      }
      return response;
    },

    async solveLayer(target: number) {
      const params = {
        state: this.cubeState as number[][][],
        target
      };
      const response = await TauriService.solveLayer(params);
      return response;
    },
  },
})