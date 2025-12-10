import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useCallback, useEffect, useRef, useState } from "react";

interface ElementRect {
  min_x: number;
  min_y: number;
  max_x: number;
  max_y: number;
}

interface WindowElement {
  rect: ElementRect;
  window_id: number;
  title: string;
  app_name: string;
}

interface SelectionState {
  isSelecting: boolean;
  startX: number;
  startY: number;
  endX: number;
  endY: number;
}

type ScreenshotPhase = "selecting" | "captured" | "annotating";

export function ScreenshotMode() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [screenshotData, setScreenshotData] = useState<string | null>(null);
  const [phase, setPhase] = useState<ScreenshotPhase>("selecting");
  const [selection, setSelection] = useState<SelectionState>({
    isSelecting: false,
    startX: 0,
    startY: 0,
    endX: 0,
    endY: 0,
  });
  const [hoveredWindow, setHoveredWindow] = useState<WindowElement | null>(
    null
  );
  const [windows, setWindows] = useState<WindowElement[]>([]);
  const [mousePos, setMousePos] = useState({ x: 0, y: 0 });
  // Screen bounds can be used for coordinate mapping in multi-monitor setups
  const [_screenBounds, setScreenBounds] = useState<ElementRect | null>(null);

  // Capture screenshot on mount
  useEffect(() => {
    const captureScreen = async () => {
      try {
        // Initialize UI automation
        await invoke("init_ui_automation");

        // Get all windows for hover detection
        const windowList = await invoke<WindowElement[]>("get_window_elements");
        setWindows(windowList);

        // Capture all monitors
        const base64Image = await invoke<string>("capture_all_monitors", {
          format: "png",
        });
        setScreenshotData(`data:image/png;base64,${base64Image}`);

        // Get screen bounds
        const monitorInfo = await invoke<{
          x: number;
          y: number;
          width: number;
          height: number;
        }>("get_monitor_info", { x: 0, y: 0 });
        setScreenBounds({
          min_x: monitorInfo.x,
          min_y: monitorInfo.y,
          max_x: monitorInfo.x + monitorInfo.width,
          max_y: monitorInfo.y + monitorInfo.height,
        });
      } catch (error) {
        console.error("Failed to capture screen:", error);
      }
    };

    captureScreen();
  }, []);

  // Draw the screenshot and selection overlay
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !screenshotData) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const img = new Image();
    img.onload = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;

      // Draw the screenshot
      ctx.drawImage(img, 0, 0, canvas.width, canvas.height);

      // Draw dark overlay
      ctx.fillStyle = "rgba(0, 0, 0, 0.5)";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // If we have a hovered window and not selecting, highlight it
      if (hoveredWindow && !selection.isSelecting && phase === "selecting") {
        const rect = hoveredWindow.rect;
        // Clear the overlay in the window area to show the screenshot
        ctx.clearRect(
          rect.min_x,
          rect.min_y,
          rect.max_x - rect.min_x,
          rect.max_y - rect.min_y
        );
        ctx.drawImage(
          img,
          rect.min_x,
          rect.min_y,
          rect.max_x - rect.min_x,
          rect.max_y - rect.min_y,
          rect.min_x,
          rect.min_y,
          rect.max_x - rect.min_x,
          rect.max_y - rect.min_y
        );
        // Draw border around the window
        ctx.strokeStyle = "#0ea5e9";
        ctx.lineWidth = 2;
        ctx.strokeRect(
          rect.min_x,
          rect.min_y,
          rect.max_x - rect.min_x,
          rect.max_y - rect.min_y
        );
      }

      // Draw selection rectangle
      if (selection.isSelecting || phase === "captured") {
        const x = Math.min(selection.startX, selection.endX);
        const y = Math.min(selection.startY, selection.endY);
        const width = Math.abs(selection.endX - selection.startX);
        const height = Math.abs(selection.endY - selection.startY);

        if (width > 0 && height > 0) {
          // Clear the overlay in the selected area
          ctx.clearRect(x, y, width, height);
          ctx.drawImage(img, x, y, width, height, x, y, width, height);

          // Draw selection border
          ctx.strokeStyle = "#0ea5e9";
          ctx.lineWidth = 2;
          ctx.setLineDash([]);
          ctx.strokeRect(x, y, width, height);

          // Draw size indicator
          ctx.fillStyle = "#0ea5e9";
          ctx.font = "12px sans-serif";
          const sizeText = `${Math.round(width)} x ${Math.round(height)}`;
          const textMetrics = ctx.measureText(sizeText);
          const textX = x + (width - textMetrics.width) / 2;
          const textY = y > 25 ? y - 8 : y + height + 18;
          ctx.fillText(sizeText, textX, textY);
        }
      }

      // Draw crosshair cursor position
      if (phase === "selecting" && !selection.isSelecting) {
        ctx.strokeStyle = "#ffffff";
        ctx.lineWidth = 1;
        ctx.setLineDash([5, 5]);

        // Horizontal line
        ctx.beginPath();
        ctx.moveTo(0, mousePos.y);
        ctx.lineTo(canvas.width, mousePos.y);
        ctx.stroke();

        // Vertical line
        ctx.beginPath();
        ctx.moveTo(mousePos.x, 0);
        ctx.lineTo(mousePos.x, canvas.height);
        ctx.stroke();

        ctx.setLineDash([]);
      }
    };
    img.src = screenshotData;
  }, [
    screenshotData,
    selection,
    hoveredWindow,
    mousePos,
    phase,
  ]);

  // Find window at mouse position
  const findWindowAtPoint = useCallback(
    (x: number, y: number): WindowElement | null => {
      for (const win of windows) {
        if (
          x >= win.rect.min_x &&
          x <= win.rect.max_x &&
          y >= win.rect.min_y &&
          y <= win.rect.max_y
        ) {
          return win;
        }
      }
      return null;
    },
    [windows]
  );

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      if (phase !== "selecting") return;

      setSelection({
        isSelecting: true,
        startX: e.clientX,
        startY: e.clientY,
        endX: e.clientX,
        endY: e.clientY,
      });
      setHoveredWindow(null);
    },
    [phase]
  );

  const handleMouseMove = useCallback(
    (e: React.MouseEvent) => {
      setMousePos({ x: e.clientX, y: e.clientY });

      if (selection.isSelecting) {
        setSelection((prev) => ({
          ...prev,
          endX: e.clientX,
          endY: e.clientY,
        }));
      } else if (phase === "selecting") {
        // Find window at current mouse position
        const win = findWindowAtPoint(e.clientX, e.clientY);
        setHoveredWindow(win);
      }
    },
    [selection.isSelecting, phase, findWindowAtPoint]
  );

  const handleMouseUp = useCallback(
    (e: React.MouseEvent) => {
      if (!selection.isSelecting) {
        // Click without drag - select the hovered window
        if (hoveredWindow) {
          setSelection({
            isSelecting: false,
            startX: hoveredWindow.rect.min_x,
            startY: hoveredWindow.rect.min_y,
            endX: hoveredWindow.rect.max_x,
            endY: hoveredWindow.rect.max_y,
          });
          setPhase("captured");
        }
        return;
      }

      const width = Math.abs(e.clientX - selection.startX);
      const height = Math.abs(e.clientY - selection.startY);

      if (width > 5 && height > 5) {
        setSelection((prev) => ({
          ...prev,
          isSelecting: false,
          endX: e.clientX,
          endY: e.clientY,
        }));
        setPhase("captured");
      } else {
        // Too small, reset
        setSelection({
          isSelecting: false,
          startX: 0,
          startY: 0,
          endX: 0,
          endY: 0,
        });
      }
    },
    [selection, hoveredWindow]
  );

  const handleKeyDown = useCallback(
    async (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        if (phase === "captured") {
          // Go back to selecting
          setPhase("selecting");
          setSelection({
            isSelecting: false,
            startX: 0,
            startY: 0,
            endX: 0,
            endY: 0,
          });
        } else {
          // Close the screenshot window
          await getCurrentWindow().close();
        }
      } else if (e.key === "Enter" && phase === "captured") {
        await handleSave();
      }
    },
    [phase]
  );

  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [handleKeyDown]);

  const handleSave = async () => {
    try {
      const x = Math.min(selection.startX, selection.endX);
      const y = Math.min(selection.startY, selection.endY);
      const width = Math.abs(selection.endX - selection.startX);
      const height = Math.abs(selection.endY - selection.startY);

      // Capture the selected region
      const base64Image = await invoke<string>("capture_region", {
        region: {
          min_x: Math.round(x),
          min_y: Math.round(y),
          max_x: Math.round(x + width),
          max_y: Math.round(y + height),
        },
        format: "png",
      });

      // Get save path from settings or use default
      const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
      const filename = `screenshot_${timestamp}.png`;

      // For now, save to Pictures folder
      const savePath = `${await getDownloadsPath()}/${filename}`;
      await invoke("save_screenshot", {
        imageData: base64Image,
        filePath: savePath,
        format: "png",
      });

      // Close the window after saving
      await getCurrentWindow().close();
    } catch (error) {
      console.error("Failed to save screenshot:", error);
    }
  };

  const handleCopy = async () => {
    try {
      const x = Math.min(selection.startX, selection.endX);
      const y = Math.min(selection.startY, selection.endY);
      const width = Math.abs(selection.endX - selection.startX);
      const height = Math.abs(selection.endY - selection.startY);

      // Capture the selected region
      const base64Image = await invoke<string>("capture_region", {
        region: {
          min_x: Math.round(x),
          min_y: Math.round(y),
          max_x: Math.round(x + width),
          max_y: Math.round(y + height),
        },
        format: "png",
      });

      // TODO: Copy to clipboard (need to implement clipboard command)
      console.log("Screenshot captured, length:", base64Image.length);

      // Close the window after copying
      await getCurrentWindow().close();
    } catch (error) {
      console.error("Failed to copy screenshot:", error);
    }
  };

  const handleCancel = async () => {
    await getCurrentWindow().close();
  };

  // Get selection dimensions for toolbar positioning
  const selectionRect = {
    x: Math.min(selection.startX, selection.endX),
    y: Math.min(selection.startY, selection.endY),
    width: Math.abs(selection.endX - selection.startX),
    height: Math.abs(selection.endY - selection.startY),
  };

  return (
    <div
      className="fixed inset-0 cursor-crosshair"
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
    >
      <canvas ref={canvasRef} className="fixed inset-0 w-full h-full" />

      {/* Toolbar - show after selection */}
      {phase === "captured" && selectionRect.width > 0 && (
        <div
          className="fixed flex gap-1 bg-zinc-900/90 backdrop-blur-sm rounded-lg p-1 shadow-xl border border-zinc-700"
          style={{
            left: selectionRect.x + selectionRect.width / 2 - 100,
            top:
              selectionRect.y + selectionRect.height + 10 <
              window.innerHeight - 50
                ? selectionRect.y + selectionRect.height + 10
                : selectionRect.y - 50,
          }}
        >
          <button
            type="button"
            onClick={handleSave}
            className="px-3 py-1.5 text-sm text-white bg-sky-600 hover:bg-sky-500 rounded-md transition-colors"
          >
            Save
          </button>
          <button
            type="button"
            onClick={handleCopy}
            className="px-3 py-1.5 text-sm text-white bg-zinc-700 hover:bg-zinc-600 rounded-md transition-colors"
          >
            Copy
          </button>
          <button
            type="button"
            onClick={handleCancel}
            className="px-3 py-1.5 text-sm text-white bg-zinc-700 hover:bg-zinc-600 rounded-md transition-colors"
          >
            Cancel
          </button>
        </div>
      )}

      {/* Instructions overlay */}
      {phase === "selecting" && !selection.isSelecting && (
        <div className="fixed bottom-4 left-1/2 -translate-x-1/2 text-white/70 text-sm bg-black/50 px-4 py-2 rounded-lg">
          Click and drag to select a region, or click on a window to capture it.
          Press ESC to cancel.
        </div>
      )}
    </div>
  );
}

// Helper to get downloads path - simplified version
async function getDownloadsPath(): Promise<string> {
  // This is a placeholder - in a real app, you'd use Tauri's path API
  // or get it from settings
  return "C:/Users/Public/Pictures";
}
