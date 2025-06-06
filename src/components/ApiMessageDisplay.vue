<template>
  <div class="api-message-container">
    <transition-group name="message-list" tag="div">
      <div v-for="message in messages" :key="message.id" :class="['api-message', message.type]" @mouseenter="pauseTimer(message)" @mouseleave="resumeTimer(message)">
        <div class="message-header">
          <span class="api-name">{{ message.apiName }}</span>
          <span class="timestamp">{{ message.timestamp }}</span>
          <span class="timer" v-if="message.timerVisible">{{ message.remainingTime }}秒</span>
          <button class="clear-button" @click="removeMessage(message.id)">×</button>
        </div>
        <div class="message-content">
          <div class="params" v-if="message.params">
            <strong>参数：</strong>{{ JSON.stringify(message.params) }}
          </div>
          <div class="response" v-if="message.response">
            <strong>返回数据：</strong>{{ JSON.stringify(message.response) }}
          </div>
          <div class="error" v-if="message.error">
            <strong>错误：</strong>{{ message.error }}
          </div>
        </div>
      </div>
    </transition-group>
  </div>
</template>

<script>
export default {
  name: 'ApiMessageDisplay',
  data() {
    return {
      messages: []
    }
  },
  methods: {
    addMessage(apiName, params, isSuccess, error = null, response = null) {
      const message = {
        timerVisible: true,
        remainingTime: 5,
        id: Date.now(),
        apiName,
        params,
        response,
        status: isSuccess ? '成功' : '失败',
        type: isSuccess ? 'success' : 'error',
        error,
        timestamp: new Date().toLocaleTimeString(),
        isPaused: false,
        intervalId: null
      }
      
      this.messages.unshift(message)
      
      this.startTimer(message)
    },
    startTimer(message) {
      if (message.intervalId) {
        clearInterval(message.intervalId)
      }
      message.intervalId = setInterval(() => {
        if (!message.isPaused && message.remainingTime > 0) {
          const index = this.messages.findIndex(m => m.id === message.id)
          if (index !== -1) {
            this.messages[index] = { ...this.messages[index], remainingTime: this.messages[index].remainingTime - 1 }
            if (this.messages[index].remainingTime === 0) {
              this.removeMessage(message.id)
            }
          }
        }
      }, 1000);
    },
    removeMessage(id) {
      const index = this.messages.findIndex(m => m.id === id)
      if (index !== -1) {
        const message = this.messages[index]
        if (message.intervalId) {
          clearInterval(message.intervalId)
        }
        this.messages.splice(index, 1)
      }
    },
    pauseTimer(message) {
      message.isPaused = true
      if (message.intervalId) {
        clearInterval(message.intervalId)
      }
    },
    resumeTimer(message) {
      message.isPaused = false
      this.startTimer(message)
    },
    clearAllMessages() {
      this.messages.forEach(message => {
        if (message.intervalId) {
          clearInterval(message.intervalId);
        }
      });
      this.messages = [];
    }
  }
}
</script>

<style scoped>
.api-message-container {
  position: fixed;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  width: 80%;
  max-width: 600px;
  z-index: 1000;
}

.api-message {
  margin-bottom: 10px;
  padding: 12px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  background-color: white;
  transition: all 0.3s ease;
}

.api-message.success {
  border-left: 4px solid #4CAF50;
}

.api-message.error {
  border-left: 4px solid #f44336;
}

.timer {
  text-align: center;
  font-size: 0.9em;
  color: #666;
  margin-bottom: 8px;
}

.message-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.api-name {
  font-weight: bold;
}

.timestamp {
  color: #666;
  font-size: 0.9em;
}

.clear-button {
  background: none;
  border: none;
  color: #999;
  font-size: 18px;
  cursor: pointer;
  padding: 0 4px;
  line-height: 1;
}

.clear-button:hover {
  color: #666;
}

.message-content {
  font-size: 0.9em;
}

.message-list-enter-active,
.message-list-leave-active {
  transition: all 0.3s ease;
}

.message-list-enter-from,
.message-list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}
</style>