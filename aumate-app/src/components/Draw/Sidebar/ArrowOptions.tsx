import type React from "react";
import { ArrowEndPublisher, ArrowStartPublisher } from "../extra";
import { usePublisherState } from "../hooks/usePublisherState";
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
  const [arrowStart, setArrowStart] = usePublisherState(ArrowStartPublisher);
  const [arrowEnd, setArrowEnd] = usePublisherState(ArrowEndPublisher);

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
        onChange={setArrowStart}
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
        onChange={setArrowEnd}
      />
    </div>
  );
};
