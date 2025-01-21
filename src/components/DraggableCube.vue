<template>
    <div ref="canvasContainer" class="canvas-container"></div>
</template>
  
  <script>
import * as THREE from "three";

export default {
  name: "DraggableCube",
  mounted() {
    this.initThree();
  },
  methods: {
    initThree() {
      const container = this.$refs.canvasContainer;

      // 设置场景
      const scene = new THREE.Scene();
      scene.background = new THREE.Color(0xeeeeee);

      // 设置相机
      const camera = new THREE.PerspectiveCamera(
        75,
        container.clientWidth / container.clientHeight,
        0.1,
        1000
      );
      camera.position.z = 5;

      // 设置渲染器
      const renderer = new THREE.WebGLRenderer({ antialias: true });
      renderer.setSize(container.clientWidth, container.clientHeight);
      container.appendChild(renderer.domElement);

      // 添加立方体
      const geometry = new THREE.BoxGeometry(3, 3, 3);
      //   const material = new THREE.MeshStandardMaterial({ color: 0x007bff });
      // 创建材质数组，每个面一个材质
      const materials = [
        new THREE.MeshBasicMaterial({ color: 0xffffff }), // 白色
        new THREE.MeshBasicMaterial({ color: 0xffff00 }), // 黄色
        new THREE.MeshBasicMaterial({ color: 0xff0000 }), // 红色
        new THREE.MeshBasicMaterial({ color: 0xffa500 }), // 橙色
        new THREE.MeshBasicMaterial({ color: 0x00ff00 }), // 绿色
        new THREE.MeshBasicMaterial({ color: 0x0000ff })  // 蓝色
      ];
      const cube = new THREE.Mesh(geometry, materials);

      // 设置立方体的旋转角度，以便从一个角落看到它
      cube.rotation.x = Math.PI / 6; // 30度沿x轴旋转
      cube.rotation.y = Math.PI / 4; // 45度沿y轴旋转
      scene.add(cube);

      // 添加光源
      const light = new THREE.DirectionalLight(0xffffff, 1);
      light.position.set(10, 10, 10);
      scene.add(light);

      // 添加鼠标拖动控制
      let isDragging = false;
      let previousMousePosition = { x: 0, y: 0 };

      container.addEventListener("mousedown", (event) => {
        isDragging = true;
      });

      container.addEventListener("mousemove", (event) => {
        if (isDragging) {
          const deltaMove = {
            x: event.offsetX - previousMousePosition.x,
            y: event.offsetY - previousMousePosition.y,
          };

          const rotationSpeed = 0.01;
          cube.rotation.y += deltaMove.x * rotationSpeed;
          cube.rotation.x += deltaMove.y * rotationSpeed;
        }

        previousMousePosition = {
          x: event.offsetX,
          y: event.offsetY,
        };
      });

      container.addEventListener("mouseup", () => {
        isDragging = false;
      });

      container.addEventListener("mouseleave", () => {
        isDragging = false;
      });

      // 动画循环
      const animate = () => {
        requestAnimationFrame(animate);
        renderer.render(scene, camera);
      };
      animate();
    },
  },
};
  </script>
  
  <style scoped>
.canvas-container {
    width: 100%;
    height: 100vh;
    overflow: hidden;
}
</style>
  