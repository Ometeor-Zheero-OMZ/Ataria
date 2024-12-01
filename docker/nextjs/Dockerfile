# ベースイメージとして Node.js を指定
FROM node:18-alpine

# 作業ディレクトリを作成し、設定
WORKDIR /frontend

# パッケージファイルをコピー
COPY ./frontend/package*.json ./

# 依存関係をインストール
RUN npm install

# 残りのアプリケーションファイルをコピー
COPY ./frontend .

# ビルドプロセスを実行
RUN npm run build

# 静的ファイルを提供するための serve パッケージをインストール
RUN npm install -g serve

# コンテナが起動した際に実行されるコマンドを指定
CMD ["serve", "-s", "out"]
