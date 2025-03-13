<template>
  <div class="rubiks-cube-2d">
    <!-- 3D Coordinate System -->
    <div class="coordinate-system">
      <div class="axis x-axis"></div>
      <div class="axis y-axis"></div>
      <div class="axis z-axis"></div>
    </div>
    <RubiksCubeRotationControls @rotate="handleRotation" @reset="initCubeState" @debug-toggle="handleDebugToggle"
      :showDebugMessages="showDebugMessages" />
    <div v-for="(cube, index) in cubes" :key="index" :style="cubeContainerStyle(cube)">
      <RubiksCubeSingleBack v-if="index === 0" :size="cube.size" :cubeState="cube.cubeState" />
      <RubiksCubeSingleRight v-if="index === 1" :size="cube.size" :cubeState="cube.cubeState" />
      <RubiksCubeSinglePreview v-if="index === 2" :size="cube.size" :cubeState="cube.cubeState" />
      <RubiksCubeSingleBottom v-if="index === 3" :size="cube.size" :cubeState="cube.cubeState" />
    </div>
    <ApiMessageDisplay ref="apiMessage" v-if="showDebugMessages" class="message-display" />
  </div>
</template>

<script>
import { useCubeStore } from '../stores/cube';
import RubiksCubeSingleNormal from './RubiksCubeSingleNormal.vue';
import RubiksCubeSingleRight from './RubiksCubeSingleRight.vue';
import RubiksCubeSingleBack from './RubiksCubeSingleBack.vue';
import RubiksCubeSingleBottom from './RubiksCubeSingleBottom.vue';
import RubiksCubeRotationControls from './RubiksCubeRotationControls.vue';
import ApiMessageDisplay from './ApiMessageDisplay.vue';

export default {
  name: 'RubiksCubeApp',

  components: {
    RubiksCubeSinglePreview: RubiksCubeSingleNormal,
    RubiksCubeSingleRight,
    RubiksCubeSingleBack,
    RubiksCubeSingleBottom,
    RubiksCubeRotationControls,
    ApiMessageDisplay
  },

  setup() {
    const store = useCubeStore();
    const cubeSize = store.cubeSize;
    return { store, cubeSize };
  },

  data() {
    return {
      showDebugMessages: false
    };
  },

  computed: {
    cubes() {
      return this.store.cubes;
    },
    showCubePreviews() {
      return this.cubes && this.cubes.length === 4;
    }
  },

  methods: {
    cubeContainerStyle(cube) {
      return {
        position: 'absolute',
        left: `${cube.x + cube.size}px`,
        top: `${cube.y + cube.size}px`,
        transform: `rotateX(${cube.rotateX}deg) rotateY(${cube.rotateY}deg)`,
        transformStyle: 'preserve-3d'
      }
    },
    async handleRotation(params) {
      const result = await this.store.handleRotation(params);
      this.handleApiResponse('turn', params, result);
    },
    async initCubeState() {
      const result = await this.store.initCubeState();
      this.handleApiResponse('init_result', null, result, '初始化完成');
    },
    handleDebugToggle(value) {
      this.showDebugMessages = value;
    },

    handleApiResponse(type, params, result, successMessage) {
      if (result.success) {
        this.$refs.apiMessage?.addMessage(
          type,
          params,
          true,
          null,
          successMessage || result.result
        );
      } else {
        this.$refs.apiMessage?.addMessage(
          type,
          params,
          false,
          result.error || `${type}失败`
        );
      }
    }
  },

  async mounted() {
    await this.initCubeState();
  }
};
</script>

<style scoped>
.rubiks-cube-2d {
  position: relative;
  width: 100%;
  height: 100%;
  margin: 0;
  perspective: 1000px;
  perspective-origin: 50% 50%;
  background-color: rgb(184, 163, 197);
  overflow: hidden;

}

.coordinate-system {
  position: absolute;
  top: 15%;
  left: 10%;
  transform-style: preserve-3d;
  transform: translate(-50%, -50%) translateY(min(20vh, 200px)) rotateX(-45deg) rotateY(45deg);
  opacity: 1;
  z-index: 10;
  scale: min(1.5, 15vh / 100);
  background-color: rgba(255, 255, 255, 0.2);
  border: 2px solid rgba(0, 0, 0, 0.5);
  padding: 20px;
  border-radius: 8px;
}

.axis {
  position: absolute;
  transform-origin: center center;
  transform-style: preserve-3d;
  /* Important for 3D transformations */
}

.x-axis {
  width: 200px;
  /* Adjust length as needed */
  height: 2px;
  background-color: red;
  transform: translateX(-50%);
}

.y-axis {
  width: 2px;
  height: 200px;
  /* Adjust length as needed */
  background-color: green;
  transform: translateY(-50%);
}

.z-axis {
  width: 2px;
  height: 200px;
  /* Adjust length as needed */
  background-color: yellow;
  transform: rotateX(90deg) translateZ(100px);
}

.cube {
  transform-style: preserve-3d;
}

.message-display {
  position: fixed;
  bottom: 20px;
  right: 20px;
  z-index: 1000;
}
</style>