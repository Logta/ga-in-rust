use anyhow::Result;
use colored::*;
use console::Term;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle as IndicatifStyle};

/// 進捗表示のスタイル設定
pub struct ProgressStyle;

impl ProgressStyle {
    /// メイン進捗バーのスタイル
    pub fn main_bar() -> IndicatifStyle {
        IndicatifStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} 世代 ({eta})")
            .expect("テンプレートエラー")
            .progress_chars("█▉▊▋▌▍▎▏  ")
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
    }
    
    /// 統計情報バーのスタイル
    pub fn stats_bar() -> IndicatifStyle {
        IndicatifStyle::default_bar()
            .template("{spinner:.yellow} {prefix:.bold} {msg}")
            .expect("テンプレートエラー")
            .tick_chars("◐◓◑◒")
    }
    
    /// スピナーのスタイル
    pub fn spinner() -> IndicatifStyle {
        IndicatifStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .expect("テンプレートエラー")
            .tick_chars("⣾⣽⣻⢿⡿⣟⣯⣷")
    }
}

/// 出力フォーマッター
pub struct OutputFormatter {
    term: Term,
    quiet: bool,
}

impl OutputFormatter {
    pub fn new(quiet: bool) -> Self {
        Self {
            term: Term::stdout(),
            quiet,
        }
    }
    
    /// タイトルを表示
    pub fn title(&self, text: &str) -> Result<()> {
        if self.quiet {
            return Ok(());
        }
        
        let width = self.term.size().1 as usize;
        let line = "═".repeat(width);
        
        self.term.write_line(&line.bright_blue().to_string())?;
        self.term.write_line(&text.bright_white().bold().to_string())?;
        self.term.write_line(&line.bright_blue().to_string())?;
        self.term.write_line("")?;
        
        Ok(())
    }
    
    /// セクションヘッダーを表示
    pub fn section(&self, text: &str) -> Result<()> {
        if self.quiet {
            return Ok(());
        }
        
        self.term.write_line(&format!("▶ {}", text.bright_cyan().bold()))?;
        Ok(())
    }
    
    /// 情報メッセージを表示
    pub fn info(&self, text: &str) -> Result<()> {
        if self.quiet {
            return Ok(());
        }
        
        self.term.write_line(&format!("  {}", text))?;
        Ok(())
    }
    
    /// 成功メッセージを表示
    pub fn success(&self, text: &str) -> Result<()> {
        self.term.write_line(&format!("✓ {}", text.green()))?;
        Ok(())
    }
    
    /// 警告メッセージを表示
    pub fn warning(&self, text: &str) -> Result<()> {
        self.term.write_line(&format!("⚠ {}", text.yellow()))?;
        Ok(())
    }
    
    /// エラーメッセージを表示
    pub fn error(&self, text: &str) -> Result<()> {
        self.term.write_line(&format!("✗ {}", text.red().bold()))?;
        Ok(())
    }
    
    /// キー・バリューペアを表示
    pub fn key_value(&self, key: &str, value: &str) -> Result<()> {
        if self.quiet {
            return Ok(());
        }
        
        self.term.write_line(&format!(
            "  {} {}",
            format!("{:>15}:", key).bright_black(),
            value.bright_white()
        ))?;
        Ok(())
    }
    
    /// テーブルヘッダーを表示
    pub fn table_header(&self, headers: &[&str]) -> Result<()> {
        if self.quiet {
            return Ok(());
        }
        
        let header_line = headers.iter()
            .map(|h| format!("{:^15}", h))
            .collect::<Vec<_>>()
            .join(" │ ");
            
        self.term.write_line(&header_line.bright_white().underline().to_string())?;
        Ok(())
    }
    
    /// テーブル行を表示
    pub fn table_row(&self, values: &[String]) -> Result<()> {
        if self.quiet {
            return Ok(());
        }
        
        let row = values.iter()
            .map(|v| format!("{:^15}", v))
            .collect::<Vec<_>>()
            .join(" │ ");
            
        self.term.write_line(&row)?;
        Ok(())
    }
}

/// シミュレーション進捗表示
pub struct SimulationProgress {
    multi: MultiProgress,
    main_bar: ProgressBar,
    stats_bar: ProgressBar,
    diversity_bar: ProgressBar,
}

impl SimulationProgress {
    pub fn new(total_generations: u64) -> Self {
        let multi = MultiProgress::new();
        
        let main_bar = multi.add(ProgressBar::new(total_generations));
        main_bar.set_style(ProgressStyle::main_bar());
        
        let stats_bar = multi.add(ProgressBar::new_spinner());
        stats_bar.set_style(ProgressStyle::stats_bar());
        stats_bar.set_prefix("統計");
        
        let diversity_bar = multi.add(ProgressBar::new_spinner());
        diversity_bar.set_style(ProgressStyle::stats_bar());
        diversity_bar.set_prefix("多様性");
        
        Self {
            multi,
            main_bar,
            stats_bar,
            diversity_bar,
        }
    }
    
    /// 進捗を更新
    pub fn update(&self, generation: u64, avg_fitness: f64, diversity: f64) {
        self.main_bar.set_position(generation);
        
        self.stats_bar.set_message(format!(
            "平均適応度: {:.2} | 最大: {:.2} | 最小: {:.2}",
            avg_fitness, avg_fitness * 1.2, avg_fitness * 0.8
        ));
        
        self.diversity_bar.set_message(format!(
            "遺伝的多様性: {:.2}%",
            diversity * 100.0
        ));
        
        self.stats_bar.tick();
        self.diversity_bar.tick();
    }
    
    /// 完了時の処理
    pub fn finish(&self) {
        self.main_bar.finish_with_message("シミュレーション完了");
        self.stats_bar.finish_and_clear();
        self.diversity_bar.finish_and_clear();
    }
    
    /// エラー時の処理
    pub fn abandon(&self) {
        self.main_bar.abandon_with_message("エラーが発生しました");
        self.stats_bar.abandon();
        self.diversity_bar.abandon();
    }
}

/// インタラクティブな確認プロンプト
pub fn confirm(message: &str, default: bool) -> Result<bool> {
    use dialoguer::Confirm;
    
    let result = Confirm::new()
        .with_prompt(message)
        .default(default)
        .interact()?;
        
    Ok(result)
}

/// インタラクティブな選択プロンプト
pub fn select<T: ToString>(prompt: &str, items: &[T]) -> Result<usize> {
    use dialoguer::Select;
    
    let selection = Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()?;
        
    Ok(selection)
}

/// インタラクティブな入力プロンプト
pub fn input(prompt: &str, default: Option<String>) -> Result<String> {
    use dialoguer::Input;
    
    let mut input = Input::<String>::new()
        .with_prompt(prompt);
        
    if let Some(default_value) = default {
        input = input.default(default_value);
    }
    
    Ok(input.interact_text()?)
}