<template>
    <div class="rubiks-page">
        <div class="cube-container">
            <RubiksCube />
        </div>
        <div class="description">
            <p>这一页是展示的仓库</p>
            <a href="https://github.com/davelet/Rubiks-Cube.git" target="_blank">
                https://github.com/davelet/Rubiks-Cube.git</a>
        </div>
    </div>
</template>

<script setup>
import RubiksCube from '../../rubiks-cube/src/components/RubiksCube.vue';

defineOptions({
  name: 'GithubRubiksPage'
})
</script>

<style scoped>
.rubiks-page {
    position: relative;
    height: 100vh;
    padding: 10px;
}

.description {
    position: absolute;
    top: 10px;
    left: 10px;
    border: 1px solid #646cff;
    background-color: #ceebcd;
    border-radius: 8px;
    padding: 10px;
    z-index: 1;
    /* Ensure the description is on top */
}

.cube-container {
    width: 100%;
    padding: 10px;
}
</style>
<style>
/* tp-dfwv 是外链这个右侧的控制面板 */
.tp-dfwv {
    position: absolute;
    top: 8rem;
    right: 8px;
    width: 256px;
}
</style>