/**
 * ImageLayer - 图像渲染层
 * 负责显示截图、图像处理和渲染
 */
import React, {
  forwardRef,
  useCallback,
  useEffect,
  useImperativeHandle,
  useRef,
} from "react";
import type {
  CaptureBoundingBoxInfo,
  ImageBuffer,
  ImageLayerActionType,
  ImageSharedBufferData,
} from "./types";
import { zIndexs } from "./extra";

interface ImageLayerProps {
  style?: React.CSSProperties;
}

export const ImageLayer = forwardRef<
  ImageLayerActionType | undefined,
  ImageLayerProps
>((props, ref) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  // biome-ignore lint/suspicious/noExplicitAny: PixiJS application type
  const appRef = useRef<any>(null);
  // biome-ignore lint/suspicious/noExplicitAny: PixiJS sprite type
  const spriteRef = useRef<any>(null);

  // 初始化 Canvas
  const initPixi = useCallback(
    async (width: number, height: number) => {
      if (!canvasRef.current) return;

      console.log("[ImageLayer] Initializing canvas:", width, "x", height);
      
      // 设置 canvas 实际尺寸
      canvasRef.current.width = width;
      canvasRef.current.height = height;
    },
    [],
  );

  // 加载并显示图像
  const loadImage = useCallback(
    async (
      imageSrc: string | undefined,
      _imageBuffer: ImageBuffer | ImageSharedBufferData | undefined,
    ) => {
      if (!canvasRef.current || !imageSrc) {
        console.warn("[ImageLayer] Canvas not initialized or no image source");
        return;
      }

      try {
        // 使用 Canvas 2D 简单渲染图像
        const ctx = canvasRef.current.getContext("2d");
        if (!ctx) return;

        const img = new Image();
        img.onload = () => {
          ctx.clearRect(0, 0, canvasRef.current!.width, canvasRef.current!.height);
          ctx.drawImage(img, 0, 0);
          console.log("[ImageLayer] Image loaded:", img.width, "x", img.height);
        };
        img.src = imageSrc;
      } catch (error) {
        console.error("[ImageLayer] Failed to load image:", error);
      }
    },
    [],
  );

  // 暴露给父组件的方法
  useImperativeHandle(
    ref,
    () => ({
      async onCaptureReady(
        _imageSrc: string | undefined,
        _imageBuffer: ImageBuffer | ImageSharedBufferData | undefined,
      ) {
        console.log("[ImageLayer] onCaptureReady called");
        // 图像准备好但还不显示
      },

      async onCaptureLoad(
        imageSrc: string | undefined,
        imageBuffer: ImageBuffer | ImageSharedBufferData | undefined,
        boundingBox: CaptureBoundingBoxInfo,
      ) {
        console.log("[ImageLayer] onCaptureLoad called", boundingBox);

        // 初始化 PixiJS
        await initPixi(boundingBox.width, boundingBox.height);

        // 加载图像
        await loadImage(imageSrc, imageBuffer);
      },

      async onCaptureFinish() {
        console.log("[ImageLayer] onCaptureFinish called");
        // 清理资源
        if (appRef.current) {
          appRef.current.destroy(true, { children: true, texture: true });
          appRef.current = null;
        }
        spriteRef.current = null;
      },

      async onCaptureBoundingBoxInfoReady(width: number, height: number) {
        console.log(
          "[ImageLayer] onCaptureBoundingBoxInfoReady:",
          width,
          "x",
          height,
        );
        await initPixi(width, height);
      },

      async onExecuteScreenshot() {
        console.log("[ImageLayer] onExecuteScreenshot called");
      },

      async renderImageSharedBufferToPng() {
        console.log("[ImageLayer] renderImageSharedBufferToPng called");
        return undefined;
      },

      getCanvas() {
        return canvasRef.current;
      },
    }),
    [initPixi, loadImage],
  );

  // 清理
  useEffect(() => {
    return () => {
      if (appRef.current) {
        appRef.current.destroy(true, { children: true, texture: true });
      }
    };
  }, []);

  return (
    <div
      ref={containerRef}
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        width: "100%",
        height: "100%",
        zIndex: zIndexs.Draw_ImageLayer,
        pointerEvents: "none",
        ...props.style,
      }}
    >
      <canvas
        ref={canvasRef}
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          width: "100%",
          height: "100%",
        }}
      />
    </div>
  );
});

ImageLayer.displayName = "ImageLayer";

