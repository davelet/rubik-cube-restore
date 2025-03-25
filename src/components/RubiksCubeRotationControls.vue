<template>
  <div class="controls-container">
    <div class="rotation-controls" :class="{ 'hidden': showSolvePanel }">
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
    <div class="solve-control">
      <button class="solve-btn" @click="toggleSolvePanel">层先法求解</button>

      <div class="solve-steps" v-show="showSolvePanel">
        <div class="solve-group bottom-layer">
          <button class="step-btn" @click="solveLowerCross">底层十字</button>
          <button class="step-btn" @click="solveLowerCorners">底层角块</button>
        </div>
        <button class="step-btn" @click="solveMiddleLayer">中层求解</button>
        <div class="solve-group top-layer">
          <button class="step-btn" @click="solveUpperCross">十字</button>
          <button class="step-btn" @click="solveUpperFace">顶面</button>
          <button class="step-btn" @click="solveUpperEdges">顶棱</button>
          <button class="step-btn" @click="solveUpperCorners">顶角</button>
        </div>
      </div>
    </div>
    <div class="shuffle-reset-container">
      <button @click="handleShuffle" class="shuffle-button">随机打乱</button>
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
      faces: ['上面', '下面', '前面', '后面', '左面', '右面'],
      showSolvePanel: false
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
    handleShuffle() {
      this.$emit('shuffle', 20);
    },
    toggleDebug(event) {
      this.$emit('debug-toggle', event.target.checked);
    },
    async toggleSolvePanel() {
      this.showSolvePanel = !this.showSolvePanel;
      this.$emit('solve-panel-toggle', this.showSolvePanel);
    },
    solveLowerCross() {
      this.$emit('solve', 'lower-cross');
    },
    solveLowerCorners() {
      this.$emit('solve', 'lower-corners');
    },
    solveMiddleLayer() {
      this.$emit('solve', 'middle-layer');
    },
    solveUpperCross() {
      this.$emit('solve', 'upper-cross');
    },
    solveUpperFace() {
      this.$emit('solve', 'upper-face');
    },
    solveUpperEdges() {
      this.$emit('solve', 'upper-edges');
    },
    solveUpperCorners() {
      this.$emit('solve', 'upper-corners');
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
  align-items: flex-end;
}

.controls-container {
  background-color: rgba(255, 255, 255, 0.9);
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.rotation-controls {
  border: 1px solid #ddd;
  margin-bottom: 0;
  padding: 15px;
  transition: opacity 0.3s ease-in-out, max-height 0.3s ease-in-out;
  opacity: 1;
  max-height: 500px;
  overflow: hidden;
}

.rotation-controls.hidden {
  opacity: 0;
  max-height: 0;
  padding: 0;
  margin: 0;
  border: none;
}

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

.rotate-button {
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

.rotate-button:hover {
  background-color: #45a049;
}

.rotate-button:active {
  background-color: #3d8b40;
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

.solve-control {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin: 10px 0;
}

.solve-btn {
  padding: 8px 16px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.solve-btn:hover {
  background-color: #45a049;
}

.solve-steps {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 10px;
  background-color: #f5f5f5;
  border-radius: 4px;
}

.step-btn {
  padding: 6px 12px;
  background-color: #2196F3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.step-btn:hover {
  background-color: #1976D2;
}

.shuffle-reset-container {
  display: flex;
  gap: 15px;
  margin: 0px 0;
  width: 100%;
}

.shuffle-button,
.reset-button {
  padding: 4px;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 15px;
}

.shuffle-button {
  width: 55%;
}

.reset-button {
  width: 45%;
}

.shuffle-button {
  background-color: #FF9800;
}

.shuffle-button:hover {
  background-color: #F57C00;
}

.shuffle-button:active {
  background-color: #EF6C00;
}

.reset-button {
  background-color: #f44336;
}

.reset-button:hover {
  background-color: #da190b;
}

.reset-button:active {
  background-color: #c41810;
}

.solve-group {
  border: 1px solid #ddd;
  padding: 8px;
  border-radius: 4px;
  margin: 8px 0;
}

.solve-group button {
  margin: 4px 1px;
}
</style>