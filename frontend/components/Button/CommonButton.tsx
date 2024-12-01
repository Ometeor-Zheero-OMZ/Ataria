import { FC } from "react";
import { CommonButtonProps } from "@/types/type";

const CommonButton: FC<CommonButtonProps> = ({ children, onClick }) => {
  return (
    <div>
      <button
        id="signup-btn"
        type="button"
        className="inline-block px-[1.2em] py-[0.5em] text-[15px] no-underline cursor-pointer select-none font-bold text-white bg-[#9d5df1] border border-[#9d5df1] rounded-sm transition-all duration-300 hover:text-[#9d5df1] hover:bg-black"
        onClick={onClick}
      >
        {children}
      </button>
    </div>
  );
};

export default CommonButton;
