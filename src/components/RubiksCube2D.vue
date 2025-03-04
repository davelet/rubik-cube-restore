<template>
  <div class="rubiks-cube-2d">
    <div class="cube" v-for="(cube, index) in cubes" :key="index" :style="cubeStyle(cube)">
      <div class="face" v-for="face in faces" :key="face.rotate"
        :style="faceStyle(face.rotate, translateZ(cube.size))">
        <div v-for="(block, blockIndex) in 9" :key="blockIndex" class="block" :style="blockStyle(cube[face.colorProp], blockIndex)"></div>
      </div>
    </div>
  </div>
</template>

<script>
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
    };
  },
  computed: {
    spacing() {
      return this.cubeSize * 4;
    },
    cubes() {
      return [
        {
          x: 0,
          y: this.spacing * 0.1,
          size: this.cubeSize * 0.9,
          topColor: 'yellow',
          bottomColor: 'white',
          leftColor: 'orange',
          rightColor: 'red',
          frontColor: 'green',
          backColor: 'green',
        },
        {
          x: this.spacing * 1.1,
          y: 0,
          size: this.cubeSize * 0.8,
          topColor: 'yellow',
          bottomColor: 'white',
          leftColor: 'red',
          rightColor: 'red',
          frontColor: 'blue',
          backColor: 'green',
        },
        {
          x: this.spacing / 2,
          y: this.spacing / 2,
          size: this.cubeSize * 1.1,
          topColor: 'yellow',
          bottomColor: 'white',
          leftColor: 'orange',
          rightColor: 'red',
          frontColor: 'blue',
          backColor: 'green',
        },
        {
          x: this.spacing * 0.55,
          y: this.spacing * 0.95,
          size: this.cubeSize * 0.85,
          topColor: 'gainsboro',
          bottomColor: 'yellow',
          leftColor: 'orange',
          rightColor: 'red',
          frontColor: 'blue',
          backColor: 'green',
        },
      ]
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
</style>