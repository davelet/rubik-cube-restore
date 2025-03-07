<template>
  <div class="rotation-controls">
    <div class="control-group">
      <label for="face-select">选择面：</label>
      <select id="face-select" v-model="selectedFace">
        <option v-for="(face, index) in faces" :key="index" :value="index">
          {{ face }}
        </option>
      </select>
    </div>
    <div class="control-group">
      <label>旋转方向：</label>
      <div class="radio-group">
        <label>
          <input type="radio" v-model="rotationDirection" value="clockwise">
          顺时针
        </label>
        <label>
          <input type="radio" v-model="rotationDirection" value="counterclockwise">
          逆时针
        </label>
      </div>
    </div>
    <div class="control-group">
      <button @click="handleRotation" class="rotate-button">执行旋转</button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'RotationControls',
  props: {
    faces: {
      type: Array,
      required: true
    }
  },
  data() {
    return {
      selectedFace: 0,
      rotationDirection: 'clockwise'
    };
  },
  methods: {
    handleRotation() {
      this.$emit('rotate', {
        face: this.selectedFace,
        direction: this.rotationDirection === 'clockwise' ? 0 : 1
      });
    }
  }
}
</script>

<style scoped>
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