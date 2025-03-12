<template>
  <div class="block" :style="blockStyle"></div>
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
  name: 'RubiksCubeFaceBlock',
  props: {
    color: {
      type: Number,
      required: true
    },
    x: {
      type: Number,
      required: true
    },
    y: {
      type: Number,
      required: true
    }
  },
  computed: {
    blockStyle() {
      return {
        position: 'absolute',
        width: '33.33%',
        height: '33.33%',
        backgroundColor: colorMap[this.color],
        left: `${this.x * 33.33}%`,
        top: `${this.y * 33.33}%`,
        border: '1px solid black',
      }
    }
  }
}
</script>

<style scoped>
.block {
  position: absolute;
  width: 33.33%;
  height: 33.33%;
  background-color: #fff;
  border: 1px solid black;
}
</style> 