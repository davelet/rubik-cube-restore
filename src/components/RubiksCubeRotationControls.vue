<template>
  <div class="controls-container">
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
    <div class="reset-control">
      <button @click="handleReset" class="reset-button">初始化</button>
    </div>
    <div class="debug-control">
      <label class="debug-label">
        <input type="checkbox" :checked="showDebugMessages" @change="toggleDebug">
        显示调试消息
      </label>
    </div>
  </div>
</template>

<script>
export default {
  name: 'RotationControls',
  props: {
    showDebugMessages: {
      type: Boolean,
      default: false
    }
  },
  data() {
    return {
      selectedFace: 0,
      rotationDirection: 'clockwise',
      faces: ['上面', '下面', '前面', '后面', '左面', '右面']
    };
  },
  methods: {
    handleRotation() {
      this.$emit('rotate', {
        face: this.selectedFace,
        direction: this.rotationDirection === 'clockwise' ? 0 : 1
      });
    },
    handleReset() {
      this.$emit('reset');
    },
    toggleDebug(event) {
      this.$emit('debug-toggle', event.target.checked);
    }
  }
}
</script>

<style scoped>
.controls-container {
  position: absolute;
  top: 20px;
  right: 20px;
  display: flex;
  flex-direction: column;
  /* Align items vertically */
  align-items: flex-end;
  /* Align to the right */
}

.controls-container {
  background-color: rgba(255, 255, 255, 0.9);
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.rotation-controls,
.reset-control {
  margin-bottom: 0;
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

.rotate-button,
.reset-button {
  width: 100%;
  padding: 10px;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  margin-top: 10px;
}

.rotate-button {
  background-color: #4CAF50;
}

.reset-button {
  background-color: #f44336;
}

.rotate-button:hover {
  background-color: #45a049;
}

.reset-button:hover {
  background-color: #da190b;
}

.rotate-button:active {
  background-color: #3d8b40;
}

.reset-button:active {
  background-color: #c41810;
}

.debug-control {
  margin-top: 15px;
  padding-top: 15px;
  border-top: 1px solid #ddd;
}

.debug-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

.debug-label input[type="checkbox"] {
  margin: 0;
}
</style>