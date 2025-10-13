# ZKP Chaum-Pedersen プロトコル実装

RustとTonicを使用したZero-Knowledge Proof（ゼロ知識証明）のChaum-Pedersenプロトコルの実装です。

## 📋 概要

このプロジェクトは、Chaum-Pedersenプロトコルを使用した認証システムを実装しています。ゼロ知識証明により、秘密情報を明かすことなく、その情報を知っていることを証明できます。

## 🚀 機能

- **Chaum-Pedersenプロトコル**: 離散対数問題に基づくゼロ知識証明の完全実装
- **gRPCサーバー**: Tonicを使用した非同期通信サーバー
- **プロトコルバッファ**: 型安全なメッセージ定義
- **ランダム数生成**: セキュアな暗号学的乱数生成
- **1024ビット定数**: 実用的なセキュリティレベル（RFC 5114準拠）
- **ユーザー管理**: ハッシュマップベースのユーザー情報管理
- **認証フロー**: 登録→チャレンジ→検証の3段階認証プロセス
- **エラーハンドリング**: 適切なエラー処理とログ出力
- **包括的テスト**: 8つのユニットテストによる検証

## 🛠️ 技術スタック

- **Rust**: システムプログラミング言語
- **Tonic**: gRPCフレームワーク
- **Prost**: Protocol Buffers実装
- **Tokio**: 非同期ランタイム
- **num-bigint**: 多倍長整数演算

## 📦 依存関係

```toml
[dependencies]
rand = "0.8"
num-bigint = { version = "0.4", features = ["rand"] }
hex = "0.4.3"
tonic = "0.14.2"
tonic-prost = "0.14.2"
prost = "0.14.1"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

[build-dependencies]
tonic-build = "0.14.2"
tonic-prost-build = "0.14.2"
```

## 🏗️ プロジェクト構造

```
zkp-chaum-pedersen/
├── src/
│   ├── lib.rs          # ZKP実装とテスト（8つのテスト、完全実装）
│   ├── server.rs       # gRPCサーバー（2/3エンドポイント実装済み）
│   ├── client.rs       # gRPCクライアント（基本実装のみ）
│   └── zkp_auth.rs     # 生成されたprotobufコード
├── proto/
│   └── zkp_auth.proto  # Protocol Buffers定義
├── build.rs            # ビルドスクリプト
└── Cargo.toml          # プロジェクト設定
```

## 🔧 セットアップ

### 前提条件

- Rust 1.75以上
- Cargo

### インストール

```bash
git clone <repository-url>
cd zkp-chaum-pedersen
cargo build
```

## 🧪 テスト実行

```bash
cargo test
```

## 🚀 使用方法

### サーバー起動

```bash
cargo run --bin server
```

サーバーが起動すると以下のメッセージが表示されます：
```
🚀 Starting server on 127.0.0.1:50051...
📡 Server is ready to accept connections
```

### クライアント実行

```bash
cargo run --bin client
```

**注意**: 現在のクライアントは基本的なHello World実装のみです。完全なクライアント機能は開発中です。

### サーバー停止

サーバーを停止するには、ターミナルで `Ctrl+C` を押すか、以下のコマンドを実行：

```bash
# プロセス確認
ss -tulpn | grep 50051

# プロセス停止
kill <PID>
```

### gRPCクライアントツール

VS Code拡張機能（grpc-clicker）やgrpcurlを使用してテストできます：

```bash
# grpcurlでのテスト例
echo '{"user":"test","y1":"","y2":""}' | grpcurl -plaintext -d @ 127.0.0.1:50051 zkp_auth.Auth/Register
```

## 📚 Chaum-Pedersenプロトコル

### 概要

Chaum-Pedersenプロトコルは、離散対数問題に基づくゼロ知識証明プロトコルです。

### パラメータ

- **p**: 大きな素数（1024ビット）
- **q**: p-1の素因数
- **g**: 生成元
- **h**: g^α mod p（αは秘密）

### プロトコル手順

1. **登録**: Proverは y1 = g^x mod p, y2 = h^x mod p を送信
2. **チャレンジ**: Proverは r1 = g^k mod p, r2 = h^k mod p を送信
3. **レスポンス**: Verifierはランダムなチャレンジ c を送信
4. **証明**: Proverは s = k - c*x mod q を送信
5. **検証**: Verifierは r1 = g^s * y1^c mod p と r2 = h^s * y2^c mod p を検証

