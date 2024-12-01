// タスク
export type TaskParam = {
  id: number;
  user_id?: number;
  game_id?: number;
  title: string;
  description: string;
  is_completed: boolean;
  status?: number;
  priority?: number;
  difficulty?: number;
  deadline?: string;
  created_at?: string;
  updated_at?: string;
  deleted_at?: string;
};

// コラム　タスクとは1対多の関係
export type ColumnProps = {
  tasks: TaskParam[];
  // onUpdateTask: (updatedTask: TaskParam) => void;
  // onDeleteTask: (taskId: number) => void;
  // onCompleteTask: (taskId: number) => void;
};

// Task
export type TaskProps = {
  task: TaskParam;
  // onUpdateTask: (updatedTask: TaskParam) => void;
  // onDeleteTask: (taskId: number) => void;
  // onCompleteTask: (taskId: number) => void;
};

// 入力フォーム
export type InputProps = {
  onSubmit: (title: string, description: string) => void;
};

// CommonButton
export type CommonButtonProps = {
  children: React.ReactNode;
  onClick: () => void;
};

// EditModal
export type EditModalProps = {
  task: TaskParam;
  isOpen: boolean;
  onClose: () => void;
  onSave: (updatedTask: TaskParam) => void;
};

// LoginModal
export type LoginModalProps = {
  isVisible: boolean;
  onClose: () => void;
};

// SignupModal
export type SignupModalProps = {
  isVisible: boolean;
  onClose: () => void;
};

// AuthContext
export type User = {
  id: number;
  name: string;
  token: string;
  image?: string;
  email?: string;
};

export type SignupRequest = {
  name: string;
  email: string;
  password: string;
};

export type LoginRequest = {
  name: string;
  email: string;
  password: string;
};

export type AuthContextType = {
  user: User | null;
  setUser: React.Dispatch<React.SetStateAction<User | null>>;
  loading: boolean;
  signup: (name: string, email: string, password: string) => Promise<boolean>;
  verifyEmail: (token: string) => Promise<boolean>;
  login: (name: string, email: string, password: string) => Promise<boolean>;
  signOut: () => Promise<boolean>;
  guestLogin: () => Promise<boolean>;
};
