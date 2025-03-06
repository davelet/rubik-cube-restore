<template>
  <div class="cube" :style="cubeStyle">
    <RubiksCubeFace v-for="face in faces" :key="face.rotate"
      :rotate="face.rotate"
      :translateZ="translateZ(size)"
      :color="colors[face.colorProp]"
    />
  </div>
</template>

<script>
import RubiksCubeFace from './RubiksCubeFace.vue'

export default {
  name: 'RubiksCubeBase',
  components: {
    RubiksCubeFace
  },
  props: {
    size: {
      type: Number,
      required: true
    },
    colors: {
      type: Object,
      required: true
    }
  },
  data() {
    return {
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
    cubeStyle() {
      return {
        width: `${this.size}px`,
        height: `${this.size}px`,
        transformStyle: 'preserve-3d'
      }
    }
  },
  methods: {
    translateZ(size) {
      return `translateZ(${(size - 2) / 2 + 1}px)`
    },
  },
}
</script>

<style scoped>
.cube {
  position: absolute;
}
</style>