import type { StrokeRoundness } from "@excalidraw/excalidraw/element/types";
import {
  ArrowDown,
  ArrowUp,
  Copy,
  Link,
  MoveDown,
  MoveUp,
  Scissors,
  Trash2,
} from "lucide-react";
import type React from "react";
import { OptionRow } from "./OptionRow";
import { SectionHeader } from "./SectionHeader";
import { SliderRow } from "./SliderRow";

interface ImageOptionsProps {
  cornerStyle: StrokeRoundness;
  opacity: number;
  onCornerStyleChange: (value: StrokeRoundness) => void;
  onOpacityChange: (value: number) => void;
  onCopy?: () => void;
  onDelete?: () => void;
  onCrop?: () => void;
  onLink?: () => void;
  onBringForward?: () => void;
  onSendBackward?: () => void;
  onBringToFront?: () => void;
  onSendToBack?: () => void;
}

export const ImageOptions: React.FC<ImageOptionsProps> = ({
  cornerStyle,
  opacity,
  onCornerStyleChange,
  onOpacityChange,
  onCopy,
  onDelete,
  onCrop,
  onLink,
  onBringForward,
  onSendBackward,
  onBringToFront,
  onSendToBack,
}) => {
  return (
    <>
      {/* 边角 */}
      <SectionHeader title="边角" />
      <OptionRow
        options={[
          { value: "round", label: "圆角", icon: "⬜" },
          { value: "sharp", label: "尖角", icon: "▢" },
        ]}
        selectedValue={cornerStyle}
        onChange={onCornerStyleChange}
      />

      {/* 透明度 */}
      <SectionHeader title="透明度" />
      <SliderRow value={opacity} min={0} max={100} onChange={onOpacityChange} />

      {/* 图层 */}
      <SectionHeader title="图层" />
      <div className="grid grid-cols-4 gap-1">
        <button
          type="button"
          onClick={onSendToBack}
          className="flex flex-col items-center justify-center p-2 rounded hover:bg-gray-100 transition-colors"
          title="置于底层"
        >
          <ArrowDown size={16} className="text-gray-700" />
        </button>
        <button
          type="button"
          onClick={onSendBackward}
          className="flex flex-col items-center justify-center p-2 rounded hover:bg-gray-100 transition-colors"
          title="下移一层"
        >
          <MoveDown size={16} className="text-gray-700" />
        </button>
        <button
          type="button"
          onClick={onBringForward}
          className="flex flex-col items-center justify-center p-2 rounded hover:bg-gray-100 transition-colors"
          title="上移一层"
        >
          <MoveUp size={16} className="text-gray-700" />
        </button>
        <button
          type="button"
          onClick={onBringToFront}
          className="flex flex-col items-center justify-center p-2 rounded hover:bg-gray-100 transition-colors"
          title="置于顶层"
        >
          <ArrowUp size={16} className="text-gray-700" />
        </button>
      </div>

      {/* 操作 */}
      <SectionHeader title="操作" />
      <div className="grid grid-cols-4 gap-1">
        <button
          type="button"
          onClick={onCopy}
          className="flex flex-col items-center justify-center p-2 rounded hover:bg-gray-100 transition-colors"
          title="复制"
        >
          <Copy size={16} className="text-gray-700" />
        </button>
        <button
          type="button"
          onClick={onDelete}
          className="flex flex-col items-center justify-center p-2 rounded hover:bg-gray-100 transition-colors"
          title="删除"
        >
          <Trash2 size={16} className="text-gray-700" />
        </button>
        <button
          type="button"
          onClick={onCrop}
          className="flex flex-col items-center justify-center p-2 rounded hover:bg-gray-100 transition-colors"
          title="裁剪"
        >
          <Scissors size={16} className="text-gray-700" />
        </button>
        <button
          type="button"
          onClick={onLink}
          className="flex flex-col items-center justify-center p-2 rounded hover:bg-gray-100 transition-colors"
          title="链接"
        >
          <Link size={16} className="text-gray-700" />
        </button>
      </div>
    </>
  );
};
