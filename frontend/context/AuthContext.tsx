"use client";

import {
  DYNAMIC_ERROR_MESSAGES,
  ERROR_MESSAGES,
  PG_ERROR_MESSAGES,
} from "@/constants/message";
import {
  AuthContextType,
  LoginRequest,
  SignupRequest,
  User,
} from "@/types/type";
import axios from "axios";
import {
  FC,
  createContext,
  useContext,
  useState,
  useEffect,
  ReactNode,
} from "react";
import { useToast } from "@/hooks/use-toast";

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const { toast } = useToast();

  const signup = async (name: string, email: string, password: string) => {
    const signupRequest: SignupRequest = { name, email, password };

    try {
      await axios.post("/api/auth/signup", signupRequest);

      // サインアップ成功後の処理
      return true;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        const response = error.response;

        if (response?.status === 409) {
          toast({
            title: "Authentication Failure",
            description:
              "The email you entered is already in use. Please choose a different email.",
            variant: "destructive",
            style: {
              borderColor: "#eb3939",
              backgroundColor: "#eb3939",
              boxShadow: "0 10px 15px rgba(0, 0, 0, 0.3)",
            },
          });

          return false;
        }

        if (response?.status === 400) {
          toast({
            title: "Bad Request Error",
            description: "Validation Error",
            variant: "destructive",
            style: {
              borderColor: "#eb3939",
              backgroundColor: "#eb3939",
              boxShadow: "0 10px 15px rgba(0, 0, 0, 0.3)",
            },
          });

          return false;
        }

        if (response?.status === 500) {
          toast({
            title: "Server Error",
            description:
              "There was a problem with the server. Please try again later.",
            variant: "destructive",
            style: {
              borderColor: "#eb3939",
              backgroundColor: "#eb3939",
              boxShadow: "0 10px 15px rgba(0, 0, 0, 0.3)",
            },
          });

          return false;
        }
      }

      toast({
        title: "Authentication Failure",
        description:
          "A critical issue has occurred on the server, and our development team is currently investigating the cause. We apologize for the inconvenience, but please try again later.",
        variant: "destructive",
        style: {
          borderColor: "#eb3939",
          backgroundColor: "#eb3939",
          boxShadow: "0 10px 15px rgba(0, 0, 0, 0.3)",
        },
      });

      return false;
    }
  };

  const verifyEmail = async (token: string) => {
    try {
      const response = await axios.get(`/api/auth/verify_email?token=${token}`);

      if (response.data) {
        console.log("メールアドレスが確認されました。");
        // ログイン処理または認証後の画面に遷移
        return true;
      } else {
        console.error("メール確認に失敗しました。");
        return false;
      }
    } catch (error) {
      console.error("確認エラー:", error);
      return false;
    }
  };

  const login = async (name: string, email: string, password: string) => {
    const loginRequest: LoginRequest = { name, email, password };

    try {
      const response = await axios.post("/api/auth/login", loginRequest);

      const loginUserData = response.data;
      loginUserData.sub = loginUserData.name;
      loginUserData.id = loginUserData.id;

      window.localStorage.setItem("login_token", loginUserData.token);

      setUser(loginUserData);
      return true;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        const response = error.response;

        if (response?.status === 401) {
          toast({
            title: "Unauthorized Error",
            description: "User not found",
            variant: "destructive",
            style: {
              borderColor: "#eb3939",
              backgroundColor: "#eb3939",
              boxShadow: "0 10px 15px rgba(0, 0, 0, 0.3)",
            },
          });

          return false;
        }

        if (response?.status === 400) {
          toast({
            title: "Bad Request Error",
            description: "Validation Error",
            variant: "destructive",
            style: {
              borderColor: "#eb3939",
              backgroundColor: "#eb3939",
              boxShadow: "0 10px 15px rgba(0, 0, 0, 0.3)",
            },
          });

          return false;
        }

        if (response?.status === 500) {
          toast({
            title: "Server Error",
            description:
              "There was a problem with the server. Please try again later.",
            variant: "destructive",
            style: {
              borderColor: "#eb3939",
              backgroundColor: "#eb3939",
              boxShadow: "0 10px 15px rgba(0, 0, 0, 0.3)",
            },
          });

          return false;
        }
      }

      toast({
        title: "Authentication Failure",
        description:
          "A critical issue has occurred on the server, and our development team is currently investigating the cause. We apologize for the inconvenience, but please try again later.",
        variant: "destructive",
        style: {
          borderColor: "#eb3939",
          backgroundColor: "#eb3939",
          boxShadow: "0 10px 15px rgba(0, 0, 0, 0.3)",
        },
      });

      return false;
    }
  };

  const getCurrentUser = async (): Promise<User | false> => {
    try {
      const token = window.localStorage.getItem("login_token");
      if (token === "" || token === "undefined") {
        console.log(token);
        return false;
      }
      const response = await axios.get("/api/auth/current_user", {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      const currentUserData = response.data;
      currentUserData.token = token;

      return currentUserData;
    } catch (error) {
      return false;
    }
  };

  const guestLogin = async () => {
    // ゲストログイン用データ
    const name = "test_user1";
    const email = "test@gmail.com";
    const password = "Password123";

    try {
      const response = await axios.post("/api/auth/guest_login", {
        name,
        email,
        password,
      });

      const loginUserData = response.data;
      loginUserData.sub = loginUserData.name;
      loginUserData.id = loginUserData.id;

      // ローカルストレージにトークンを保存
      window.localStorage.setItem("login_token", loginUserData.token);

      // ユーザー情報をアプリケーションの状態に設定
      setUser(loginUserData);
      return true;
    } catch (error) {
      console.error(ERROR_MESSAGES.LOGIN_FAILED_MESSAGE);
      return false;
    }
  };

  useEffect(() => {
    const loadUser = async () => {
      try {
        const user = await getCurrentUser();
        if (user) {
          setUser(user);
        } else {
          console.log(ERROR_MESSAGES.LOAD_USER_NOT_FOUND_MESSAGE);
          setUser(null);
        }
      } catch (error: any) {
        console.error(
          DYNAMIC_ERROR_MESSAGES(error).FETCH_CURRENT_USER_FAILED_MESSAGE
        );
        setUser(null);
      } finally {
        setLoading(false);
      }
    };

    loadUser();
  }, [setUser, setLoading]);

  const signOut = async () => {
    try {
      localStorage.removeItem("login_token");
      setUser(null);
      return true;
    } catch (error) {
      return false;
    }
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        setUser,
        loading,
        signup,
        verifyEmail,
        login,
        signOut,
        guestLogin,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error(PG_ERROR_MESSAGES.USEAUTH_REQUIRED_MESSAGE);
  }
  return context;
};
