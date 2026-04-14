/*If you're reading this, this Toolip is not currently used anywhere*/
import { useState, useRef } from "react";
import { createPortal } from "react-dom";
import "./Tooltip.css";

const TOOLTIP_HEIGHT = 36;
const OFFSET = 10;

interface Props {
  text: string;
  children: React.ReactNode;
}

export default function Tooltip({ text, children }: Props) {
  const [visible, setVisible] = useState(false);
  const [style, setStyle] = useState<React.CSSProperties>({});
  const [flipped, setFlipped] = useState(false);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const wrapperRef = useRef<HTMLDivElement>(null);

  const handleEnter = () => {
    timer.current = setTimeout(() => {
      if (wrapperRef.current) {
        const rect = wrapperRef.current.getBoundingClientRect();
        const centerX = rect.left + rect.width / 2;

        const spaceAbove = rect.top;
        const showBelow = spaceAbove < TOOLTIP_HEIGHT + OFFSET;
        setFlipped(showBelow);

        setStyle({
          left: centerX,
          top: showBelow ? rect.bottom + OFFSET : rect.top - OFFSET,
          transform: showBelow
            ? "translateX(-50%)"
            : "translateX(-50%) translateY(-100%)",
        });
      }
      setVisible(true);
    }, 1000);
  };

  const handleLeave = () => {
    if (timer.current) clearTimeout(timer.current);
    setVisible(false);
  };

  return (
    <div
      ref={wrapperRef}
      className="tooltip-wrapper"
      onMouseEnter={handleEnter}
      onMouseLeave={handleLeave}
    >
      {children}
      {createPortal(
        <div
          className={`tooltip-box ${visible ? "tooltip-visible" : ""} ${flipped ? "tooltip-flipped" : ""}`}
          style={style}
        >
          <span className="tooltip-text">{text}</span>
        </div>,
        document.body,
      )}
    </div>
  );
}
