import { useCallback, useEffect, useRef, useState } from "react";
import {
  getWindowSize,
  setVibrancy,
  setWindowSizeImmediate,
} from "@/services/window";

interface UseWindowResizeOptions {
  duration?: number; // animation duration in ms
}

interface UseWindowResizeReturn {
  resizeTo: (targetWidth: number, targetHeight: number) => Promise<void>;
  isAnimating: boolean;
  animationClass: string;
}

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/**
 * Hook for managing window resize with HTML-based animations
 *
 * Uses CSS animations for smooth transitions and temporarily disables
 * vibrancy during animations to avoid visual artifacts.
 *
 * Growing (smaller → larger): Set window size first, then animate content
 * Shrinking (larger → smaller): Animate content first, then set window size
 */
export function useWindowResize(
  options: UseWindowResizeOptions = {},
): UseWindowResizeReturn {
  const { duration = 200 } = options;
  const [isAnimating, setIsAnimating] = useState(false);
  const [animationClass, setAnimationClass] = useState<string>("");
  const currentSizeRef = useRef({ width: 0, height: 0 });
  const isInitializedRef = useRef(false);

  // Initialize current size on mount
  useEffect(() => {
    const initSize = async () => {
      if (isInitializedRef.current) return;
      try {
        const size = await getWindowSize();
        currentSizeRef.current = size;
        isInitializedRef.current = true;
      } catch (error) {
        console.error("Failed to get initial window size:", error);
      }
    };
    initSize();
  }, []);

  const resizeTo = useCallback(
    async (targetWidth: number, targetHeight: number) => {
      const currentSize = currentSizeRef.current;

      // Skip if already at target size
      if (
        Math.abs(currentSize.width - targetWidth) < 1 &&
        Math.abs(currentSize.height - targetHeight) < 1
      ) {
        return;
      }

      // Determine if growing or shrinking
      const isGrowing =
        targetWidth > currentSize.width || targetHeight > currentSize.height;

      setIsAnimating(true);

      try {
        // Disable vibrancy before resize to avoid artifacts
        await setVibrancy(false);

        if (isGrowing) {
          // GROWING: Set window size first, then animate content
          await setWindowSizeImmediate(targetWidth, targetHeight);
          setAnimationClass("window-content-growing");
          await sleep(duration);
        } else {
          // SHRINKING: Animate content first, then set window size
          setAnimationClass("window-content-shrinking");
          await sleep(duration);
          await setWindowSizeImmediate(targetWidth, targetHeight);
        }

        // Re-enable vibrancy after animation
        await setVibrancy(true);
      } catch (error) {
        console.error("Window resize failed:", error);
        // Try to re-enable vibrancy even on error
        try {
          await setVibrancy(true);
        } catch {
          // ignore
        }
      } finally {
        setAnimationClass("");
        setIsAnimating(false);
        currentSizeRef.current = { width: targetWidth, height: targetHeight };
      }
    },
    [duration],
  );

  return { resizeTo, isAnimating, animationClass };
}
