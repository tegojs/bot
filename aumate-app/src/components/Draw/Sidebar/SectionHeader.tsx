import type React from "react";

export interface SectionHeaderProps {
  title: string;
}

/**
 * 分区标题组件
 */
export const SectionHeader: React.FC<SectionHeaderProps> = ({ title }) => {
  return (
    <div className="text-xs text-gray-500 font-medium mt-2 first:mt-0">
      {title}
    </div>
  );
};
