import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useCallback, useEffect, useState } from "react";
import { ElementLabel } from "./ElementLabel";
import type { ScannableElement } from "./types";

export const ElementScanPage: React.FC = () => {
  const [elements, setElements] = useState<ScannableElement[]>([]);
  const [hoveredLabel, setHoveredLabel] = useState<string | null>(null);
  const [pressedLabel, setPressedLabel] = useState<string | null>(null);
  const [isScanning, setIsScanning] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // 扫描元素
  const scanElements = useCallback(async () => {
    try {
      setIsScanning(true);
      setError(null);
      console.log("[ElementScan] Starting element scan...");

      const scannedElements =
        await invoke<ScannableElement[]>("scan_screen_elements");

      console.log(
        `[ElementScan] Found ${scannedElements.length} elements:`,
        scannedElements,
      );
      setElements(scannedElements);
    } catch (err) {
      console.error("[ElementScan] Failed to scan elements:", err);
      setError(String(err));
    } finally {
      setIsScanning(false);
    }
  }, []);

  // 触发元素操作
  const triggerAction = useCallback(
    async (elementId: string, elementType: string) => {
      try {
        // 根据元素类型决定操作
        const actionType =
          elementType === "InputField" ? "focus" : "click";

        console.log(
          `[ElementScan] Triggering ${actionType} on element:`,
          elementId,
        );

        setPressedLabel(elementId);

        await invoke("trigger_element_action", {
          elementId,
          actionType,
        });

        console.log("[ElementScan] Action triggered successfully");

        // 立即关闭窗口（移除延迟以提升响应速度）
        const window = getCurrentWindow();
        await window.hide();
      } catch (err) {
        console.error("[ElementScan] Failed to trigger action:", err);
        setError(String(err));
        setPressedLabel(null);
      }
    },
    [],
  );

  // 处理字母键按下
  const handleLetterPress = useCallback(
    (letter: string) => {
      const element = elements.find((e) => e.label === letter);
      if (element) {
        triggerAction(element.id, element.element_type);
      }
    },
    [elements, triggerAction],
  );

  // 关闭窗口
  const closeWindow = useCallback(async () => {
    console.log("[ElementScan] Closing window");
    const window = getCurrentWindow();
    await window.hide();
  }, []);

  // 确保窗口获得焦点
  useEffect(() => {
    const ensureFocus = async () => {
      try {
        const window = getCurrentWindow();
        await window.setFocus();
        console.log("[ElementScan] Window focus set");
      } catch (err) {
        console.error("[ElementScan] Failed to set focus:", err);
      }
    };
    ensureFocus();
  }, []);

  // 键盘事件处理
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      console.log("[ElementScan] KeyDown event:", e.key, e.code, e);
      
      // 按 Escape 关闭窗口
      if (e.key === "Escape" || e.code === "Escape") {
        console.log("[ElementScan] ESC pressed, closing window");
        e.preventDefault();
        e.stopPropagation();
        closeWindow();
        return;
      }

      // 按 Ctrl+5 也可以关闭窗口（与全局快捷键保持一致）
      if (e.ctrlKey && (e.key === "5" || e.code === "Digit5")) {
        console.log("[ElementScan] Ctrl+5 pressed, closing window");
        e.preventDefault();
        e.stopPropagation();
        closeWindow();
        return;
      }

      // 只响应单个字母键（无修饰键）
      if (
        e.key.match(/^[a-zA-Z]$/) &&
        !e.ctrlKey &&
        !e.altKey &&
        !e.metaKey &&
        !e.shiftKey
      ) {
        e.preventDefault();
        e.stopPropagation();
        const letter = e.key.toUpperCase();
        console.log("[ElementScan] Letter key pressed:", letter);
        handleLetterPress(letter);
      }
    };

    window.addEventListener("keydown", handleKeyDown, { capture: true });
    return () => {
      window.removeEventListener("keydown", handleKeyDown, {
        capture: true,
      });
    };
  }, [handleLetterPress, closeWindow]);

  // 启动时自动扫描
  useEffect(() => {
    console.log("[ElementScan] Component mounted, starting scan");
    scanElements();
  }, [scanElements]);

  return (
    <div
      className="fixed inset-0 bg-black/20"
      style={{ cursor: "crosshair" }}
    >
      {/* 加载中提示 */}
      {isScanning && (
        <div className="fixed top-8 left-1/2 transform -translate-x-1/2 z-[20000]">
          <div className="bg-gray-900/95 text-white px-6 py-3 rounded-lg shadow-lg flex items-center gap-3">
            <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white" />
            <span>扫描元素中...</span>
          </div>
        </div>
      )}

      {/* 错误提示 */}
      {error && (
        <div className="fixed top-8 left-1/2 transform -translate-x-1/2 z-[20000]">
          <div className="bg-red-900/95 text-white px-6 py-3 rounded-lg shadow-lg">
            错误: {error}
          </div>
        </div>
      )}

      {/* 帮助提示 */}
      {!isScanning && elements.length > 0 && (
        <div className="fixed bottom-8 left-1/2 transform -translate-x-1/2 z-[20000]">
          <div className="bg-gray-900/95 text-white px-6 py-3 rounded-lg shadow-lg text-sm">
            按字母键选择元素 • 按 ESC 或 Ctrl+5 退出
          </div>
        </div>
      )}

      {/* 无元素提示 */}
      {!isScanning && elements.length === 0 && !error && (
        <div className="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-[20000]">
          <div className="bg-gray-900/95 text-white px-8 py-6 rounded-lg shadow-lg text-center">
            <p className="text-lg mb-2">未找到可交互元素</p>
            <p className="text-sm text-gray-400">
              请确保屏幕上有输入框或任务栏图标
            </p>
            <button
              type="button"
              onClick={closeWindow}
              className="mt-4 px-4 py-2 bg-blue-500 hover:bg-blue-600 rounded text-white text-sm"
            >
              关闭 (ESC 或 Ctrl+5)
            </button>
          </div>
        </div>
      )}

      {/* 渲染元素标签 */}
      {elements.map((element) => (
        <ElementLabel
          key={element.id}
          element={element}
          isHovered={hoveredLabel === element.id}
          isPressed={pressedLabel === element.id}
          onHover={() => setHoveredLabel(element.id)}
          onLeave={() => setHoveredLabel(null)}
        />
      ))}
    </div>
  );
};

