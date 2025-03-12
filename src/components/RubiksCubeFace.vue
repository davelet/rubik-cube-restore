<template>
  <div class="face" :style="faceStyle">
    <div v-for="(block, blockIndex) in 9" :key="blockIndex">
      <RubiksCubeFaceBlock
        :color="color[blockIndex % 3][Math.floor(blockIndex / 3)]"
        :x="blockIndex % 3"
        :y="Math.floor(blockIndex / 3)"
      />
    </div>
  </div>
</template>

<script>
import RubiksCubeFaceBlock from './RubiksCubeFaceBlock.vue'

export default {
  name: 'RubiksCubeFace',
  components: {
    RubiksCubeFaceBlock
  },
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