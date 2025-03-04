<template>
  <div class="rubiks-cube-2d">
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
    <div class="cube" v-for="(cube, index) in cubes" :key="index" :style="cubeStyle(cube)">
      <div class="face" v-for="face in faces" :key="face.rotate"
        :style="faceStyle(face.rotate, translateZ(cube.size))">
        <div v-for="(block, blockIndex) in 9" :key="blockIndex" class="block" :style="blockStyle(cube[face.colorProp], blockIndex)"></div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

export default {
  data() {
    return {
      cubeSize: 90,
      faces: [
        { colorProp: 'topColor', rotate: 'rotateX(90deg)' },
        { colorProp: 'leftColor', rotate: 'rotateY(-90deg)' },
        { colorProp: 'frontColor', rotate: 'rotateY(0deg)' },
        { colorProp: 'rightColor', rotate: 'rotateY(90deg)' },
        { colorProp: 'backColor', rotate: 'rotateY(180deg)' },
        { colorProp: 'bottomColor', rotate: 'rotateX(-90deg)' },
      ],
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
          size: this.cubeSize * 0.9,
          topColor: colorMap[this.cubeState[0][1][1]],
          bottomColor: colorMap[this.cubeState[1][1][1]],
          frontColor: colorMap[this.cubeState[3][1][1]],
          backColor: colorMap[this.cubeState[2][1][1]],
          leftColor: colorMap[this.cubeState[4][1][1]],
          rightColor: colorMap[this.cubeState[5][1][1]],
        },
        {
          x: this.spacing * 1.1,
          y: 0,
          size: this.cubeSize * 0.8,
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
          size: this.cubeSize * 1.1,
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
          size: this.cubeSize * 0.85,
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
    cubeStyle(cube) {
      return {
        position: 'absolute',
        transformStyle: 'preserve-3d',
        transform: `translate3d(${cube.x + 50}px, ${cube.y + 50}px, 0px) rotateX(-45deg) rotateY(45deg)`,
        width: `${cube.size}px`,
        height: `${cube.size}px`,
      }
    },
    faceStyle(rotate, translateZ) {
      return {
        position: 'absolute',
        width: '100%',
        height: '100%',
        transform: `${rotate} ${translateZ}`,
        backfaceVisibility: 'hidden',
      }
    },
    blockStyle(color, index) {
      const x = index % 3;
      const y = Math.floor(index / 3);
      return {
        position: 'absolute',
        width: '33.33%',
        height: '33.33%',
        backgroundColor: color,
        left: `${x * 33.33}%`,
        top: `${y * 33.33}%`,
        border: '1px solid black',
      }
    },
    translateZ(size) {
      return `translateZ(${(size - 2) / 2 + 1}px)`
    },
  },
}
</script>

<style scoped>
.rubiks-cube-2d {
  position: relative;
  width: 600px;
  height: 600px;
  margin: 5px auto;
  perspective: 1000px;
  perspective-origin: 250px 250px;
  background-color: black;
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