import { ReactNode } from "react";

const ColorfulButton = ({
  title,
  icon,
  position,
  handleClick,
  otherClasses,
}: {
  title: string;
  icon?: ReactNode;
  position?: string;
  handleClick?: () => void;
  otherClasses?: string;
}) => {
  return (
    <button className="p-[3px] top-1 relative" onClick={handleClick}>
      <div className="absolute inset-0 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-lg animate-btn-right" />

      <div
        className={`px-10 py-3  bg-black rounded-[6px]  relative group transition duration-300 text-white hover:bg-transparent animate-btn-right  ${otherClasses}`}
      >
        {position === "left" && icon}
        {title}
        {position === "right" && icon}
      </div>
    </button>
  );
};

export default ColorfulButton;
