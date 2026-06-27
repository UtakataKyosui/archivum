---
name: okf
description: "Open Knowledge Format (OKF) のドキュメント作成・検証・バンドル管理を支援する。以下の場合に使用: (1) OKF準拠のMarkdownドキュメントを新規作成したいとき (2) 既存MarkdownのYAMLフロントマターをOKF仕様に合わせて検証・修正したいとき (3) OKF知識バンドルの index.md / log.md を生成・更新したいとき (4) OKFの仕様や構造について質問があるとき"
argument-hint: "[create|validate|bundle|help] [引数...]"
allowed-tools: Read Bash Write Edit Glob Grep
---

# OKF (Open Knowledge Format) スキル

Google が定義した **Open Knowledge Format v0.1** に準拠したドキュメントの作成・検証・管理を支援します。

OKF の仕様詳細は [spec-reference.md](./spec-reference.md) を参照してください。

## 操作モード

`$ARGUMENTS` の先頭キーワードで動作が変わります。

| キーワード | 説明 |
|-----------|------|
| `create`  | 新規 OKF ドキュメントを作成する |
| `validate`| 指定ファイル/ディレクトリの OKF 準拠を検証する |
| `bundle`  | 知識バンドルの `index.md` / `log.md` を生成・更新する |
| `help`    | OKF の仕様概要を表示する |
| （省略）  | ユーザーの意図を推測して適切な操作を行う |

---

## create — ドキュメント作成

```
/okf create <type> <title> [出力パス]
```

- `<type>`: フロントマター `type` フィールドの値（例: `Concept`, `Playbook`, `BigQuery Table`）
- `<title>`: ドキュメントのタイトル
- `[出力パス]`: 省略時は `<title>.md`（スペースはハイフンに変換）

### 手順

1. テンプレート [templates/concept.md](./templates/concept.md) を読み込む
2. 引数から `type`, `title`, `description` を設定する
3. `timestamp` を現在時刻（ISO 8601）で設定する
4. ユーザーに追加フィールド（`resource`, `tags` など）の要否を確認する
5. 指定パスに書き出す

---

## validate — 準拠検証

```
/okf validate [ファイルパス or ディレクトリ]
```

- 引数省略時はカレントディレクトリの全 `.md` を対象とする

### 検証ルール

各 `.md` ファイルについて以下を確認してください：

1. **YAML フロントマターの存在** — ファイル先頭が `---` で始まり、閉じ `---` があること
2. **`type` フィールドの非空** — フロントマターに `type:` が存在し、値が空でないこと
3. **予約ファイル名の構造** — `index.md` は複数ドキュメントのリスト、`log.md` は変更履歴であること
4. **推奨フィールド** — `title`, `description`, `timestamp` の有無を警告として報告する
5. **リンクの健全性** — 絶対パス（`/`始まり）と相対パスの `*.md` リンク先が存在するか確認する（存在しなくてもエラーではなく警告）

結果は以下の形式でまとめる：

```
✅ PASS  path/to/file.md
⚠️  WARN  path/to/other.md — missing recommended fields: title, description
❌ FAIL  path/to/bad.md   — missing `type` field
```

---

## bundle — バンドル管理

```
/okf bundle [ディレクトリ]
```

- 引数省略時はカレントディレクトリ

### 手順

1. 対象ディレクトリ内の `.md` ファイル（`index.md`, `log.md` を除く）を列挙する
2. 各ファイルのフロントマターから `title`, `type`, `description` を読み取る
3. `index.md` をテンプレート [templates/index.md](./templates/index.md) に基づいて生成・更新する
4. `log.md` が存在しない場合は [templates/log.md](./templates/log.md) を元に初期ファイルを作成する
5. 変更内容を `log.md` に追記する

---

## OKF フロントマター フィールド一覧

| フィールド    | 必須  | 説明 |
|-------------|-------|------|
| `type`      | **必須** | コンセプトカテゴリ（例: `Concept`, `Playbook`, `API`, `Dataset`） |
| `title`     | 推奨  | 人間可読なタイトル |
| `description` | 推奨 | 1文の要約 |
| `resource`  | 推奨  | 対象アセットを識別する URI |
| `tags`      | 推奨  | 分類用文字列リスト |
| `timestamp` | 推奨  | ISO 8601 形式の更新日時 |

カスタムフィールドの追加は許可されています。不明なキーはコンシューマが無視します。

---

## クロスリンク規則

- バンドル内リンクは `/` 始まりの絶対パス（バンドルルート相対）を推奨
- 相対パスも利用可
- リンク先が存在しない「壊れたリンク」はエラーではない（コンシューマは許容する）
