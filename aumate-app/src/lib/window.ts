import { getCurrentWindow, LogicalSize, PhysicalPosition, currentMonitor } from '@tauri-apps/api/window';

// 用于取消之前的动画
let currentAnimationId: number | null = null;

/**
 * Animate window resize and keep it centered
 * @param targetWidth Target width
 * @param targetHeight Target height
 * @param duration Animation duration (ms), default 300ms
 */
export async function animateResizeAndCenter(
    targetWidth: number,
    targetHeight: number,
    duration: number = 300
): Promise<void> {
    // 取消之前的动画
    if (currentAnimationId !== null) {
        cancelAnimationFrame(currentAnimationId);
        currentAnimationId = null;
    }

    const window = getCurrentWindow();
    const monitor = await currentMonitor();
    
    if (!monitor) {
        console.error('Unable to get monitor information');
        return;
    }

    // 获取屏幕尺寸（逻辑像素）
    const screenWidth = monitor.size.width / monitor.scaleFactor;
    const screenHeight = monitor.size.height / monitor.scaleFactor;

    // 获取显示器的位置偏移（逻辑像素）
    const monitorX = monitor.position.x / monitor.scaleFactor;
    const monitorY = monitor.position.y / monitor.scaleFactor;

    // 计算目标位置（逻辑像素，居中，考虑显示器偏移）
    const targetX = monitorX + (screenWidth - targetWidth) / 2;
    console.log(targetX, 'targetX')
    const targetY = monitorY + (screenHeight - targetHeight) / 2;
    console.log(targetY, 'targetY')

    // 获取当前窗口尺寸（逻辑像素）
    const currentSize = await window.innerSize();
    const currentWidth = currentSize.width;
    const currentHeight = currentSize.height;

    // 如果窗口已经是目标大小，直接居中并返回
    if (currentWidth === targetWidth && currentHeight === targetHeight) {
        await window.setSize(new LogicalSize(targetWidth, targetHeight));
        await window.setPosition(new PhysicalPosition(
            Math.round(targetX * monitor.scaleFactor),
            Math.round(targetY * monitor.scaleFactor)
        ));
        return;
    }

    // 获取当前窗口位置（物理像素，转换为逻辑像素）
    const currentPosition = await window.outerPosition();
    const currentX = currentPosition.x / monitor.scaleFactor;
    const currentY = currentPosition.y / monitor.scaleFactor;

    // 缓动函数 (easeInOutCubic)
    const easeInOutCubic = (t: number): number => {
        return t < 0.5 
            ? 4 * t * t * t 
            : 1 - Math.pow(-2 * t + 2, 3) / 2;
    };

    // 动画开始时间
    const startTime = Date.now();

    return new Promise((resolve) => {
        const animate = () => {
            const elapsed = Date.now() - startTime;
            const progress = Math.min(elapsed / duration, 1);
            const easedProgress = easeInOutCubic(progress);

            // 计算当前帧的尺寸（逻辑像素）
            const currentFrameWidth = Math.round(
                currentWidth + (targetWidth - currentWidth) * easedProgress
            );
            const currentFrameHeight = Math.round(
                currentHeight + (targetHeight - currentHeight) * easedProgress
            );

            // 计算当前帧的位置（逻辑像素）
            const currentFrameX = currentX + (targetX - currentX) * easedProgress;
            const currentFrameY = currentY + (targetY - currentY) * easedProgress;

            // 先设置尺寸，再设置位置（转换为物理像素）
            // 使用 Promise 链确保顺序，但不阻塞动画循环
            window.setSize(new LogicalSize(currentFrameWidth, currentFrameHeight))
                .then(() => {
                    return window.setPosition(new PhysicalPosition(
                        Math.round(currentFrameX * monitor.scaleFactor),
                        Math.round(currentFrameY * monitor.scaleFactor)
                    ));
                });

            // 继续动画或结束
            if (progress < 1) {
                currentAnimationId = requestAnimationFrame(animate);
            } else {
                currentAnimationId = null;
                // 确保最终状态精确
                window.setSize(new LogicalSize(targetWidth, targetHeight))
                    .then(() => {
                        return window.setPosition(new PhysicalPosition(
                            Math.round(targetX * monitor.scaleFactor),
                            Math.round(targetY * monitor.scaleFactor)
                        ));
                    })
                    .then(() => resolve());
            }
        };

        currentAnimationId = requestAnimationFrame(animate);
    });
}