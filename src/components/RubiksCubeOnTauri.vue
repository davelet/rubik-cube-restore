<template>
  <div class="rubiks-cube-2d">
    <!-- 3D Coordinate System -->
    <div class="coordinate-system">
      <div class="axis x-axis"></div>
      <div class="axis y-axis"></div>
      <div class="axis z-axis"></div>
    </div>
    <div class="cube-state-display">
      <div v-for="face in cubeStateDisplay" :key="face.name" class="face-state">
        <h3>{{ face.name }}</h3>
        <div class="face-grid">
          <div v-for="(row, rowIndex) in face.colors" :key="rowIndex" class="face-row">
            <div v-for="(color, colIndex) in row" :key="colIndex" class="face-cell">
              {{ color }}
            </div>
          </div>
        </div>
      </div>
    </div>
    <div v-for="(cube, index) in cubes" :key="index" :style="cubeContainerStyle(cube)">
      <RubiksCubeBase :size="cube.size" :colors="getCubeColors(cube)" />
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import RubiksCubeBase from './RubiksCubeBase.vue';

export default {
  name: 'RubiksCubeOnTauri',
  components: {
    RubiksCubeBase
  },
  data() {
    return {
      cubeSize: 90,
      cubeState: null,
    };
  },
  async created() {
    try {
      this.cubeState = await invoke('init_get_get_state')
      // 移除控制台日志
    } catch (error) {
      console.error('Error fetching cube state:', error)
    }
  },
  computed: {
    spacing() {
      return this.cubeSize * 4;
    },
    // 添加用于显示魔方状态的计算属性
    cubeStateDisplay() {
      if (!this.cubeState) return [];
      const faces = ['上面', '下面', '前面', '后面', '左面', '右面'];
      return this.cubeState.map((face, index) => ({
        name: faces[index],
        colors: face
      }));
    },
    cubes() {
      if (!this.cubeState) return [];

      const colorMap = {
        0: 'yellow',  // UP
        1: 'gainsboro',   // DOWN
        2: 'blue',   // FRONT
        3: 'green',    // BACK
        4: 'orange',  // LEFT
        5: 'red'      // RIGHT
      };

      return [
        {
          x: 0,
          y: this.spacing * 0.1,
          size: this.cubeSize * 0.8,
          rotateX: -45,
          rotateY: 45,
          topColor: colorMap[this.cubeState[0][1][1]],
          bottomColor: colorMap[this.cubeState[1][1][1]],
          frontColor: colorMap[this.cubeState[3][1][1]],
          backColor: colorMap[this.cubeState[2][1][1]],
          leftColor: colorMap[this.cubeState[4][1][1]],
          rightColor: colorMap[this.cubeState[5][1][1]],
        },
        {
          x: this.spacing,
          y: this.spacing * 0.1,
          size: this.cubeSize * 0.85,
          rotateX: -45,
          rotateY: 45,
          topColor: colorMap[this.cubeState[0][1][1]],
          bottomColor: colorMap[this.cubeState[1][1][1]],
          frontColor: colorMap[this.cubeState[2][1][1]],
          backColor: colorMap[this.cubeState[3][1][1]],
          leftColor: colorMap[this.cubeState[5][1][1]],
          rightColor: colorMap[this.cubeState[4][1][1]],
        },
        {
          x: this.spacing / 2,
          y: this.spacing / 2,
          size: this.cubeSize,
          rotateX: -45,
          rotateY: 45,
          topColor: colorMap[this.cubeState[0][1][1]],
          bottomColor: colorMap[this.cubeState[1][1][1]],
          frontColor: colorMap[this.cubeState[2][1][1]],
          backColor: colorMap[this.cubeState[3][1][1]],
          leftColor: colorMap[this.cubeState[4][1][1]],
          rightColor: colorMap[this.cubeState[5][1][1]],
        },
        {
          x: this.spacing * 0.53,
          y: this.spacing * 0.99,
          size: this.cubeSize * 0.75,
          rotateX: -45,
          rotateY: 45,
          topColor: colorMap[this.cubeState[1][1][1]],
          bottomColor: colorMap[this.cubeState[0][1][1]],
          frontColor: colorMap[this.cubeState[2][1][1]],
          backColor: colorMap[this.cubeState[3][1][1]],
          leftColor: colorMap[this.cubeState[4][1][1]],
          rightColor: colorMap[this.cubeState[5][1][1]],
        },
      ];
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
  top: 30%;
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

.cube-state-display {
  display: none;
  flex-wrap: wrap;
  gap: 20px;
  margin-bottom: 20px;
  padding: 10px;
  background-color: #f5f5f5;
  border-radius: 8px;
}

.face-state {
  background-color: white;
  padding: 10px;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.face-grid {
  display: grid;
  grid-template-rows: repeat(3, 1fr);
  gap: 2px;
  margin-top: 8px;
}

.face-row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 2px;
}

.face-cell {
  padding: 8px;
  text-align: center;
  background-color: #eee;
  border-radius: 2px;
}
</style>