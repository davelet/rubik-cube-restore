<script setup>
import { RouterLink, RouterView } from 'vue-router';
</script>

<template>
  <div class="app-container">
    <nav class="nav-buttons">
      <RouterLink to="/cube" class="nav-button" active-class="active">魔方观察</RouterLink>
      <RouterLink to="/github-rubiks" class="nav-button" active-class="active">外链魔方</RouterLink>
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
  background-color: #999;
  color: white;
  text-decoration: none;
  transition: background-color 0.3s;
}

.nav-button:hover {
  background-color: #777;
}

.nav-button.active {
  background-color: #646cff;
}

.nav-button.active:hover {
  background-color: #535bf2;
}

@media (prefers-color-scheme: dark) {
  .nav-buttons {
    background-color: #2f2f2f;
  }
}
</style>
