import { FC, useState } from "react";
import { useAuth } from "../context/AuthContext";
import SignupModal from "./Modal/SignupModal";
import LoginModal from "./Modal/LoginModal";
import CommonButton from "./Button/CommonButton";

const AuthenticationForm: FC = () => {
  const [isSignupVisible, setIsSignupVisible] = useState(false);
  const [isLoginVisible, setIsLoginVisible] = useState(false);

  const { user, signOut } = useAuth();

  const handleSignupClick = () => {
    setIsSignupVisible(true);
  };

  const handleLoginClick = () => {
    setIsLoginVisible(true);
  };

  const handleCloseSignup = () => {
    setIsSignupVisible(false);
  };

  const handleCloseLogin = () => {
    setIsLoginVisible(false);
  };

  return (
    <div>
      {user ? (
        <CommonButton onClick={signOut}>Sign Out</CommonButton>
      ) : (
        <div>
          <div
            id="signup-btn"
            className="inline-block px-[1.2em] py-[0.5em] text-[15px] no-underline cursor-pointer select-none font-bold text-white bg-[#9d5df1] border border-[#9d5df1] rounded-sm transition-all duration-300 hover:text-[#9d5df1] hover:bg-black"
            onClick={handleSignupClick}
          >
            Get started
          </div>
          <div
            id="login-btn"
            className="ml-6 inline-block px-[1.2em] py-[0.5em] text-[15px] no-underline cursor-pointer select-none font-bold text-white bg-[#9d5df1] border border-[#9d5df1] rounded-sm transition-all duration-300 hover:text-[#9d5df1] hover:bg-black"
            onClick={handleLoginClick}
          >
            Login
          </div>
        </div>
      )}

      {/* モーダルを表示 */}
      <SignupModal isVisible={isSignupVisible} onClose={handleCloseSignup} />
      <LoginModal isVisible={isLoginVisible} onClose={handleCloseLogin} />
    </div>
  );
};

export default AuthenticationForm;
