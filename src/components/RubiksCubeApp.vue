<template>
  <div class="rubiks-cube-2d">
    <!-- 3D Coordinate System -->
    <div class="coordinate-system">
      <div class="axis x-axis"></div>
      <div class="axis y-axis"></div>
      <div class="axis z-axis"></div>
    </div>
    <RubiksCubeRotationControls @rotate="handleRotation" @reset="initCubeState" @debug-toggle="handleDebugToggle"
      @solve-panel-toggle="handleSolvePanelToggle" @solve-lower-layer="handleSolveLowerLayer"
      @solve-middle-layer="handleSolveMiddleLayer" @solve-upper-layer="handleSolveUpperLayer"
      :showDebugMessages="showDebugMessages" @shuffle="handleShuffle" />
    <div v-for="(cube, index) in cubes" :key="index" :style="cubeContainerStyle(cube)">
      <RubiksCubeSingleBack v-if="index === 0" :size="cube.size" :cubeState="cube.cubeState" />
      <RubiksCubeSingleRight v-if="index === 1" :size="cube.size" :cubeState="cube.cubeState" />
      <RubiksCubeSinglePreview v-if="index === 2" :size="cube.size" :cubeState="cube.cubeState" />
      <RubiksCubeSingleBottom v-if="index === 3" :size="cube.size" :cubeState="cube.cubeState" />
    </div>
    <ApiMessageDisplay ref="apiMessage" v-if="showDebugMessages" class="message-display" />
  </div>
</template>

<script>
import { useCubeStore } from '../stores/cube';
import RubiksCubeSingleNormal from './RubiksCubeSingleNormal.vue';
import RubiksCubeSingleRight from './RubiksCubeSingleRight.vue';
import RubiksCubeSingleBack from './RubiksCubeSingleBack.vue';
import RubiksCubeSingleBottom from './RubiksCubeSingleBottom.vue';
import RubiksCubeRotationControls from './RubiksCubeRotationControls.vue';
import ApiMessageDisplay from './ApiMessageDisplay.vue';
import TauriService from '../services/invoke-utils';

export default {
  name: 'RubiksCubeApp',

  components: {
    RubiksCubeSinglePreview: RubiksCubeSingleNormal,
    RubiksCubeSingleRight,
    RubiksCubeSingleBack,
    RubiksCubeSingleBottom,
    RubiksCubeRotationControls,
    ApiMessageDisplay
  },

  setup() {
    const store = useCubeStore();
    const cubeSize = store.cubeSize;
    const PANEL_SIZE = 200; // 面板展开/收起时的尺寸变化值
    return { store, cubeSize, PANEL_SIZE };
  },

  data() {
    return {
      showDebugMessages: false,
    };
  },

  computed: {
    cubes() {
      return this.store.cubes;
    },
    showCubePreviews() {
      return this.cubes && this.cubes.length === 4;
    }
  },

  methods: {
    cubeContainerStyle(cube) {
      return {
        position: 'absolute',
        left: `${cube.x + cube.size}px`,
        top: `${cube.y + cube.size}px`,
        transform: `rotateX(${cube.rotateX}deg) rotateY(${cube.rotateY}deg)`,
        transformStyle: 'preserve-3d'
      }
    },
    async handleRotation(params) {
      const result = await this.store.handleRotation(params);
      this.handleApiResponse('turn', params, result);
    },
    async initCubeState() {
      const result = await this.store.initCubeState();
      this.handleApiResponse('init_result', null, result, '初始化完成');
    },
    handleDebugToggle(value) {
      this.showDebugMessages = value;
    },
    async handleShuffle(times) {
      const result = await this.store.handleShuffle(times);
      this.handleApiResponse('shuffle', times, result);
    },
    handleApiResponse(type, params, result, successMessage) {
      if (result.success) {
        this.$refs.apiMessage?.addMessage(
          type,
          params,
          true,
          null,
          successMessage || result.result
        );
      } else {
        this.$refs.apiMessage?.addMessage(
          type,
          params,
          false,
          result.error || `${type}失败`
        );
      }
    },

    async handleSolvePanelToggle(isOpen) {
      try {
        const { result: { width, height } } = await TauriService.getWindowSize();
        if (isOpen) {
          await TauriService.resizeWindow({
            height: height + this.PANEL_SIZE,
            width: width + this.PANEL_SIZE
          });
        } else {
          await TauriService.resizeWindow({
            height: height - this.PANEL_SIZE,
            width: width - this.PANEL_SIZE
          });
        }
      } catch (error) {
        this.handleApiResponse('resize_window', { isOpen }, {
          success: false,
          error: '调整窗口大小失败' + error
        });
      }
    },

    async handleSolveLowerLayer() {
      try {
        const result = await TauriService.solveLayer('lower');
        this.handleApiResponse('solve_lower_layer', null, result, '底层求解完成');
      } catch (error) {
        this.handleApiResponse('solve_lower_layer', null, {
          success: false,
          error: '底层求解失败'
        });
      }
    },

    async handleSolveMiddleLayer() {
      try {
        const result = await TauriService.solveLayer('middle');
        this.handleApiResponse('solve_middle_layer', null, result, '中层求解完成');
      } catch (error) {
        this.handleApiResponse('solve_middle_layer', null, {
          success: false,
          error: '中层求解失败'
        });
      }
    },

    async handleSolveUpperLayer() {
      try {
        const result = await TauriService.solveLayer('upper');
        this.handleApiResponse('solve_upper_layer', null, result, '顶层求解完成');
      } catch (error) {
        this.handleApiResponse('solve_upper_layer', null, {
          success: false,
          error: '顶层求解失败'
        });
      }
    },
    async getWindowSize() {
      try {
        const { width, height } = await invoke('get_window_size');
        return { width, height };
      } catch (error) {
        this.handleApiResponse('solve_upper_layer', null, {
          success: false,
          error: '获取窗口大小失败:' + error
        });
        return { width: 800, height: 700 };
      }
    },
  },

  async mounted() {
    await this.initCubeState();
  }
};
</script>

<style scoped>
.rubiks-cube-2d {
  position: relative;
  width: 100%;
  height: 100%;
  margin: 0;
  perspective: 1000px;
  perspective-origin: 50% 50%;
  background-color: rgb(184, 163, 197);
  overflow: hidden;

}

.coordinate-system {
  position: absolute;
  top: 15%;
  left: 10%;
  transform-style: preserve-3d;
  transform: translate(-50%, -50%) translateY(min(20vh, 200px)) rotateX(-45deg) rotateY(45deg);
  opacity: 1;
  z-index: 10;
  scale: min(1.5, 15vh / 100);
  background-color: rgba(255, 255, 255, 0.2);
  border: 2px solid rgba(0, 0, 0, 0.5);
  padding: 20px;
  border-radius: 8px;
}

.axis {
  position: absolute;
  transform-origin: center center;
  transform-style: preserve-3d;
}

.x-axis {
  width: 200px;
  height: 2px;
  background-color: red;
  transform: translateX(-50%);
}

.y-axis {
  width: 2px;
  height: 200px;
  background-color: green;
  transform: translateY(-50%);
}

.z-axis {
  width: 2px;
  height: 200px;
  background-color: yellow;
  transform: rotateX(90deg) translateZ(100px);
}

.cube {
  transform-style: preserve-3d;
}

.message-display {
  position: fixed;
  bottom: 20px;
  right: 20px;
  z-index: 1000;
}
</style>