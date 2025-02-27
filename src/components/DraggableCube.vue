<template>
  <div class="cube-container">
    <div class="control-panel">
      <div class="control-group">
        <label>当前时间：</label>
        <span class="time-display">{{ currentTime }}</span>
      </div>
      <div class="control-group">
        <button @click="refreshTime" class="rotate-button">刷新时间</button>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

export default {
  name: "DraggableCube",
  data() {
    return {
      currentTime: '加载中...',
      timer: null
    };
  },
  async mounted() {
    await this.refreshTime();
    // 每秒更新一次时间
    this.timer = setInterval(async () => {
      await this.refreshTime();
    }, 1000);
  },
  beforeUnmount() {
    if (this.timer) {
      clearInterval(this.timer);
    }
  },
  methods: {
    async refreshTime() {
      try {
        this.currentTime = await invoke('get_current_time');
      } catch (error) {
        console.error('获取时间失败:', error);
        this.currentTime = '获取时间失败';
      }
    }
  }
};
</script>

<style scoped>
.cube-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.control-panel {
  padding: 1rem;
  background-color: #f5f5f5;
  display: flex;
  gap: 1rem;
  align-items: center;
  justify-content: center;
}

.control-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.time-display {
  font-size: 1.2rem;
  font-weight: bold;
  color: #646cff;
}

.rotate-button {
  padding: 0.5rem 1rem;
  background-color: #646cff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.rotate-button:hover {
  background-color: #535bf2;
}

@media (prefers-color-scheme: dark) {
  .control-panel {
    background-color: #2f2f2f;
    color: white;
  }
  
  .time-display {
    color: #a4a9ff;
  }
}
</style>