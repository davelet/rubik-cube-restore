<template>
  <div class="face" :style="faceStyle">
    <div v-for="(block, blockIndex) in 9" :key="blockIndex" class="block" :style="blockStyle(blockIndex)"></div>
  </div>
</template>

<script>
const colorMap = {
  0: 'yellow',  // UP
  1: 'gainsboro',   // DOWN
  2: 'blue',   // FRONT
  3: 'green',    // BACK
  4: 'orange',  // LEFT
  5: 'red'      // RIGHT
}
export default {
  name: 'RubiksCubeFace',
  props: {
    rotate: {
      type: String,
      required: true
    },
    translateZ: {
      type: String,
      required: true
    },
    color: {
      type: Array,
      required: true
    }
  },
  computed: {
    faceStyle() {
      return {
        position: 'absolute',
        width: '100%',
        height: '100%',
        transform: `${this.rotate} ${this.translateZ}`,
        backfaceVisibility: 'hidden',
        transformStyle: 'preserve-3d'
      }
    }
  },
  methods: {
    blockStyle(index) {
      const x = index % 3;
      const y = Math.floor(index / 3);
      return {
        position: 'absolute',
        width: '33.33%',
        height: '33.33%',
        backgroundColor: colorMap[this.color[x][y]],
        left: `${x * 33.33}%`,
        top: `${y * 33.33}%`,
        border: '1px solid black',
      }
    }
  }
}
</script>

<style scoped>
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