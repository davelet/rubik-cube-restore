<template>
  <div class="rubiks-cube-2d">
    <!-- 3D Coordinate System -->
    <div class="coordinate-system">
      <div class="axis x-axis"></div>
      <div class="axis y-axis"></div>
      <div class="axis z-axis"></div>
    </div>
    <RubiksCubeRotationControls 
      @rotate="handleRotation" 
      @reset="initCubeState" 
      @debug-toggle="handleDebugToggle"
      :showDebugMessages="showDebugMessages" 
    />
    <div 
      v-for="(cube, index) in cubes" 
      :key="index" 
      :style="cubeContainerStyle(cube)"
    >
      <RubiksCubeBase 
        :size="cube.size" 
        :cubeState="cube.cubeState" 
        :converedFaces="cube.coveredFaces" 
      />
    </div>
    <ApiMessageDisplay 
      ref="apiMessage" 
      v-if="showDebugMessages" 
      class="message-display"
    />
  </div>
</template>

<script>
import { useCubeStore } from '../stores/cube';
import RubiksCubeBase from './RubiksCubeBase.vue';
import RubiksCubeRotationControls from './RubiksCubeRotationControls.vue';
import ApiMessageDisplay from './ApiMessageDisplay.vue';

export default {
  name: 'RubiksCubeOnTauri',

  components: {
    RubiksCubeBase,
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
      showDebugMessages: false,
    };
  },

  computed: {
    cubes() {
      return this.store.cubes;
    },
  },

  methods: {
    cubeContainerStyle(cube) {
      return {
        position: 'absolute',
        transformStyle: 'preserve-3d',
        transform: `translate3d(${cube.x + cube.size}px, ${cube.y + cube.size / 2}px, 0px) rotateX(${cube.rotateX}deg) rotateY(${cube.rotateY}deg)`,
      }
    },

    handleDebugToggle(value) {
      this.showDebugMessages = value;
      this.handleDebugMessage(value);
    },

    handleDebugMessage(isEnabled) {
      if (isEnabled) {
        this.$refs.apiMessage?.addMessage(
          'debug_mode',
          null,
          true,
          null,
          '调试模式已启用'
        );
      } else {
        this.$refs.apiMessage?.clearAllMessages();
      }
    },

    async handleRotation(rotationParams) {
      const result = await this.store.handleRotation(rotationParams);
      this.handleApiResponse('turn', rotationParams, result);
    },

    async initCubeState() {
      const result = await this.store.initCubeState();
      this.handleApiResponse('init_result', null, result, '初始化完成');
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

  async created() {
    const result = await this.store.initCubeState();
  }
}
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

/* Coordinate System Styles */
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
  left: 50%;
  transform: translateX(-50%);
  z-index: 1000;
  width: 80%;
  max-width: 600px;
}
</style>