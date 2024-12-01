export const ERROR_MESSAGES = {
  LOGIN_FAILED_MESSAGE: "ログインに失敗しました。",
  SIGNUP_FAILED_MESSAGE: "サインアップに失敗しました。",
  LOAD_USER_NOT_FOUND_MESSAGE: "ユーザーはログインしていません。",
  LOAD_USER_FAILED_MESSAGE: "ユーザーの読み込みに失敗しました。",
  INVALID_DATA_FORMAT_NOT_ARRAY_MESSAGE:
    "データ形式が正しくありません。Array型を期待しています。",
  FETCH_MISSION_FAILED_MESSAGE: "ミッションの取得に失敗しました。",
  APPEND_MISSION_FAILED_MESSAGE: "ミッションの追加に失敗しました。",
  UPDATE_MISSION_FAILED_MESSAGE: "ミッションの更新に失敗しました。",
  DELETE_MISSION_FAILED_MESSAGE: "ミッションの削除に失敗しました。",
  PUT_MISSION_STATUS_FAILED_MESSAGE:
    "ミッションのステータスの変更に失敗しました。",
};

export const PG_ERROR_MESSAGES = {
  USEAUTH_REQUIRED_MESSAGE:
    "AuthProvider 内で useAuth を使用する必要があります。",
};

export const DYNAMIC_ERROR_MESSAGES = (message: any) => ({
  FETCH_CURRENT_USER_FAILED_MESSAGE: `現在のユーザーの取得に失敗しました： [${message}]`,
});
