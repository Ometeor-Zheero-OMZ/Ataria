"use client";

import ColorfulButton from "@/components/ui/ColorfulButton";
import SignupModal from "@/components/Modal/SignupModal";
import { useState } from "react";

const Cta = () => {
  const [isSignupVisible, setIsSignupVisible] = useState(false);

  const handleCloseSignup = () => {
    setIsSignupVisible(false);
  };

  const handleSignupClick = () => {
    setIsSignupVisible(true);
  };

  return (
    <>
      <section id="cta" className="px-[96px] pb-[50px] bg-black">
        <div className="flex flex-col justify-center items-center p-2">
          <h5 className="mt-20 font-bold text-white text-center xs:text-4xl xs:mb-10 sm:text-4xl md:text-4xl xl:text-5xl">
            Boost Your Creativity Now
          </h5>
          <div>
            <ColorfulButton
              title="Get Started"
              handleClick={handleSignupClick}
            />
          </div>
        </div>
      </section>

      <SignupModal isVisible={isSignupVisible} onClose={handleCloseSignup} />
    </>
  );
};

export default Cta;
