//! 宝可梦卡牌的攻击相关结构和功能

use crate::core::card::EnergyType;
use crate::core::player::SpecialCondition;
use serde::{Deserialize, Serialize};

/// 宝可梦卡牌的攻击信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attack {
    /// 攻击名称
    pub name: String,
    /// 使用此攻击所需的能量费用
    pub cost: Vec<EnergyType>,
    /// 此攻击造成的基准伤害
    pub damage: u32,
    /// 此攻击的特殊效果
    pub effect: Option<String>,
    /// 附加伤害计算模式
    pub damage_mode: Option<DamageMode>,
    /// 此攻击可施加的状态效果
    pub status_effects: Vec<StatusEffect>,
    /// 使用此攻击所需的附加条件
    pub conditions: Vec<String>,
    /// 此攻击的目标选择
    pub target_type: AttackTargetType,
}

/// 不同的伤害计算模式
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageMode {
    /// 基于附加能量数量的伤害
    PerEnergy {
        per_energy: u32,
        energy_type: Option<EnergyType>,
    },
    /// 基于抛硬币结果的伤害
    CoinFlip { per_heads: u32, flips: u32 },
    /// 基于特定位置宝可梦数量的伤害
    PerPokemon { per_pokemon: u32, location: String },
    /// 可变伤害范围
    Variable { min: u32, max: u32 },
}

/// 攻击可施加的状态效果
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusEffect {
    /// 状态条件类型
    pub condition: SpecialCondition,
    /// 概率百分比（0-100）
    pub probability: u32,
    /// 效果目标
    pub target: String,
}

/// 攻击目标类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackTargetType {
    /// 目标为活跃宝可梦
    Active,
    /// 选择对手的任意宝可梦
    Choose,
    /// 对手的所有宝可梦
    All,
    /// 特定的备战区位置
    Bench,
    /// 自身（用于治疗等）
    Self_,
}

impl Attack {
    /// 创建具有固定伤害的简单攻击
    pub fn simple(name: String, cost: Vec<EnergyType>, damage: u32) -> Self {
        Self {
            name,
            cost,
            damage,
            effect: None,
            damage_mode: None,
            status_effects: Vec::new(),
            conditions: Vec::new(),
            target_type: AttackTargetType::Active,
        }
    }

    /// 创建带有状态效果的攻击
    pub fn with_status(
        name: String,
        cost: Vec<EnergyType>,
        damage: u32,
        status: SpecialCondition,
        probability: u32,
    ) -> Self {
        Self {
            name,
            cost,
            damage,
            effect: None,
            damage_mode: None,
            status_effects: vec![StatusEffect {
                condition: status,
                probability,
                target: "defending".to_string(),
            }],
            conditions: Vec::new(),
            target_type: AttackTargetType::Active,
        }
    }

    /// 创建具有抛硬币伤害的攻击
    pub fn coin_flip_damage(
        name: String,
        cost: Vec<EnergyType>,
        base_damage: u32,
        damage_per_heads: u32,
        flips: u32,
    ) -> Self {
        Self {
            name,
            cost,
            damage: base_damage,
            effect: None,
            damage_mode: Some(DamageMode::CoinFlip {
                per_heads: damage_per_heads,
                flips,
            }),
            status_effects: Vec::new(),
            conditions: Vec::new(),
            target_type: AttackTargetType::Active,
        }
    }

    /// 向此攻击添加状态效果
    pub fn add_status_effect(&mut self, effect: StatusEffect) {
        self.status_effects.push(effect);
    }

    /// 向此攻击添加条件
    pub fn add_condition(&mut self, condition: String) {
        self.conditions.push(condition);
    }

    /// 设置此攻击的伤害模式
    pub fn set_damage_mode(&mut self, mode: DamageMode) {
        self.damage_mode = Some(mode);
    }

    /// 设置此攻击的目标类型
    pub fn set_target_type(&mut self, target: AttackTargetType) {
        self.target_type = target;
    }

    /// 计算此攻击将造成的实际伤害
    pub fn calculate_damage(&self, energy_count: u32, coin_results: &[bool]) -> u32 {
        let mut total_damage = self.damage;

        if let Some(ref mode) = self.damage_mode {
            match mode {
                DamageMode::PerEnergy { per_energy, .. } => {
                    total_damage += per_energy * energy_count;
                }
                DamageMode::CoinFlip { per_heads, .. } => {
                    let heads_count = coin_results.iter().filter(|&&result| result).count() as u32;
                    total_damage += per_heads * heads_count;
                }
                DamageMode::PerPokemon { per_pokemon, .. } => {
                    // TODO: 当游戏状态可用时实现
                    total_damage += per_pokemon * 2; // 占位符
                }
                DamageMode::Variable { min, .. } => {
                    total_damage = *min; // 默认为最小值
                }
            }
        }

        total_damage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::player::SpecialCondition;

    #[test]
    fn test_simple_attack() {
        let attack = Attack::simple(
            "Tackle".to_string(),
            vec![EnergyType::Colorless],
            10,
        );
        
        assert_eq!(attack.name, "Tackle");
        assert_eq!(attack.damage, 10);
        assert_eq!(attack.cost.len(), 1);
    }

    #[test]
    fn test_attack_with_status() {
        let attack = Attack::with_status(
            "Thunder Wave".to_string(),
            vec![EnergyType::Lightning],
            20,
            SpecialCondition::Paralyzed,
            100,
        );
        
        assert_eq!(attack.name, "Thunder Wave");
        assert_eq!(attack.status_effects.len(), 1);
        assert_eq!(attack.status_effects[0].condition, SpecialCondition::Paralyzed);
    }

    #[test]
    fn test_calculate_damage() {
        let attack = Attack::coin_flip_damage(
            "Gust".to_string(),
            vec![EnergyType::Colorless],
            10,
            10,
            2,
        );
        
        // 模拟抛硬币结果：两个正面
        let coin_results = vec![true, true];
        let damage = attack.calculate_damage(0, &coin_results);
        
        // 基础伤害10 + 2个正面 * 每个正面10 = 30
        assert_eq!(damage, 30);
    }
}