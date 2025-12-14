import {
  ArrowDown,
  ArrowDownToLine,
  ArrowUp,
  ArrowUpToLine,
} from "lucide-react";
import type React from "react";
import { SectionHeader } from "./SectionHeader";

export interface LayerActionsProps {
  className?: string;
  onSendToBack?: () => void;
  onSendBackward?: () => void;
  onBringForward?: () => void;
  onBringToFront?: () => void;
}

/**
 * 图层操作组件
 * 提供移动图层顺序的按钮
 */
export const LayerActions: React.FC<LayerActionsProps> = ({
  className = "",
  onSendToBack,
  onSendBackward,
  onBringForward,
  onBringToFront,
}) => {
  return (
    <div className={className}>
      <SectionHeader title="图层" />
      <div className="flex items-center gap-1.5 flex-wrap">
        <button
          type="button"
          className="w-10 h-10 rounded border border-gray-200 hover:border-blue-500 hover:bg-blue-50 flex items-center justify-center transition-all"
          onClick={onSendToBack}
          title="移到最底层"
        >
          <ArrowDownToLine size={16} className="text-gray-600" />
        </button>
        <button
          type="button"
          className="w-10 h-10 rounded border border-gray-200 hover:border-blue-500 hover:bg-blue-50 flex items-center justify-center transition-all"
          onClick={onSendBackward}
          title="下移一层"
        >
          <ArrowDown size={16} className="text-gray-600" />
        </button>
        <button
          type="button"
          className="w-10 h-10 rounded border border-gray-200 hover:border-blue-500 hover:bg-blue-50 flex items-center justify-center transition-all"
          onClick={onBringForward}
          title="上移一层"
        >
          <ArrowUp size={16} className="text-gray-600" />
        </button>
        <button
          type="button"
          className="w-10 h-10 rounded border border-gray-200 hover:border-blue-500 hover:bg-blue-50 flex items-center justify-center transition-all"
          onClick={onBringToFront}
          title="移到最顶层"
        >
          <ArrowUpToLine size={16} className="text-gray-600" />
        </button>
      </div>
    </div>
  );
};
