/// ロギングシステムの設定とユーティリティ
///
/// tracing クレートを使用した構造化ログの設定
use anyhow::{Context, Result};
use tracing::Level;
use tracing_subscriber::{
    fmt::format::FmtSpan,
    EnvFilter,
};
use std::path::Path;

/// ログレベル
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

/// ロギング設定
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// ログレベル
    pub level: LogLevel,
    
    /// JSON形式で出力するか
    pub json_format: bool,
    
    /// スパンイベントを含めるか
    pub include_spans: bool,
    
    /// ファイル出力を有効にするか
    pub file_output: bool,
    
    /// ファイル出力先（file_output = true の場合）
    pub file_path: Option<std::path::PathBuf>,
}

impl LogConfig {
    /// 新しい設定を作成
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            json_format: false,
            include_spans: false,
            file_output: false,
            file_path: None,
        }
    }

    /// 開発用設定
    pub fn development() -> Self {
        Self {
            level: LogLevel::Debug,
            json_format: false,
            include_spans: true,
            file_output: false,
            file_path: None,
        }
    }

    /// 本番用設定
    pub fn production() -> Self {
        Self {
            level: LogLevel::Info,
            json_format: true,
            include_spans: false,
            file_output: true,
            file_path: Some("logs/app.log".into()),
        }
    }

    /// JSON形式を有効にする
    pub fn with_json(mut self) -> Self {
        self.json_format = true;
        self
    }

    /// スパンイベントを有効にする
    pub fn with_spans(mut self) -> Self {
        self.include_spans = true;
        self
    }

    /// ファイル出力を設定
    pub fn with_file_output<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.file_output = true;
        self.file_path = Some(path.as_ref().to_path_buf());
        self
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self::new(LogLevel::default())
    }
}

/// ロギングシステムを初期化
pub fn init_logging(config: &LogConfig) -> Result<()> {
    // 環境変数またはConfigからフィルターレベルを設定
    let env_filter = {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| {
                EnvFilter::new(format!("{}={}", 
                    env!("CARGO_PKG_NAME").replace('-', "_"),
                    match config.level {
                        LogLevel::Trace => "trace",
                        LogLevel::Debug => "debug", 
                        LogLevel::Info => "info",
                        LogLevel::Warn => "warn",
                        LogLevel::Error => "error",
                    }
                ))
            })
    };

    // 簡略化されたロギング設定
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(env_filter)
        .with_span_events(if config.include_spans {
            FmtSpan::NEW | FmtSpan::CLOSE
        } else {
            FmtSpan::NONE
        });

    if config.json_format {
        subscriber.json().try_init()
            .map_err(|e| anyhow::anyhow!("ロギングシステムの初期化に失敗しました: {}", e))?;
    } else {
        subscriber.try_init()
            .map_err(|e| anyhow::anyhow!("ロギングシステムの初期化に失敗しました: {}", e))?;
    }

    tracing::info!(
        "ロギングシステムが初期化されました: レベル={:?}, JSON形式={}",
        config.level,
        config.json_format
    );

    Ok(())
}

/// ロギングマクロのユーティリティ
pub mod utils {
    /// エラー情報を詳細に出力
    pub fn log_error(error: &anyhow::Error) {
        tracing::error!("エラーが発生しました: {}", error);
        
        let mut source = error.source();
        let mut level = 1;
        
        while let Some(err) = source {
            tracing::error!("  {}: {}", level, err);
            source = err.source();
            level += 1;
        }
    }

    /// パフォーマンス測定のマクロ
    #[macro_export]
    macro_rules! measure_time {
        ($name:expr, $block:block) => {{
            let start = std::time::Instant::now();
            tracing::debug!("開始: {}", $name);
            
            let result = $block;
            
            let elapsed = start.elapsed();
            tracing::info!("完了: {} ({:.2}ms)", $name, elapsed.as_millis());
            
            result
        }};
    }
}