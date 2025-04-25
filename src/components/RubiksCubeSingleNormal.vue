<template>
  <div class="cube" :style="cubeStyle">
    <RubiksCubeFace v-for="(face, index) in faces" :key="face.name"
      :rotate="face.rotate"
      :translateZ="translateZ(size)"
      :color="getFaceColor(face.faceIndex)"
    />
  </div>
</template>

<script>
import RubiksCubeFace from './RubiksCubeFace.vue'

export default {
  name: 'RubiksCubeSingleNormal',
  components: {
    RubiksCubeFace
  },
  props: {
    size: {
      type: Number,
      required: true
    },
    cubeState: {
      type: Array,
      required: true
    },

  },
  data() {
    return {
      faces: [
        { faceIndex: 0, name: 'top', rotate: 'rotateX(90deg)' },
        { faceIndex: 4, name: 'left', rotate: 'rotateY(-90deg)' },
        { faceIndex: 2, name: 'front', rotate: 'rotateY(0deg)' },
        { faceIndex: 5, name: 'right', rotate: 'rotateY(90deg)' },
        { faceIndex: 3, name: 'back', rotate: 'rotateY(180deg)' },
        { faceIndex: 1, name: 'bottom', rotate: 'rotateX(-90deg)' },
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
    getFaceColor(faceIndex) {
      return this.cubeState[faceIndex];
    }
  },
}
</script>

<style scoped>
.cube {
  position: absolute;
}
</style>