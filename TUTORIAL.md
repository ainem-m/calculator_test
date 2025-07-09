# Rust製CLI電卓 開発チュートリアル（完全版）

このドキュメントは、シンプルなコマンドライン電卓をRustでゼロから作成するためのステップバイステップガイドです。プロジェクトの初期設定から、コーディング、GitHubでの管理まで、すべての手順を網羅しています。

## Part 1: プロジェクトの準備と公開

まず、開発の土台となるプロジェクトのセットアップと、バージョン管理のためのGitHubリポジトリの準備を行います。

### ステップ1: プロジェクト方針の決定

AIアシスタント（Gemini）と協力して開発を進めるため、最初の方針を `GEMINI.md` として定義しました。これにより、AIはプロジェクトの目標と手順を正確に理解できます。

<details>
<summary>GEMINI.mdの内容</summary>

```markdown
# Project: CLI Calculator in Rust

This document outlines the step-by-step plan for building a command-line interface (CLI) calculator application using Rust. The goal is to create a simple, yet robust, calculator that can perform basic arithmetic operations.

This plan is designed to be executed by an AI assistant.

## Development Plan

### 1. Project Setup
- **Action:** Initialize a new Rust project using Cargo.
- **Command:** `cargo new calculator_cli`
- **Verification:** Confirm that `Cargo.toml` and `src/main.rs` are created successfully.

### 2. Implement Addition
- **File:** `src/main.rs`
- **Action:** Write a simple function to add two numbers. For now, use hardcoded values.
- **Goal:** Verify the basic program structure and compilation.
- **Verification:** Run `cargo run` and check that the correct sum is printed to the console.

... (以降のステップも同様) ...
```
</details>

### ステップ2: GitHubリポジトリの作成と公開

次に、ソースコードを管理するためにGitHub上にリポジトリを作成し、ローカルのプロジェクトを公開しました。

1.  **README.mdの作成**:
    プロジェクトの顔となる簡単な説明フ���イルを作成しました。

    ```bash
    # CLI電卓
    コマンドラインで動作するシンプルな電卓です。
    ```

2.  **ローカルリポジトリの初期化**:
    `git`コマンドでローカルリポジトリを準備しました。

    ```bash
    git init
    git add .
    git commit -m "Initial commit"
    ```

3.  **GitHubリポジトリの作成とプッシュ**:
    GitHub CLI (`gh`) を使って、`calculator_test` という名前の公開リポジトリを作成し、ローカルの変更をプッシュしました。

    ```bash
    gh repo create calculator_test --public --source=. --remote=origin
    git push -u origin master
    ```

## Part 2: 開発の実装

ここからは、実際のコーディング作業です。

### ステップ3: 開発環境のセットアップ (Rust)

1.  **インストーラーのダウンロードと実行**:
    `rustup` を使ってRustのツールチェーン（コンパイラやCargoなど）をインストールしました。

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh
    sh rustup-init.sh -y
    ```

2.  **環境変数の設定**:
    `cargo`コマンドを使えるようにパスを通しました。

    ```bash
    source "$HOME/.cargo/env"
    ```

### ステップ4: Rustプロジェクトの作成

`cargo` を使って、電卓アプリケーションのプロジェクト `calculator_cli` を作成しました。

```bash
cargo new calculator_cli
```

### ステップ5: 四則演算の実装 (完了)

`calculator_cli/src/lib.rs` に四則演算を行う関数を実装しました。初期の固定値での実装から、引数を受け取る形に進化しました。

1.  **ソースコードの概要**:
    `src/lib.rs` に `calculate` 関数を定義し、加算、減算、乗算、除算のロジックをカプセル化しました。

2.  **実行と確認**:
    `cargo run` でプログラムを実行し、正しい計算結果が表示されることを確認しました。

### ステップ6: コマンドライン引数のパース (完了)

`clap` クレートを導入し、コマンドライン引数から数値と演算子を受け取れるようにしました。

1.  **`Cargo.toml` の更新**:
    `clap = { version = "4.5.40", features = ["derive"] }` を追加しました。

2.  **`src/main.rs` の実装**:
    `Args` 構造体を定義し、`clap::Parser` を使って引数をパースし、`calculate` 関数に渡すようにしました。

3.  **実行例**:
    `cargo run -- 10 + 5` のように実行し、結果を確認しました。

### ステップ7: リファクタリングとエラーハンドリング (完了)

コードをより論理的なモジュールに整理し、無効な入力（非数値、ゼロ除算、無効な演算子）に対する堅牢なエラーハンドリングを実装しました。

1.  **モジュール化**:
    計算ロジックを `src/lib.rs` に移動し、`src/main.rs` から呼び出すようにしました。

2.  **エラーハンドリング**:
    `calculate` 関数が `Result<f64, String>` を返すように変更し、エラーメッセージをユーザーフレンドリーな形式で表示するようにしました。

3.  **実行例**:
    `cargo run -- 10 x 5` や `cargo run -- 10 / 0` のような無効な入力で、適切なエラーメッセージが表示されることを確認しました。

### ステップ8: テストの追加 (完了)

計算ロジックの正確性を保証するために、`src/lib.rs` にユニットテストを追加しました。

1.  **テスト��ード**:
    `#[cfg(test)]` モジュール内に、加算、減算、乗算、除算、ゼロ除算、無効な演算子などのケースをカバーするテスト関数を記述しました。

2.  **テストの実行**:
    `cargo test` コマンドを実行し、すべてのテストがパスすることを確認しました。

### ステップ9: 進捗のプッシュ (完了)

ここまでの全作業（`GEMINI.md`と`TUTORIAL.md`の更新、`calculator_cli`プロジェクトの全実装）をGitHubにコミットし、プッシュしました。

```bash
git add .
git commit -m "feat: Complete CLI calculator implementation and update documentation"
git push origin master
```

これで、CLI電卓アプリケーションの全開発が完了し、その全履歴がGitHubに記録されました.