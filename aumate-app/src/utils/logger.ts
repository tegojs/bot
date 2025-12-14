import { invoke } from "@tauri-apps/api/core";

/**
 * 日志工具 - 将日志输出到 Rust 终端
 * 使用方式：import { log } from "@/utils/logger";
 *          log.info("[DrawPage] message", data);
 */

const formatMessage = (...args: unknown[]): string => {
  return args
    .map((arg) => {
      if (typeof arg === "object") {
        try {
          return JSON.stringify(arg);
        } catch {
          return String(arg);
        }
      }
      return String(arg);
    })
    .join(" ");
};

const sendLog = (level: string, ...args: unknown[]) => {
  const message = formatMessage(...args);
  // 同时输出到浏览器控制台（方便调试）
  // eslint-disable-next-line no-console
  console[level as "log" | "info" | "warn" | "error"]?.(...args);
  // 发送到 Rust 终端
  invoke("frontend_log", { level, message }).catch(() => {
    // 忽略错误，避免无限循环
  });
};

export const log = {
  info: (...args: unknown[]) => sendLog("info", ...args),
  warn: (...args: unknown[]) => sendLog("warn", ...args),
  error: (...args: unknown[]) => sendLog("error", ...args),
  debug: (...args: unknown[]) => sendLog("debug", ...args),
};

// 便捷导出
export default log;
