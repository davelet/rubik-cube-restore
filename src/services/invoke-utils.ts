import { invoke } from '@tauri-apps/api/core';

interface Response<T> {
  success: boolean;
  result?: T;
  error?: string;
}

export default class TauriService {
  private static formatError(error: unknown, prefix: string): string {
    return `${prefix}: ${error instanceof Error ? error.message : String(error)}`;
  }

  private static createSuccessResponse<T>(result: T): Response<T> {
    return { success: true, result };
  }

  private static createErrorResponse<T>(error: unknown, prefix: string): Response<T> {
    return { success: false, error: this.formatError(error, prefix) };
  }


  static async resizeWindow({ width, height }: { width: number; height: number }): Promise<Response<string>> {
    try {
      await invoke('resize_window', { width, height });
      return this.createSuccessResponse('窗口大小调整成功');
    } catch (error: unknown) {
      return this.createErrorResponse(error, '窗口调整失败');
    }
  }

  static async solveLayer(params: { state: number[][][], target: number }): Promise<Response<{seq: String[], cube: number[][][]}>> {
    try {
      const steps = await invoke<{seq: String[], cube: number[][][]}>('solve', params);
      return this.createSuccessResponse(steps);
    } catch (error: unknown) {
      return this.createErrorResponse(error, `${params.target}求解失败`);
    }
  }

  static async getWindowSize(): Promise<Response<{ width: number; height: number }>> {
    try {
      const [width, height] = await invoke<number[]>('get_window_size');
      return this.createSuccessResponse({ width, height });
    } catch (error: unknown) {
      return this.createErrorResponse(error, '获取窗口尺寸失败');
    }
  }

  static async initCubeState(): Promise<Response<number[][][]>> {
    try {
      const state = await invoke<number[][][]>('init_get_get_state');
      return this.createSuccessResponse(state);
    } catch (error: unknown) {
      return this.createErrorResponse(error, '初始化魔方状态失败');
    }
  }

  static async handleRotation(params: { state: number[][][], face: number, direction: boolean }): Promise<Response<number[][][]>> {
    try {
      const result = await invoke<number[][][]>('turn', params);
      return this.createSuccessResponse(result);
    } catch (error: unknown) {
      return this.createErrorResponse(error, '魔方旋转失败');
    }
  }

    static async handleShuffle(params: { state: number[][][], times: number }): Promise<Response<number[][][]>> {
    try {
      const result = await invoke<number[][][]>('shuffle', params);
      return this.createSuccessResponse(result);
    } catch (error: unknown) {
      return this.createErrorResponse(error, '魔方打乱失败');
    }
  }

}