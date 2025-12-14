import { AlignCenter, AlignLeft, AlignRight, Type } from "lucide-react";
import type React from "react";
import { useCallback, useContext, useState } from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import {
  FontFamilyPublisher,
  FontSizePublisher,
  TextAlignPublisher,
} from "../extra";
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
  const [fontFamily, setFontFamily] = useState<number>(1);
  const [fontSize, setFontSize] = useState<number>(20);
  const [textAlign, setTextAlign] = useState<string>("left");

  // 获取 publisher contexts
  const fontFamilyContext = useContext(FontFamilyPublisher.context);
  const fontSizeContext = useContext(FontSizePublisher.context);
  const textAlignContext = useContext(TextAlignPublisher.context);

  // 订阅 publishers
  useStateSubscriber(FontFamilyPublisher, setFontFamily);
  useStateSubscriber(FontSizePublisher, setFontSize);
  useStateSubscriber(TextAlignPublisher, setTextAlign);

  // 处理器函数
  const handleFontFamilyChange = useCallback(
    (value: number) => {
      setFontFamily(value);
      fontFamilyContext.publish(value);
    },
    [fontFamilyContext],
  );

  const handleFontSizeChange = useCallback(
    (value: number) => {
      setFontSize(value);
      fontSizeContext.publish(value);
    },
    [fontSizeContext],
  );

  const handleTextAlignChange = useCallback(
    (value: string) => {
      setTextAlign(value);
      textAlignContext.publish(value);
    },
    [textAlignContext],
  );

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
        onChange={handleFontFamilyChange}
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
        onChange={handleFontSizeChange}
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
        onChange={handleTextAlignChange}
      />
    </div>
  );
};
