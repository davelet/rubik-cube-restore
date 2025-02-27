import React, { useEffect, useRef } from 'react';
import * as THREE from 'three';
import { Pane } from 'tweakpane';
import anime from 'animejs';

const DraggableCube: React.FC = () => {
  const containerRef = useRef<HTMLDivElement>(null);
  const sceneRef = useRef<THREE.Scene>();
  const cameraRef = useRef<THREE.PerspectiveCamera>();
  const rendererRef = useRef<THREE.WebGLRenderer>();
  const cubeRef = useRef<THREE.Group>();
  const isDraggingRef = useRef(false);
  const previousMousePositionRef = useRef({ x: 0, y: 0 });

  useEffect(() => {
    if (!containerRef.current) return;

    // 初始化场景
    sceneRef.current = new THREE.Scene();
    cameraRef.current = new THREE.PerspectiveCamera(
      75,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    );
    rendererRef.current = new THREE.WebGLRenderer({ antialias: true });

    const scene = sceneRef.current;
    const camera = cameraRef.current;
    const renderer = rendererRef.current;

    renderer.setSize(window.innerWidth, window.innerHeight);
    containerRef.current.appendChild(renderer.domElement);

    // 创建魔方
    cubeRef.current = createRubiksCube();
    scene.add(cubeRef.current);

    // 设置相机位置
    camera.position.z = 5;

    // 添加灯光
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
    directionalLight.position.set(10, 10, 10);
    scene.add(directionalLight);

    // 动画循环
    const animate = () => {
      requestAnimationFrame(animate);
      renderer.render(scene, camera);
    };
    animate();

    // 事件监听器
    const handleMouseDown = (event: MouseEvent) => {
      isDraggingRef.current = true;
      previousMousePositionRef.current = {
        x: event.clientX,
        y: event.clientY
      };
    };

    const handleMouseMove = (event: MouseEvent) => {
      if (!isDraggingRef.current || !cubeRef.current) return;

      const deltaMove = {
        x: event.clientX - previousMousePositionRef.current.x,
        y: event.clientY - previousMousePositionRef.current.y
      };

      cubeRef.current.rotation.y += deltaMove.x * 0.01;
      cubeRef.current.rotation.x += deltaMove.y * 0.01;

      previousMousePositionRef.current = {
        x: event.clientX,
        y: event.clientY
      };
    };

    const handleMouseUp = () => {
      isDraggingRef.current = false;
    };

    renderer.domElement.addEventListener('mousedown', handleMouseDown);
    renderer.domElement.addEventListener('mousemove', handleMouseMove);
    renderer.domElement.addEventListener('mouseup', handleMouseUp);

    // 清理函数
    return () => {
      renderer.domElement.removeEventListener('mousedown', handleMouseDown);
      renderer.domElement.removeEventListener('mousemove', handleMouseMove);
      renderer.domElement.removeEventListener('mouseup', handleMouseUp);
      containerRef.current?.removeChild(renderer.domElement);
    };
  }, []);

  return <div ref={containerRef} style={{ width: '100%', height: '100vh' }} />;
};

// 创建魔方的辅助函数
function createRubiksCube() {
  const cubeGroup = new THREE.Group();
  const size = 1;
  const gap = 0.1;

  for (let x = -1; x <= 1; x++) {
    for (let y = -1; y <= 1; y++) {
      for (let z = -1; z <= 1; z++) {
        const geometry = new THREE.BoxGeometry(size, size, size);
        const materials = [
          new THREE.MeshPhongMaterial({ color: 0xff0000 }), // 右
          new THREE.MeshPhongMaterial({ color: 0xff8c00 }), // 左
          new THREE.MeshPhongMaterial({ color: 0xffff00 }), // 上
          new THREE.MeshPhongMaterial({ color: 0xffffff }), // 下
          new THREE.MeshPhongMaterial({ color: 0x0000ff }), // 前
          new THREE.MeshPhongMaterial({ color: 0x00ff00 })  // 后
        ];

        const cube = new THREE.Mesh(geometry, materials);
        cube.position.set(
          x * (size + gap),
          y * (size + gap),
          z * (size + gap)
        );

        cubeGroup.add(cube);
      }
    }
  }

  return cubeGroup;
}

export default DraggableCube;