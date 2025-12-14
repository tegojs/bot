import type React from "react";
import { useCallback, useContext, useState } from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import { ArrowEndPublisher, ArrowStartPublisher } from "../extra";
import { OptionRow } from "./OptionRow";
import { SectionHeader } from "./SectionHeader";

export interface ArrowOptionsProps {
  className?: string;
}

/**
 * 箭头配置组件
 * 配置箭头起点和终点样式
 */
export const ArrowOptions: React.FC<ArrowOptionsProps> = ({
  className = "",
}) => {
  const [arrowStart, setArrowStart] = useState<string | null>(null);
  const [arrowEnd, setArrowEnd] = useState<string | null>("arrow");

  // 获取 publisher contexts
  const arrowStartContext = useContext(ArrowStartPublisher.context);
  const arrowEndContext = useContext(ArrowEndPublisher.context);

  // 订阅 publishers
  useStateSubscriber(ArrowStartPublisher, setArrowStart);
  useStateSubscriber(ArrowEndPublisher, setArrowEnd);

  // 处理器函数
  const handleArrowStartChange = useCallback(
    (value: string | null) => {
      setArrowStart(value);
      arrowStartContext.publish(value);
    },
    [arrowStartContext],
  );

  const handleArrowEndChange = useCallback(
    (value: string | null) => {
      setArrowEnd(value);
      arrowEndContext.publish(value);
    },
    [arrowEndContext],
  );

  return (
    <div className={className}>
      {/* 箭头起点 */}
      <SectionHeader title="起点" />
      <OptionRow
        options={[
          { value: null, label: "无", icon: "—" },
          { value: "arrow", label: "箭头", icon: "←" },
          { value: "dot", label: "圆点", icon: "●" },
          { value: "bar", label: "条形", icon: "┤" },
        ]}
        selectedValue={arrowStart}
        onChange={handleArrowStartChange}
      />

      {/* 箭头终点 */}
      <SectionHeader title="端点" />
      <OptionRow
        options={[
          { value: null, label: "无", icon: "—" },
          { value: "arrow", label: "箭头", icon: "→" },
          { value: "dot", label: "圆点", icon: "●" },
          { value: "bar", label: "条形", icon: "├" },
        ]}
        selectedValue={arrowEnd}
        onChange={handleArrowEndChange}
      />
    </div>
  );
};
