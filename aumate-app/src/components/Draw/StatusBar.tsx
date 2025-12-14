/**
 * 状态栏组件 - 显示截图信息
 */
export default function StatusBar() {
  return (
    <div
      style={{
        position: "fixed",
        bottom: "10px",
        left: "50%",
        transform: "translateX(-50%)",
        padding: "8px 16px",
        background: "rgba(0, 0, 0, 0.75)",
        color: "white",
        borderRadius: "6px",
        fontSize: "12px",
        pointerEvents: "none",
        backdropFilter: "blur(10px)",
        zIndex: 1000,
      }}
    >
      <span>Screenshot Editor - Press ESC to exit</span>
    </div>
  );
}
