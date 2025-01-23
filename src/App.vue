<script setup>
import { RouterLink, RouterView } from 'vue-router';
import RubiksCube from '../../rubiks-cube/src/components/RubiksCube.vue';
</script>

<template>
  <div class="app-container">
    <nav class="nav-buttons">
      <RouterLink to="/hello" class="nav-button">Hello World</RouterLink>
      <RouterLink to="/cube" class="nav-button">Draggable Cube</RouterLink>
      <RouterLink to="/rubiks" class="nav-button">Rubiks Cube</RouterLink>
    </nav>
    <RouterView />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.nav-buttons {
  padding: 1rem;
  display: flex;
  gap: 1rem;
  justify-content: center;
  background-color: #f6f6f6;
}

.nav-button {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  background-color: #646cff;
  color: white;
  text-decoration: none;
  transition: background-color 0.3s;
}

.nav-button:hover {
  background-color: #535bf2;
}

@media (prefers-color-scheme: dark) {
  .nav-buttons {
    background-color: #2f2f2f;
  }
}
</style>
