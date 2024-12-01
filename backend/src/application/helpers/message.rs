use lazy_static::lazy_static;
use std::collections::HashMap;

// 認証
lazy_static! {
    pub static ref AUTH_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("TOKEN_NOT_FOUND_IN_REQUEST_HEADER_MSG", "リクエストヘッダーにトークンが含まれていません。");

        map
    };
}

// データベース
lazy_static! {
    pub static ref DB_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("DB_CONNECTION_SUCCESS_MSG", "✅ データベース接続に成功しました。");
        map.insert("DB_CONNECTION_FAILURE_MSG", "🔥 データベース接続に失敗しました。");
        map.insert("TRANSACTION_COMMIT_FAILURE_MSG", "トランザクションをコミット中にエラーが発生しました：");
        map.insert("TRANSACTION_ROLLBACK_FAILURE_MSG", "トランザクションをロールバック中にエラーが発生しました：");

        // User information
        map.insert("USER_INFO_NOT_FOUND_MSG", "ユーザー情報が見つかりませんでした。");

        // CRUD operations
        map.insert("FETCH_DATA_SUCCESS_MSG", "データ検索に成功しました。");
        map.insert("CREATE_DATA_SUCCESS_MSG", "データ作成に成功しました。");
        map.insert("UPDATE_DATA_SUCCESS_MSG", "データ更新に成功しました。");
        map.insert("DELETE_DATA_SUCCESS_MSG", "データ削除に成功しました。");

        map
    };
}

// 環境変数
lazy_static! {
    pub static ref SET_ENV_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("NO_SET_ENV_VAR_FRONTEND_PORT", "⚠️ FRONTEND_PORT の環境変数が設定されていません。");
        map.insert("NO_SET_ENV_VAR_DATABASE_PORT", "⚠️ DATABASE_PORT の環境変数が設定されていません。");
        map.insert("NO_SET_ENV_VAR_DATABASE_URL", "⚠️ DATABASE_URL の環境変数が設定されていません。");

        map
    };
}