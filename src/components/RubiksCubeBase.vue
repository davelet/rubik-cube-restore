<template>
  <div class="cube" :style="cubeStyle">
    <RubiksCubeFace v-for="(face, index) in faces" :key="face.name"
      :rotate="face.rotate"
      :translateZ="translateZ(size)"
      :color="getFaceColor(index)"
    />
  </div>
</template>

<script>
import RubiksCubeFace from './RubiksCubeFace.vue'

const cubeFaces = {
  0: 'top',
  1: 'bottom',
  2: 'front',
  3: 'back',
  4: 'left',
  5: 'right',
}

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
    cubeState: {
      type: Array,
      required: true
    },
    converedFaces: {
      type: Object,
      required: true
    }
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
      let f = cubeFaces[faceIndex];
      let n = this.converedFaces[f];

      return this.cubeState[n];
    }
  },
}
</script>

<style scoped>
.cube {
  position: absolute;
}
</style>