import { useCallback, useContext, useState } from "react";
import {
  type StatePublisher,
  useStateSubscriber,
} from "@/hooks/useStatePublisher";

/**
 * 统一的状态管理 Hook
 * 简化 Publisher 的订阅和发布模式
 *
 * @param publisher - StatePublisher 对象
 * @param onChange - 可选的变化回调函数
 * @returns [value, handleChange] - 当前值和更新函数
 *
 * @example
 * const [strokeColor, setStrokeColor] = usePublisherState(StrokeColorPublisher);
 */
export function usePublisherState<T>(
  publisher: StatePublisher<T>,
  onChange?: (value: T) => void,
): readonly [T, (value: T) => void] {
  const [value, setValue] = useState<T>(publisher.defaultValue);
  const context = useContext(publisher.context);

  // 订阅 Publisher 的变化
  useStateSubscriber(publisher, (newValue) => {
    if (newValue !== undefined) {
      setValue(newValue);
    }
  });

  // 处理变化：更新本地状态 + 发布到 Publisher + 调用回调
  const handleChange = useCallback(
    (newValue: T) => {
      setValue(newValue);
      context.publish(newValue);
      onChange?.(newValue);
    },
    [context, onChange],
  );

  return [value, handleChange] as const;
}
