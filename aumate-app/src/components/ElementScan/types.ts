// 元素扫描相关类型定义

export interface ScannableElement {
  id: string;
  element_type: "InputField" | "TaskbarIcon";
  bounds: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
  title: string | null;
  label: string;
}

