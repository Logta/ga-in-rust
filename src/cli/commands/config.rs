/// 設定コマンドの実装
use anyhow::Result;
use crate::cli::app::{ConfigAction, ConfigFormat};
use crate::config::{Config, ConfigLoader};
use std::path::Path;

/// 設定コマンドを実行
pub async fn execute_config(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Show { key } => show_config(key.as_deref()).await,
        ConfigAction::Validate => validate_config().await,
        ConfigAction::Init { path, format, force } => {
            init_config(path.as_deref(), format, force).await
        }
        ConfigAction::Path => show_config_path().await,
        ConfigAction::Set { key, value } => set_config(&key, &value).await,
        ConfigAction::Reset { yes } => reset_config(yes).await,
    }
}

/// 現在の設定を表示
async fn show_config(key: Option<&str>) -> Result<()> {
    let loader = ConfigLoader::new();
    let config = loader.load(None)?;
    
    let toml_string = toml::to_string_pretty(&config)?;
    
    if let Some(key) = key {
        println!("設定キー '{}' の値:", key);
        // TODO: 特定のキーの値を表示する実装
        println!("指定されたキーの値表示は未実装です");
    } else {
        println!("現在の設定:\n{}", toml_string);
    }
    
    Ok(())
}

/// 設定の妥当性を検証
async fn validate_config() -> Result<()> {
    let loader = ConfigLoader::new();
    let config = loader.load(None)?;
    
    config.validate()?;
    println!("設定は有効です");
    
    Ok(())
}

/// 新しい設定ファイルを初期化
async fn init_config(
    path: Option<&Path>,
    format: ConfigFormat,
    force: bool
) -> Result<()> {
    let target_path = if let Some(p) = path {
        p.to_path_buf()
    } else {
        ConfigLoader::default_config_path()?
    };
    
    if target_path.exists() && !force {
        anyhow::bail!(
            "設定ファイルが既に存在します: {}\n--force オプションで上書きできます",
            target_path.display()
        );
    }
    
    let config = Config::default();
    
    let final_path = match format {
        ConfigFormat::Toml => target_path.with_extension("toml"),
        ConfigFormat::Yaml => target_path.with_extension("yaml"),
        ConfigFormat::Json => target_path.with_extension("json"),
    };
    
    ConfigLoader::save(&config, &final_path)?;
    
    println!("設定ファイルを作成しました: {}", final_path.display());
    
    Ok(())
}

/// 設定ファイルのパスを表示
async fn show_config_path() -> Result<()> {
    let loader = ConfigLoader::new();
    if let Some(path) = loader.find_config_path() {
        println!("設定ファイル: {}", path.display());
    } else {
        println!("設定ファイルが見つかりません");
        let default_path = ConfigLoader::default_config_path()?;
        println!("デフォルトパス: {}", default_path.display());
    }
    Ok(())
}

/// 設定値を変更
async fn set_config(_key: &str, _value: &str) -> Result<()> {
    println!("設定値の変更は未実装です");
    Ok(())
}

/// 設定をリセット
async fn reset_config(_yes: bool) -> Result<()> {
    println!("設定のリセットは未実装です");
    Ok(())
}