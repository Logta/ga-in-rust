/// メルセンヌツイスタ乱数生成器のラッパー
///
/// randクレートのStdRngを使用してメルセンヌツイスタを実装
/// 遺伝的アルゴリズムで大量の乱数が必要になるため、高速な乱数生成器を採用
use anyhow::Result;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::sync::Mutex;

/// グローバル乱数生成器のラッパー
pub struct RandomGenerator {
    rng: Mutex<StdRng>,
}

impl RandomGenerator {
    /// シードを指定して新しい乱数生成器を作成
    pub fn new(seed: Option<u32>) -> Self {
        let actual_seed = seed.unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32
        });
        
        Self {
            rng: Mutex::new(StdRng::seed_from_u64(actual_seed as u64)),
        }
    }

    /// 0.0以上1.0未満の浮動小数点数を生成
    pub fn gen_f64(&self) -> Result<f64> {
        let mut rng = self.rng.lock()
            .map_err(|_| anyhow::anyhow!("乱数生成器のロック取得に失敗しました"))?;
        Ok(rng.random::<f64>())
    }

    /// 0以上max未満の整数を生成
    pub fn gen_range(&self, max: usize) -> Result<usize> {
        if max == 0 {
            anyhow::bail!("範囲の最大値は0より大きくなければなりません");
        }
        
        let mut rng = self.rng.lock()
            .map_err(|_| anyhow::anyhow!("乱数生成器のロック取得に失敗しました"))?;
        Ok(rng.random_range(0..max))
    }

    /// 指定された確率でtrueを返す
    pub fn gen_bool(&self, probability: f64) -> Result<bool> {
        if !(0.0..=1.0).contains(&probability) {
            anyhow::bail!("確率は0.0から1.0の間である必要があります: {}", probability);
        }
        
        let mut rng = self.rng.lock()
            .map_err(|_| anyhow::anyhow!("乱数生成器のロック取得に失敗しました"))?;
        Ok(rng.random_bool(probability))
    }

    /// 配列からランダムに要素を選択
    pub fn choose<'a, T>(&self, items: &'a [T]) -> Result<&'a T> {
        if items.is_empty() {
            anyhow::bail!("空の配列から要素を選択することはできません");
        }
        
        let index = self.gen_range(items.len())?;
        Ok(&items[index])
    }

    /// 配列をシャッフル（Fisher-Yates アルゴリズム）
    pub fn shuffle<T>(&self, items: &mut [T]) -> Result<()> {
        let mut rng = self.rng.lock()
            .map_err(|_| anyhow::anyhow!("乱数生成器のロック取得に失敗しました"))?;
        items.shuffle(&mut *rng);
        Ok(())
    }

    /// 重み付きランダム選択（ルーレット選択）
    pub fn weighted_choice<'a, T>(&self, items: &'a [(T, f64)]) -> Result<&'a T> {
        if items.is_empty() {
            anyhow::bail!("空の配列から重み付き選択はできません");
        }

        let total_weight: f64 = items.iter().map(|(_, weight)| weight).sum();
        if total_weight <= 0.0 {
            anyhow::bail!("重みの合計は0より大きくなければなりません: {}", total_weight);
        }

        let mut target = self.gen_f64()? * total_weight;
        
        for (item, weight) in items {
            target -= weight;
            if target <= 0.0 {
                return Ok(item);
            }
        }
        
        // 浮動小数点の誤差により、最後の要素を返す
        Ok(&items.last().unwrap().0)
    }

    /// 正規分布に従う乱数を生成（Box-Muller変換）
    pub fn gen_normal(&self, mean: f64, std_dev: f64) -> Result<f64> {
        let u1 = self.gen_f64()?;
        let u2 = self.gen_f64()?;
        
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        Ok(mean + std_dev * z)
    }

    /// 指数分布に従う乱数を生成
    pub fn gen_exponential(&self, lambda: f64) -> Result<f64> {
        if lambda <= 0.0 {
            anyhow::bail!("λ（ラムダ）は0より大きくなければなりません: {}", lambda);
        }
        
        let u = self.gen_f64()?;
        Ok(-u.ln() / lambda)
    }

    /// ポアソン分布に従う乱数を生成（Knuthのアルゴリズム）
    pub fn gen_poisson(&self, lambda: f64) -> Result<u32> {
        if lambda <= 0.0 {
            anyhow::bail!("λ（ラムダ）は0より大きくなければなりません: {}", lambda);
        }
        
        let l = (-lambda).exp();
        let mut k = 0;
        let mut p = 1.0;
        
        loop {
            k += 1;
            p *= self.gen_f64()?;
            if p <= l {
                break;
            }
        }
        
        Ok(k - 1)
    }
}

/// デフォルトの乱数生成器
static DEFAULT_RNG: std::sync::OnceLock<RandomGenerator> = std::sync::OnceLock::new();

/// デフォルトの乱数生成器を取得
pub fn default_rng() -> &'static RandomGenerator {
    DEFAULT_RNG.get_or_init(|| RandomGenerator::new(None))
}

