<script setup>
import { ref } from 'vue'

defineProps({
  msg: String,
})

const count = ref(0)
const input = ref('')
const message = ref('')

async function greet() {
  console.log(input.value);
  message.value = input.value;
  count.value += 1;
}
</script>

<template>
  <main class="container">
    <h1>{{ msg }}</h1>
    <h2>{{count}} : {{ message }}</h2>
  
    <div class="row">
      <a href="https://tauri.app" target="_blank">
        <img src="../assets/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript" target="_blank">
        <img src="../assets/javascript.svg" class="logo vanilla" alt="JavaScript logo" />
      </a>
    </div>
    <p>Click on the Tauri logo to learn more about the framework</p>
  
    <input id="greet-input" v-model="input" placeholder="Enter a name..." />
    <button type="submit" @click="greet()">Greet</button>
  </main>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
