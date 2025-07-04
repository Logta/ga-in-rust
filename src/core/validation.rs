/// バリデーション関数とエラーメッセージの定義
///
/// 従来のGAErrorを削除し、anyhowを使用したエラーハンドリングに移行
use anyhow::{ensure, Result};
use crate::core::types::*;

/// 個体群サイズの妥当性を検証
pub fn validate_population_size(size: Population) -> Result<()> {
    ensure!(
        size > 0,
        "個体群サイズは1以上である必要があります（現在: {}）",
        size
    );
    Ok(())
}

/// 突然変異率の妥当性を検証
pub fn validate_mutation_rate(rate: MutationRate) -> Result<()> {
    ensure!(
        (0.0..=1.0).contains(&rate),
        "突然変異率は0.0から1.0の間である必要があります（現在: {}）",
        rate
    );
    Ok(())
}

/// DNAの妥当性を検証
pub fn validate_dna(dna: &str) -> Result<()> {
    ensure!(!dna.is_empty(), "DNAは空文字列にできません");
    
    ensure!(
        dna.chars().all(|c| c == '0' || c == '1'),
        "DNAは'0'と'1'の文字のみで構成される必要があります: '{}'",
        dna
    );
    
    Ok(())
}

/// エリートサイズの妥当性を検証
pub fn validate_elite_size(elite_size: usize, population_size: Population) -> Result<()> {
    ensure!(
        elite_size < population_size,
        "エリートサイズ（{}）は個体群サイズ（{}）より小さくなければなりません",
        elite_size,
        population_size
    );
    Ok(())
}

/// 交叉点の妥当性を検証
pub fn validate_crossover_point(point: CrossoverPoint, dna_length: usize) -> Result<()> {
    ensure!(
        point < dna_length,
        "交叉点（{}）はDNA長（{}）より小さくなければなりません",
        point,
        dna_length
    );
    Ok(())
}

/// 世代数の妥当性を検証
pub fn validate_generation_count(count: usize) -> Result<()> {
    ensure!(
        count > 0,
        "世代数は1以上である必要があります（現在: {}）",
        count
    );
    Ok(())
}

/// DNA長の妥当性を検証
pub fn validate_dna_length(length: usize) -> Result<()> {
    ensure!(
        length > 0 && length <= 64,
        "DNA長は1から64の間である必要があります（現在: {}）",
        length
    );
    Ok(())
}

/// 候補数が十分であることを検証
pub fn validate_sufficient_candidates(count: usize, required: usize) -> Result<()> {
    ensure!(
        count >= required,
        "候補数が不足しています。必要: {}, 現在: {}",
        required,
        count
    );
    Ok(())
}

/// ゲーム設定の妥当性を検証
pub fn validate_game_config(rounds_per_match: usize) -> Result<()> {
    ensure!(
        rounds_per_match > 0,
        "対戦ラウンド数は1以上である必要があります（現在: {}）",
        rounds_per_match
    );
    Ok(())
}

/// レポート間隔の妥当性を検証
pub fn validate_report_interval(interval: usize, total_generations: usize) -> Result<()> {
    ensure!(
        interval > 0,
        "レポート間隔は1以上である必要があります（現在: {}）",
        interval
    );
    
    if interval > total_generations {
        anyhow::bail!(
            "レポート間隔（{}）が総世代数（{}）を超えています",
            interval,
            total_generations
        );
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_population_size() {
        assert!(validate_population_size(10).is_ok());
        assert!(validate_population_size(1).is_ok());
        assert!(validate_population_size(0).is_err());
    }

    #[test]
    fn test_validate_mutation_rate() {
        assert!(validate_mutation_rate(0.5).is_ok());
        assert!(validate_mutation_rate(0.0).is_ok());
        assert!(validate_mutation_rate(1.0).is_ok());
        assert!(validate_mutation_rate(-0.1).is_err());
        assert!(validate_mutation_rate(1.1).is_err());
    }

    #[test]
    fn test_validate_dna() {
        assert!(validate_dna("101010").is_ok());
        assert!(validate_dna("000000").is_ok());
        assert!(validate_dna("111111").is_ok());
        assert!(validate_dna("0").is_ok());
        assert!(validate_dna("").is_err());
        assert!(validate_dna("102010").is_err());
        assert!(validate_dna("abcdef").is_err());
    }

    #[test]
    fn test_validate_elite_size() {
        assert!(validate_elite_size(2, 10).is_ok());
        assert!(validate_elite_size(0, 10).is_ok());
        assert!(validate_elite_size(9, 10).is_ok());
        assert!(validate_elite_size(10, 10).is_err());
        assert!(validate_elite_size(15, 10).is_err());
    }

    #[test]
    fn test_validate_crossover_point() {
        assert!(validate_crossover_point(3, 6).is_ok());
        assert!(validate_crossover_point(0, 6).is_ok());
        assert!(validate_crossover_point(5, 6).is_ok());
        assert!(validate_crossover_point(6, 6).is_err());
        assert!(validate_crossover_point(10, 6).is_err());
    }

    #[test]
    fn test_validate_generation_count() {
        assert!(validate_generation_count(1).is_ok());
        assert!(validate_generation_count(1000).is_ok());
        assert!(validate_generation_count(0).is_err());
    }

    #[test]
    fn test_validate_dna_length() {
        assert!(validate_dna_length(1).is_ok());
        assert!(validate_dna_length(32).is_ok());
        assert!(validate_dna_length(64).is_ok());
        assert!(validate_dna_length(0).is_err());
        assert!(validate_dna_length(65).is_err());
    }

    #[test]
    fn test_validate_sufficient_candidates() {
        assert!(validate_sufficient_candidates(10, 5).is_ok());
        assert!(validate_sufficient_candidates(5, 5).is_ok());
        assert!(validate_sufficient_candidates(4, 5).is_err());
    }

    #[test]
    fn test_validate_game_config() {
        assert!(validate_game_config(1).is_ok());
        assert!(validate_game_config(100).is_ok());
        assert!(validate_game_config(0).is_err());
    }

    #[test]
    fn test_validate_report_interval() {
        assert!(validate_report_interval(10, 100).is_ok());
        assert!(validate_report_interval(100, 100).is_ok());
        assert!(validate_report_interval(0, 100).is_err());
        assert!(validate_report_interval(101, 100).is_err());
    }
}