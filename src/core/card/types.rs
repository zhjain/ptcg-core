//! 核心卡牌类型和枚举

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 卡牌的唯一标识符
pub type CardId = Uuid;

/// 表示PTCG中的不同卡牌类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    /// 可以在场上使用的宝可梦卡
    Pokemon {
        /// 宝可梦种类（例如："皮卡丘"）
        species: String,
        /// 生命值
        hp: u32,
        /// 撤退费用（撤退所需的能量）
        retreat_cost: u32,
        /// 弱点（造成双倍伤害的类型）
        weakness: Option<EnergyType>,
        /// 抗性（造成较少伤害的类型）
        resistance: Option<EnergyType>,
        /// 进化阶段（基础、第一阶段、第二阶段等）
        stage: EvolutionStage,
        /// 前一进化形态（如果适用）
        evolves_from: Option<String>,
    },
    /// 用于发动攻击的能量卡
    Energy {
        /// 能量类型
        energy_type: EnergyType,
        /// 是否为基本能量卡？
        is_basic: bool,
    },
    /// 具有各种效果的训练家卡
    Trainer {
        /// 训练家卡类型
        trainer_type: TrainerType,
    },
}

/// PTCG中的不同能量类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnergyType {
    Grass,      // 草
    Fire,       // 火
    Water,      // 水
    Lightning,  // 电
    Psychic,    // 超能力
    Fighting,   // 格斗
    Darkness,   // 恶
    Metal,      // 钢
    Fairy,      // 妖精
    Dragon,     // 龙
    Colorless,  // 无色
}

/// 宝可梦的进化阶段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionStage {
    Basic,    // 基础
    Stage1,   // 第一阶段
    Stage2,   // 第二阶段
    Mega,     // Mega进化
    GX,       // GX
    EX,       // EX
    V,        // V
    VMax,     // VMAX
}

/// 训练家卡类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainerType {
    Item,      // 道具
    Supporter, // 支援者
    Stadium,   // 体育场
    Tool,      // 工具
}

/// 卡牌稀有度等级
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardRarity {
    Common,      // 普通
    Uncommon,    // 不常见
    Rare,        // 稀有
    RareHolo,    // 稀有闪卡
    UltraRare,   // 超稀有
    SecretRare,  // 秘密稀有
    Promo,       // 推广卡
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_types() {
        let grass = EnergyType::Grass;
        let fire = EnergyType::Fire;
        assert_ne!(grass, fire);
    }

    #[test]
    fn test_evolution_stages() {
        let basic = EvolutionStage::Basic;
        let stage1 = EvolutionStage::Stage1;
        assert_ne!(basic, stage1);
    }

    #[test]
    fn test_card_rarities() {
        let common = CardRarity::Common;
        let rare = CardRarity::Rare;
        assert_ne!(common, rare);
    }
}