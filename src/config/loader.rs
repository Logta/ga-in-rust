use anyhow::{Context, Result, bail};
use dirs;
use std::path::{Path, PathBuf};
use crate::config::Config;

/// 設定ローダー
pub struct ConfigLoader {
    search_paths: Vec<PathBuf>,
}

impl ConfigLoader {
    /// 新しいローダーを作成
    pub fn new() -> Self {
        let mut search_paths = Vec::new();
        
        // 現在のディレクトリ
        search_paths.push(PathBuf::from("."));
        
        // ユーザー設定ディレクトリ
        if let Some(config_dir) = dirs::config_dir() {
            search_paths.push(config_dir.join("ga-prisoners-dilemma"));
        }
        
        // ホームディレクトリ
        if let Some(home_dir) = dirs::home_dir() {
            search_paths.push(home_dir.join(".ga-prisoners-dilemma"));
        }
        
        Self { search_paths }
    }
    
    /// カスタム検索パスを追加
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.insert(0, path);
    }
    
    /// 設定を読み込む
    pub fn load(&self, explicit_path: Option<&Path>) -> Result<Config> {
        // 明示的なパスが指定されている場合
        if let Some(path) = explicit_path {
            return Config::from_file(path)
                .with_context(|| format!("指定された設定ファイルの読み込みに失敗: {}", path.display()));
        }
        
        // 検索パスから設定ファイルを探す
        let config_names = [
            "ga-prisoners-dilemma.toml",
            "ga-prisoners-dilemma.yaml",
            "ga-prisoners-dilemma.yml",
            "ga-prisoners-dilemma.json",
            "config.toml",
            "config.yaml",
            "config.yml",
            "config.json",
        ];
        
        for dir in &self.search_paths {
            for name in &config_names {
                let path = dir.join(name);
                if path.exists() {
                    tracing::info!("設定ファイルを発見: {}", path.display());
                    return Config::from_file(&path)
                        .with_context(|| format!("設定ファイルの読み込みに失敗: {}", path.display()));
                }
            }
        }
        
        // 設定ファイルが見つからない場合はデフォルトを使用
        tracing::info!("設定ファイルが見つかりません。デフォルト設定を使用します");
        Ok(Config::default())
    }
    
    /// 設定ファイルのパスを取得
    pub fn find_config_path(&self) -> Option<PathBuf> {
        let config_names = [
            "ga-prisoners-dilemma.toml",
            "ga-prisoners-dilemma.yaml",
            "ga-prisoners-dilemma.yml",
            "ga-prisoners-dilemma.json",
            "config.toml",
            "config.yaml",
            "config.yml",
            "config.json",
        ];
        
        for dir in &self.search_paths {
            for name in &config_names {
                let path = dir.join(name);
                if path.exists() {
                    return Some(path);
                }
            }
        }
        
        None
    }
    
    /// デフォルトの設定パスを取得
    pub fn default_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("設定ディレクトリの取得に失敗しました")?;
        
        Ok(config_dir.join("ga-prisoners-dilemma").join("config.toml"))
    }
    
    /// 設定ファイルを保存
    pub fn save(config: &Config, path: &Path) -> Result<()> {
        // ディレクトリが存在しない場合は作成
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("ディレクトリの作成に失敗: {}", parent.display()))?;
        }
        
        // 拡張子に基づいて適切な形式で保存
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("toml");
        
        let content = match extension {
            "toml" => toml::to_string_pretty(config)
                .context("TOML形式への変換に失敗")?,
            "yaml" | "yml" => serde_yaml::to_string(config)
                .context("YAML形式への変換に失敗")?,
            "json" => serde_json::to_string_pretty(config)
                .context("JSON形式への変換に失敗")?,
            _ => bail!("サポートされていないファイル形式: {}", extension),
        };
        
        std::fs::write(path, content)
            .with_context(|| format!("設定ファイルの書き込みに失敗: {}", path.display()))?;
        
        Ok(())
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// 設定のマージ
pub fn merge_configs(_base: Config, overrides: Config) -> Config {
    // TODO: より洗練されたマージロジックを実装
    // 現在は単純に上書き
    overrides
}

/// 環境変数から設定を読み込む
pub fn load_from_env() -> Result<Config> {
    use std::env;
    
    let mut config = Config::default();
    
    // GA_PD_ プレフィックスを持つ環境変数を読み取る
    for (key, value) in env::vars() {
        if !key.starts_with("GA_PD_") {
            continue;
        }
        
        let key = key.trim_start_matches("GA_PD_").to_lowercase();
        match key.as_str() {
            "generations" => {
                config.genetic.generations = value.parse()
                    .with_context(|| format!("GA_PD_GENERATIONS の値が不正: {}", value))?;
            }
            "population" => {
                config.genetic.population_size = value.parse()
                    .with_context(|| format!("GA_PD_POPULATION の値が不正: {}", value))?;
            }
            "mutation_rate" => {
                config.genetic.mutation_rate = value.parse()
                    .with_context(|| format!("GA_PD_MUTATION_RATE の値が不正: {}", value))?;
            }
            "strategy" => {
                config.simulation.default_strategy = value;
            }
            _ => {
                tracing::warn!("未知の環境変数: GA_PD_{}", key.to_uppercase());
            }
        }
    }
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_loader_new() {
        let loader = ConfigLoader::new();
        assert!(!loader.search_paths.is_empty());
    }
    
    #[test]
    fn test_default_config_path() {
        let path = ConfigLoader::default_config_path();
        assert!(path.is_ok());
    }
}