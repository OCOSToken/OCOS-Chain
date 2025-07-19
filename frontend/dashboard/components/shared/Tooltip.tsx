import React, { useState } from "react";

export interface TooltipProps {
  content: string | React.ReactNode;
  children: React.ReactNode;
  position?: "top" | "right" | "bottom" | "left";
}

export const Tooltip: React.FC<TooltipProps> = ({ content, children, position = "top" }) => {
  const [show, setShow] = useState(false);
  return (
    <span
      className="relative"
      onMouseEnter={() => setShow(true)}
      onMouseLeave={() => setShow(false)}
      onFocus={() => setShow(true)}
      onBlur={() => setShow(false)}
    >
      {children}
      {show && (
        <span className={`absolute z-10 ${position === "top" ? "bottom-full mb-2" : "top-full mt-2"} left-1/2 -translate-x-1/2 px-3 py-2 rounded bg-black text-white text-xs shadow-lg`}>
          {content}
        </span>
      )}
    </span>
  );
};