/// シード付きでデフォルト乱数生成器を初期化
pub fn init_default_rng(seed: u32) {
    let _ = DEFAULT_RNG.set(RandomGenerator::new(Some(seed)));
}

/// 便利関数群
pub mod utils {
    use super::*;

    /// 0.0以上1.0未満の浮動小数点数を生成
    pub fn random() -> Result<f64> {
        default_rng().gen_f64()
    }

    /// 0以上max未満の整数を生成
    pub fn random_range(max: usize) -> Result<usize> {
        default_rng().gen_range(max)
    }

    /// 指定された確率でtrueを返す
    pub fn random_bool(probability: f64) -> Result<bool> {
        default_rng().gen_bool(probability)
    }

    /// 配列からランダムに要素を選択
    pub fn choose<'a, T>(items: &'a [T]) -> Result<&'a T> {
        default_rng().choose(items)
    }

    /// 配列をシャッフル
    pub fn shuffle<T>(items: &mut [T]) -> Result<()> {
        default_rng().shuffle(items)
    }

    /// 重み付きランダム選択
    pub fn weighted_choice<'a, T>(items: &'a [(T, f64)]) -> Result<&'a T> {
        default_rng().weighted_choice(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_generator_creation() {
        let rng = RandomGenerator::new(Some(42));
        assert!(rng.gen_f64().is_ok());
    }

    #[test]
    fn test_gen_range() {
        let rng = RandomGenerator::new(Some(42));
        
        for _ in 0..100 {
            let val = rng.gen_range(10).unwrap();
            assert!(val < 10);
        }
        
        assert!(rng.gen_range(0).is_err());
    }

    #[test]
    fn test_gen_bool() {
        let rng = RandomGenerator::new(Some(42));
        
        // 確率0では常にfalse
        for _ in 0..10 {
            assert!(!rng.gen_bool(0.0).unwrap());
        }
        
        // 確率1では常にtrue
        for _ in 0..10 {
            assert!(rng.gen_bool(1.0).unwrap());
        }
        
        // 不正な確率はエラー
        assert!(rng.gen_bool(-0.1).is_err());
        assert!(rng.gen_bool(1.1).is_err());
    }

    #[test]
    fn test_choose() {
        let rng = RandomGenerator::new(Some(42));
        let items = [1, 2, 3, 4, 5];
        
        for _ in 0..100 {
            let chosen = rng.choose(&items).unwrap();
            assert!(items.contains(chosen));
        }
        
        let empty: &[i32] = &[];
        assert!(rng.choose(empty).is_err());
    }

    #[test]
    fn test_shuffle() {
        let rng = RandomGenerator::new(Some(42));
        let mut items = vec![1, 2, 3, 4, 5];
        let original = items.clone();
        
        rng.shuffle(&mut items).unwrap();
        
        // 要素は同じだが順序が変わる可能性がある
        items.sort();
        assert_eq!(items, original);
    }

    #[test]
    fn test_weighted_choice() {
        let rng = RandomGenerator::new(Some(42));
        let items = [("A", 0.1), ("B", 0.9)];
        
        // Bが選ばれる確率が高いはず
        let mut count_b = 0;
        for _ in 0..1000 {
            if rng.weighted_choice(&items).unwrap() == &"B" {
                count_b += 1;
            }
        }
        
        // 統計的にBが多く選ばれるはず（厳密な値は避けて範囲でテスト）
        assert!(count_b > 800);
        
        let empty: &[(&str, f64)] = &[];
        assert!(rng.weighted_choice(empty).is_err());
        
        let zero_weight = [("A", 0.0), ("B", 0.0)];
        assert!(rng.weighted_choice(&zero_weight).is_err());
    }

    #[test]
    fn test_distributions() {
        let rng = RandomGenerator::new(Some(42));
        
        // 正規分布
        let normal = rng.gen_normal(0.0, 1.0).unwrap();
        assert!(normal.is_finite());
        
        // 指数分布
        let exp = rng.gen_exponential(1.0).unwrap();
        assert!(exp >= 0.0 && exp.is_finite());
        assert!(rng.gen_exponential(-1.0).is_err());
        
        // ポアソン分布
        let poisson = rng.gen_poisson(2.0).unwrap();
        assert!(poisson < 1000); // 常識的な範囲
        assert!(rng.gen_poisson(-1.0).is_err());
    }

    #[test]
    fn test_utils() {
        init_default_rng(123);
        
        assert!(utils::random().is_ok());
        assert!(utils::random_range(10).is_ok());
        assert!(utils::random_bool(0.5).is_ok());
        
        let items = [1, 2, 3];
        assert!(utils::choose(&items).is_ok());
        
        let mut items = vec![1, 2, 3, 4, 5];
        assert!(utils::shuffle(&mut items).is_ok());
        
        let weighted = [("A", 0.3), ("B", 0.7)];
        assert!(utils::weighted_choice(&weighted).is_ok());
    }
}