/// プロジェクト初期化コマンドの実装
use anyhow::Result;
use std::path::Path;
use crate::config::{Config, ConfigLoader};

/// プロジェクトを初期化
pub async fn execute_init(path: &Path, force: bool) -> Result<()> {
    if path.exists() && !force {
        anyhow::bail!(
            "ディレクトリが既に存在します: {}\n--force オプションで上書きできます",
            path.display()
        );
    }
    
    // ディレクトリを作成
    std::fs::create_dir_all(path)?;
    
    // デフォルト設定ファイルを作成
    let config_path = path.join("config.toml");
    let default_config = Config::default();
    ConfigLoader::save(&default_config, &config_path)?;
    
    // README.mdを作成
    let readme_path = path.join("README.md");
    let readme_content = create_readme_content();
    std::fs::write(&readme_path, readme_content)?;
    
    // examples ディレクトリを作成
    let examples_dir = path.join("examples");
    std::fs::create_dir_all(&examples_dir)?;
    
    // 基本的な例を作成
    let example_config = path.join("examples").join("basic.toml");
    let mut example = Config::default();
    example.genetic.population_size = 50;
    example.genetic.generations = 100;
    ConfigLoader::save(&example, &example_config)?;
    
    println!("プロジェクトを初期化しました: {}", path.display());
    println!("  - {}", config_path.display());
    println!("  - {}", readme_path.display());
    println!("  - {}", example_config.display());
    
    Ok(())
}

fn create_readme_content() -> String {
    r#"# GA Prisoner's Dilemma Project

遺伝的アルゴリズムを用いた囚人のジレンマ戦略の進化シミュレーションプロジェクトです。

## 使用方法

### 基本的な実行

```bash
ga-sim run
```

### 設定ファイルを使用した実行

```bash
ga-sim run --config config.toml
```

### カスタムパラメータでの実行

```bash
ga-sim run --generations 1000 --population 100 --mutation-rate 0.02
```

## 設定ファイル

`config.toml` に各種パラメータを設定できます。

例については `examples/` ディレクトリを参照してください。

## 出力

シミュレーション結果は JSON 形式で出力され、以下の情報が含まれます：

- 各世代の統計情報
- 最良個体の情報
- 収束分析結果
- パフォーマンス情報

詳細な使用方法については、プロジェクトのドキュメントを参照してください。
"#.to_string()
}