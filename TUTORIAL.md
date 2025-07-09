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

### ステップ5: 四則演算の実装

`calculator_cli/src/main.rs` を編集し、固定値（10と5）で四則演算を行う基本的なプログラムを作成しました。

1.  **ソースコード**:
    ```rust
    fn main() {
        let a = 10;
        let b = 5;

        // 足し算, 引き算, 掛け算, 割り算...
        println!("{} + {} = {}", a, b, a + b);
        println!("{} - {} = {}", a, b, a - b);
        println!("{} * {} = {}", a, b, a * b);
        println!("{} / {} = {}", a, b, a / b);
    }
    ```

2.  **実行と確認**:
    `cargo run` でプログラムを実行し、正しい計算結果が表示されることを確認しました。

    ```bash
    cd calculator_cli
    cargo run
    ```

### ステップ6: 進捗のプッシュ

ここまでの全作業（`GEMINI.md`の更新、`TUTORIAL.md`の作成、`calculator_cli`プロジェクト）をGitHubにコミットし、プッシュしました。

```bash
git add .
git commit -m "feat: Set up Rust project and create tutorial"
git push origin master
```

これで、プロジェクトの初期設定と最初の実装が完了し、その全履歴が記録されました。