## 🔒 セキュリティ

- **離散対数問題**: 計算困難性に基づくセキュリティ
- **ランダム性**: 各セッションで異なるランダム値を使用
- **ゼロ知識性**: 秘密情報を漏洩しない

## 📖 API仕様

### gRPCサービス

```protobuf
service Auth {
    rpc Register(RegisterRequest) returns (RegisterResponse);
    rpc CreateAuthenticationChallenge(AuthenticationChallengeRequest) returns (AuthenticationChallengeResponse);
    rpc VerifyAuthentication(AuthenticationAnswerRequest) returns (AuthenticationAnswerResponse);
}
```

### メッセージ型

- `RegisterRequest`: ユーザー登録（user, y1, y2）
- `RegisterResponse`: 登録応答
- `AuthenticationChallengeRequest`: 認証チャレンジ要求（user, r1, r2）
- `AuthenticationChallengeResponse`: チャレンジ応答（auth_id, c）
- `AuthenticationAnswerRequest`: 認証応答（auth_id, s）
- `AuthenticationAnswerResponse`: 認証結果（session_id）

### API実装状況

| エンドポイント | 実装状況 | 説明 |
|---|---|---|
| `Register` | ✅ 完了 | ユーザー登録機能（y1, y2の保存） |
| `CreateAuthenticationChallenge` | ✅ 完了 | 認証チャレンジ生成（r1, r2の保存、cの生成） |
| `VerifyAuthentication` | 🚧 開発中 | 認証検証機能（`todo!()`状態） |

## 🏗️ 実装状況

### ✅ 完了済み

- **プロジェクトセットアップ**: Cargo.toml、build.rs、プロトコル定義
- **Tonic統合**: gRPCサーバー/クライアントの基本実装
- **バージョン互換性**: Tonic 0.14.2対応
- **ユーザー管理**: ハッシュマップベースのユーザー情報管理
- **Registerエンドポイント**: ユーザー登録機能の完全実装
- **CreateAuthenticationChallengeエンドポイント**: 認証チャレンジ機能の実装
- **Chaum-Pedersenプロトコル**: ZKPライブラリの完全実装
- **エラーハンドリング**: 適切なエラー処理とログ出力
- **テスト**: 8つのユニットテスト（すべて成功）
- **1024ビット定数**: 実用的なセキュリティレベルの実装

### 🚧 開発中

- **VerifyAuthenticationエンドポイント**: 認証検証機能（`todo!()`状態）
- **gRPCクライアント**: 基本的なクライアント実装（Hello World状態）

### 📋 今後の予定

- **VerifyAuthenticationエンドポイントの完全実装**: 認証検証ロジックの実装
- **gRPCクライアントの実装**: 完全なクライアント機能
- **セッション管理**: 認証後のセッション管理機能
- **セキュリティ強化**: より堅牢なエラーハンドリング
- **パフォーマンス最適化**: 大規模ユーザー対応
- **ドキュメント**: API仕様書の詳細化

## 📄 ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は`LICENSE`ファイルを参照してください。

## 🐛 トラブルシューティング

### よくある問題

#### サーバーが起動しない
```bash
# ポートの使用状況を確認
ss -tulpn | grep 50051

# 既存のプロセスを停止
kill <PID>
```

#### gRPCクライアントツールのエラー
```bash
# grpcurlがインストールされていない場合
wget https://github.com/fullstorydev/grpcurl/releases/download/v1.8.7/grpcurl_1.8.7_linux_x86_64.tar.gz
tar -xzf grpcurl_1.8.7_linux_x86_64.tar.gz
sudo mv grpcurl /usr/local/bin/
```

#### ビルドエラー
```bash
# 依存関係の更新
cargo update

# クリーンビルド
cargo clean
cargo build
```

## 🔗 参考資料

- [Chaum-Pedersen Protocol](https://crypto.stackexchange.com/questions/99262/chaum-pedersen-protocol)
- [Cryptography: An Introduction (3rd Edition)](https://www.cs.umd.edu/~waa/414-F11/IntroToCrypto.pdf)
- [Tonic Documentation](https://github.com/hyperium/tonic)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Protocol Buffers](https://developers.google.com/protocol-buffers)
