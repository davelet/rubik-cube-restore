import React, { useEffect, useRef } from 'react';
import * as THREE from 'three';

const DraggableCube: React.FC = () => {
  const containerRef = useRef<HTMLDivElement>(null);
  const sceneRef = useRef<THREE.Scene>(null);
  const cameraRef = useRef<THREE.PerspectiveCamera>(null);
  const rendererRef = useRef<THREE.WebGLRenderer>(null);
  const cubeRef = useRef<THREE.Group>(null);

  useEffect(() => {
    if (!containerRef.current) return;

    // Scene, Camera, Renderer
    const scene = new THREE.Scene();
    sceneRef.current = scene;
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    cameraRef.current = camera;
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    rendererRef.current = renderer;
    renderer.setSize(window.innerWidth, window.innerHeight);
    containerRef.current.appendChild(renderer.domElement);

    // Cube
    const cube = createRubiksCube();
    cubeRef.current = cube;
    scene.add(cube);

    // 添加3个魔方
    const cube2 = createRubiksCube();
    cube2.position.set(-2, 0, -2);
    scene.add(cube2);

    const cube3 = createRubiksCube();
    cube3.position.set(2, 0, -2);
    scene.add(cube3);

    const cube4 = createRubiksCube();
    cube4.position.set(0, 0, -4);
    scene.add(cube4);

    // Lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);
    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(5, 5, 5);
    scene.add(directionalLight);

    // Camera Position
    camera.position.set(5, 5, 5);
    camera.lookAt(0, 2, 0);

    // Render Loop
    const animate = () => {
      requestAnimationFrame(animate);
      renderer.render(scene, camera);
    };
    animate();

    return () => {
      containerRef.current?.removeChild(renderer.domElement);
    };
  }, []);

  return <div ref={containerRef} style={{ width: '100%', height: '100vh' }} />;
};

function createRubiksCube() {
  const cubeGroup = new THREE.Group();
  const size = 0.6;
  const gap = 0.01;
  for (let x = -1; x <= 1; x++) {
    for (let y = -1; y <= 1; y++) {
      for (let z = -1; z <= 1; z++) {
        const geometry = new THREE.BoxGeometry(size, size, size);
        const materials = [
          new THREE.MeshStandardMaterial({ color: 0xff0000 }),
          new THREE.MeshStandardMaterial({ color: 0xff8c00 }),
          new THREE.MeshStandardMaterial({ color: 0xffff00 }),
          new THREE.MeshStandardMaterial({ color: 0xffffff }),
          new THREE.MeshStandardMaterial({ color: 0x0000ff }),
          new THREE.MeshStandardMaterial({ color: 0x00ff00 }),
        ];
        const cube = new THREE.Mesh(geometry, materials);
        cube.position.set(x * (size + gap), y * (size + gap), z * (size + gap));
        cubeGroup.add(cube);
      }
    }
  }
  return cubeGroup;
}

export default DraggableCube;
