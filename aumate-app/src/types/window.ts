export interface WindowInfo {
  id: string;
  windowId: number;
  title: string;
  appName: string;
  processName: string;
  processPath: string;
  icon?: string;
  rect: {
    minX: number;
    minY: number;
    maxX: number;
    maxY: number;
  };
}
