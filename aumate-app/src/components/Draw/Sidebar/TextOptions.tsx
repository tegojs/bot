import { AlignCenter, AlignLeft, AlignRight, Type } from "lucide-react";
import type React from "react";
import {
  FontFamilyPublisher,
  FontSizePublisher,
  TextAlignPublisher,
} from "../extra";
import { usePublisherState } from "../hooks/usePublisherState";
import { OptionRow } from "./OptionRow";
import { SectionHeader } from "./SectionHeader";

export interface TextOptionsProps {
  className?: string;
}

/**
 * 文字配置组件
 * 配置字体、字体大小、文本对齐
 */
export const TextOptions: React.FC<TextOptionsProps> = ({ className = "" }) => {
  const [fontFamily, setFontFamily] = usePublisherState(FontFamilyPublisher);
  const [fontSize, setFontSize] = usePublisherState(FontSizePublisher);
  const [textAlign, setTextAlign] = usePublisherState(TextAlignPublisher);

  return (
    <div className={className}>
      {/* 字体 */}
      <SectionHeader title="字体" />
      <OptionRow
        options={[
          {
            value: 1,
            label: "手写",
            icon: <Type size={16} className="font-serif" />,
          },
          {
            value: 2,
            label: "普通",
            icon: <Type size={16} className="font-sans" />,
          },
          {
            value: 3,
            label: "代码",
            icon: <Type size={16} className="font-mono" />,
          },
        ]}
        selectedValue={fontFamily}
        onChange={setFontFamily}
      />

      {/* 字体大小 */}
      <SectionHeader title="字体大小" />
      <OptionRow
        options={[
          { value: 16, label: "S", icon: "S" },
          { value: 20, label: "M", icon: "M" },
          { value: 28, label: "L", icon: "L" },
          { value: 36, label: "XL", icon: "XL" },
        ]}
        selectedValue={fontSize}
        onChange={setFontSize}
      />

      {/* 文本对齐 */}
      <SectionHeader title="文本对齐" />
      <OptionRow
        options={[
          { value: "left", label: "左对齐", icon: <AlignLeft size={16} /> },
          { value: "center", label: "居中", icon: <AlignCenter size={16} /> },
          { value: "right", label: "右对齐", icon: <AlignRight size={16} /> },
        ]}
        selectedValue={textAlign}
        onChange={setTextAlign}
      />
    </div>
  );
};
