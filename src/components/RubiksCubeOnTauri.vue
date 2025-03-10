<template>
  <div class="rubiks-cube-2d">
    <!-- 3D Coordinate System -->
    <div class="coordinate-system">
      <div class="axis x-axis"></div>
      <div class="axis y-axis"></div>
      <div class="axis z-axis"></div>
    </div>
    <RubiksCubeRotationControls
      :faces="faces"
      @rotate="handleRotation"
    />
    <div v-for="(cube, index) in cubes" :key="index" :style="cubeContainerStyle(cube)">
      <RubiksCubeBase :size="cube.size" :colors="getCubeColors(cube)" />
    </div>
    <ApiMessageDisplay ref="apiMessage" />
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
      faces: ['上面', '下面', '前面', '后面', '左面', '右面']
    };
  },
  async created() {
    const result = await this.store.initCubeState()
    if (result.success) {
      this.$refs.apiMessage?.addMessage('init_get_get_state', null, true, null, result.result)
    } else {
      this.$refs.apiMessage?.addMessage('init_get_get_state', null, false, result.error)
    }
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
    getCubeColors(cube) {
      return {
        topColor: cube.topColor,
        bottomColor: cube.bottomColor,
        frontColor: cube.frontColor,
        backColor: cube.backColor,
        leftColor: cube.leftColor,
        rightColor: cube.rightColor
      }
    },
    async handleRotation(rotationParams) {
      const result = await this.store.handleRotation(rotationParams);
      if (result.success) {
        this.$refs.apiMessage?.addMessage('turn', rotationParams, true, null, result.result);
      } else {
        this.$refs.apiMessage?.addMessage('turn', rotationParams, false, result.error);
      }
    },
  },
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

.face {
  position: absolute;
  width: 100%;
  height: 100%;
  transform-style: preserve-3d;
}

.block {
  position: absolute;
  width: 33.33%;
  height: 33.33%;
  background-color: #fff;
  border: 1px solid black;
}

.rotation-controls {
  position: absolute;
  top: 20px;
  right: 20px;
  background-color: rgba(255, 255, 255, 0.9);
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.control-group {
  margin-bottom: 15px;
}

.control-group:last-child {
  margin-bottom: 0;
}

.control-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: bold;
}

select {
  width: 100%;
  padding: 8px;
  border-radius: 4px;
  border: 1px solid #ddd;
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.radio-group label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: normal;
}

input[type="radio"] {
  margin: 0;
}

.rotate-button {
  width: 100%;
  padding: 10px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  margin-top: 10px;
}

.rotate-button:hover {
  background-color: #45a049;
}

.rotate-button:active {
  background-color: #3d8b40;
}
</style>