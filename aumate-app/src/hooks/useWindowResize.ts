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
  isInitialized: boolean;
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
  const [isInitialized, setIsInitialized] = useState(false);
  const currentSizeRef = useRef({ width: 0, height: 0 });
  const isAnimatingRef = useRef(false); // For synchronous check to prevent concurrent calls

  // Initialize current size on mount
  useEffect(() => {
    const initSize = async () => {
      try {
        const size = await getWindowSize();
        currentSizeRef.current = size;
        setIsInitialized(true);
      } catch (error) {
        console.error("Failed to get initial window size:", error);
        // Still mark as initialized to prevent blocking, use fallback size
        setIsInitialized(true);
      }
    };
    initSize();
  }, []);

  const resizeTo = useCallback(
    async (targetWidth: number, targetHeight: number) => {
      // Guard against concurrent calls
      if (isAnimatingRef.current) {
        return;
      }

      const currentSize = currentSizeRef.current;

      // Skip if already at target size
      if (
        Math.abs(currentSize.width - targetWidth) < 1 &&
        Math.abs(currentSize.height - targetHeight) < 1
      ) {
        return;
      }

      // Determine if growing or shrinking
      // If not initialized (size is 0,0), treat as growing to avoid wrong animation
      const isGrowing =
        currentSize.width === 0 ||
        currentSize.height === 0 ||
        targetWidth > currentSize.width ||
        targetHeight > currentSize.height;

      // Set animating flag synchronously to prevent concurrent calls
      isAnimatingRef.current = true;
      setIsAnimating(true);

      let resizeSucceeded = false;

      try {
        // Disable vibrancy before resize to avoid artifacts
        await setVibrancy(false);

        if (isGrowing) {
          // GROWING: Set window size first, then animate content
          await setWindowSizeImmediate(targetWidth, targetHeight);
          resizeSucceeded = true;
          setAnimationClass("window-content-growing");
          await sleep(duration);
        } else {
          // SHRINKING: Animate content first, then set window size
          setAnimationClass("window-content-shrinking");
          await sleep(duration);
          await setWindowSizeImmediate(targetWidth, targetHeight);
          resizeSucceeded = true;
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
        isAnimatingRef.current = false;

        // Only update tracked size if resize actually succeeded
        if (resizeSucceeded) {
          currentSizeRef.current = { width: targetWidth, height: targetHeight };
        }
      }
    },
    [duration],
  );

  return { resizeTo, isAnimating, animationClass, isInitialized };
}
