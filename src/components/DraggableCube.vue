<template>
  <div ref="canvasContainer" class="canvas-container"></div>
</template>

<script>
import * as THREE from 'three';

export default {
  name: "DraggableCube",
  mounted() {
    this.initThree();
  },
  created() {
    this.COLORS = {
      BLACK: 0x000000,
      YELLOW: 0xffff00,
      RED: 0xff0000,
      ORANGE: 0xffa500,
      GREEN: 0x00ff00,
      BLUE: 0x0000ff,
      LIGHT_GRAY: 0xeeeeee,
      WHITE: 0xffffff,
    };
  },
  methods: {
    initThree() {
      const container = this.$refs.canvasContainer;

      // 场景设置
      const scene = new THREE.Scene();
      scene.background = new THREE.Color(this.COLORS.LIGHT_GRAY);

      // 相机设置
      const camera = new THREE.PerspectiveCamera(
        75,
        container.clientWidth / container.clientHeight,
        0.1,
        1000
      );
      camera.position.z = 5;

      // 渲染器设置
      const renderer = new THREE.WebGLRenderer({ antialias: true });
      renderer.setSize(container.clientWidth, container.clientHeight);
      container.appendChild(renderer.domElement);

      // 魔方尺寸和层间隙
      const cubeSize = 1;
      const layerGap = 0.1;

      // 创建魔方层
      const layers = [];
      for (let z = 0; z < 3; z++) {
        const layer = [];
        for (let x = 0; x < 3; x++) {
          for (let y = 0; y < 3; y++) {
            const geometry = new THREE.BoxGeometry(cubeSize, cubeSize, cubeSize);
            const material = new THREE.MeshBasicMaterial({
              color: this.getCubeColor(x, y, z),
            });
            const cube = new THREE.Mesh(geometry, material);
            cube.position.set(
              (x - 1) * (cubeSize + layerGap),
              (y - 1) * (cubeSize + layerGap),
              (z - 1) * (cubeSize + layerGap)
            );
            layer.push(cube);
            scene.add(cube);

            // 标记中心块为不可拖动
            cube.userData.isDraggable = !(x === 1 && y === 1 && z === 1);
          }
        }
        layers.push(layer);
      }

      // 添加光源
      const light = new THREE.DirectionalLight(this.COLORS.WHITE, 1);
      light.position.set(10, 10, 10);
      scene.add(light);

      // 拖动控制变量
      let isDragging = false;
      let draggedCube = null;
      let startMousePosition = { x: 0, y: 0 };
      let startCubePosition = { x: 0, y: 0, z: 0 };

      // 鼠标事件处理
      container.addEventListener('mousedown', (event) => {
        isDragging = true;
        const raycaster = new THREE.Raycaster();
        const mouse = new THREE.Vector2(
          (event.offsetX / container.clientWidth) * 2 - 1,
          -(event.offsetY / container.clientHeight) * 2 + 1
        );
        raycaster.setFromCamera(mouse, camera);
        const intersects = raycaster.intersectObjects(scene.children);
        if (intersects.length > 0 && intersects[0].object.userData.isDraggable) {
          draggedCube = intersects[0].object;
          startMousePosition = { x: event.offsetX, y: event.offsetY };
          startCubePosition = { ...draggedCube.position };
        }
      });

      container.addEventListener('mousemove', (event) => {
        if (isDragging && draggedCube) {
          const deltaMove = {
            x: (event.offsetX - startMousePosition.x) / container.clientWidth,
            y: (event.offsetY - startMousePosition.y) / container.clientHeight,
          };
          const moveDistance = 0.1; // 拖动距离系数

          // 计算新位置
          const newPosition = {
            x: startCubePosition.x + deltaMove.x * moveDistance,
            y: startCubePosition.y + deltaMove.y * moveDistance,
            z: startCubePosition.z,
          };

          // 确保新位置在层内
          const layerIndex = layers.findIndex(layer => layer.includes(draggedCube));
          const cubeIndex = layers[layerIndex].findIndex(cube => cube === draggedCube);
          const xIndex = Math.floor(cubeIndex / 3);
          const yIndex = cubeIndex % 3;

          // 调整位置以保持对齐
          newPosition.x = Math.round(newPosition.x / (cubeSize + layerGap)) * (cubeSize + layerGap) - (cubeSize / 2);
          newPosition.y = Math.round(newPosition.y / (cubeSize + layerGap)) * (cubeSize + layerGap) - (cubeSize / 2);

          // 确保不会超出层边界
          newPosition.x = Math.min(Math.max(newPosition.x, -(2 * cubeSize + layerGap)), cubeSize);
          newPosition.y = Math.min(Math.max(newPosition.y, -(2 * cubeSize + layerGap)), cubeSize);

          // 应用新位置
          draggedCube.position.set(newPosition.x, newPosition.y, newPosition.z);
        }
      });

      container.addEventListener('mouseup', () => {
        isDragging = false;
        draggedCube = null;
      });

      container.addEventListener('mouseleave', () => {
        isDragging = false;
        draggedCube = null;
      });

      // 动画循环
      const animate = () => {
        requestAnimationFrame(animate);
        renderer.render(scene, camera);
      };
      animate();
    },
    getCubeColor(x, y, z) {
      // 定义专门用于魔方块颜色的数组
      const cubeColors = [
        this.COLORS.WHITE,
        this.COLORS.YELLOW,
        this.COLORS.RED,
        this.COLORS.ORANGE,
        this.COLORS.GREEN,
        this.COLORS.BLUE,
      ];

      // 使用颜色常量字段
      return (x === 1 && y === 1 && z === 1) ? this.COLORS.BLACK : cubeColors[(x + y + z) % cubeColors.length];
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