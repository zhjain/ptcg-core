//! 宝可梦卡牌特定功能

use crate::core::card::{Attack, Ability, CardId, CardType, CardRarity, EnergyType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 主卡牌结构
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    /// 此卡牌的唯一标识符
    pub id: CardId,
    /// 卡牌名称
    pub name: String,
    /// 卡牌类型（宝可梦、能量、训练家）
    pub card_type: CardType,
    /// 所属卡包信息
    pub set_name: String,
    /// 在卡包中的编号
    pub set_number: String,
    /// 卡牌稀有度
    pub rarity: CardRarity,
    /// 攻击（针对宝可梦卡）
    pub attacks: Vec<Attack>,
    /// 能力（针对宝可梦卡）
    pub abilities: Vec<Ability>,
    /// 卡牌规则文本
    pub rules: Vec<String>,
    /// 附加元数据
    pub metadata: HashMap<String, String>,
}

impl Card {
    /// 使用给定参数创建新卡牌
    pub fn new(
        name: String,
        card_type: CardType,
        set_name: String,
        set_number: String,
        rarity: CardRarity,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            card_type,
            set_name,
            set_number,
            rarity,
            attacks: Vec::new(),
            abilities: Vec::new(),
            rules: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// 检查是否为宝可梦卡
    pub fn is_pokemon(&self) -> bool {
        matches!(self.card_type, CardType::Pokemon { .. })
    }

    /// 检查是否为能量卡
    pub fn is_energy(&self) -> bool {
        matches!(self.card_type, CardType::Energy { .. })
    }

    /// 检查是否为训练家卡
    pub fn is_trainer(&self) -> bool {
        matches!(self.card_type, CardType::Trainer { .. })
    }

    /// 获取宝可梦卡的生命值（非宝可梦卡返回None）
    pub fn get_hp(&self) -> Option<u32> {
        match &self.card_type {
            CardType::Pokemon { hp, .. } => Some(*hp),
            _ => None,
        }
    }

    /// 获取能量卡的能量类型
    pub fn get_energy_type(&self) -> Option<&EnergyType> {
        match &self.card_type {
            CardType::Energy { energy_type, .. } => Some(energy_type),
            _ => None,
        }
    }

    /// 向宝可梦卡添加攻击
    pub fn add_attack(&mut self, attack: Attack) {
        if self.is_pokemon() {
            self.attacks.push(attack);
        }
    }

    /// 向宝可梦卡添加能力
    pub fn add_ability(&mut self, ability: Ability) {
        if self.is_pokemon() {
            self.abilities.push(ability);
        }
    }

    /// 向卡牌添加规则
    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    /// 向卡牌添加元数据
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// 计算能量类型计数
    fn count_energy_types(
        energy_list: &[EnergyType],
    ) -> std::collections::HashMap<EnergyType, usize> {
        let mut counts = std::collections::HashMap::new();
        for energy_type in energy_list {
            *counts.entry(energy_type.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// 获取满足能量需求的攻击数组
    ///
    /// # 参数
    /// * `attached_energy` - 附加到宝可梦的能量类型列表
    ///
    /// # 返回值
    /// 返回可以使用的攻击列表及其索引
    pub fn get_usable_attacks(
        &self,
        attached_energy: &[EnergyType],
    ) -> Vec<(usize, &Attack)> {
        if !self.is_pokemon() {
            return Vec::new();
        }

        let attached_counts = Self::count_energy_types(attached_energy);
        let mut usable_attacks = Vec::new();

        for (index, attack) in self.attacks.iter().enumerate() {
            let required_counts = Self::count_energy_types(&attack.cost);

            let mut can_use = true;
            for (energy_type, &required_count) in &required_counts {
                let attached_count = attached_counts.get(energy_type).cloned().unwrap_or(0);
                if attached_count < required_count {
                    can_use = false;
                    break;
                }
            }

            if can_use {
                usable_attacks.push((index, attack));
            }
        }

        usable_attacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::{Attack, EvolutionStage};

    #[test]
    fn test_create_pokemon_card() {
        let card_type = CardType::Pokemon {
            species: "Pikachu".to_string(),
            hp: 60,
            retreat_cost: 1,
            weakness: Some(EnergyType::Fighting),
            resistance: None,
            stage: EvolutionStage::Basic,
            evolves_from: None,
        };
        
        let card = Card::new(
            "Pikachu".to_string(),
            card_type,
            "Base Set".to_string(),
            "58".to_string(),
            CardRarity::Common,
        );
        
        assert!(card.is_pokemon());
        assert!(!card.is_energy());
        assert!(!card.is_trainer());
        assert_eq!(card.name, "Pikachu");
        assert_eq!(card.get_hp(), Some(60));
    }

    #[test]
    fn test_create_energy_card() {
        let card_type = CardType::Energy {
            energy_type: EnergyType::Lightning,
            is_basic: true,
        };
        
        let card = Card::new(
            "Lightning Energy".to_string(),
            card_type,
            "Base Set".to_string(),
            "101".to_string(),
            CardRarity::Common,
        );
        
        assert!(!card.is_pokemon());
        assert!(card.is_energy());
        assert!(!card.is_trainer());
        assert_eq!(card.get_energy_type(), Some(&EnergyType::Lightning));
    }

    #[test]
    fn test_add_attack_to_pokemon() {
        let card_type = CardType::Pokemon {
            species: "Pikachu".to_string(),
            hp: 60,
            retreat_cost: 1,
            weakness: Some(EnergyType::Fighting),
            resistance: None,
            stage: EvolutionStage::Basic,
            evolves_from: None,
        };
        
        let mut card = Card::new(
            "Pikachu".to_string(),
            card_type,
            "Base Set".to_string(),
            "58".to_string(),
            CardRarity::Common,
        );
        
        let attack = Attack::simple(
            "Thunder Shock".to_string(),
            vec![EnergyType::Lightning],
            10,
        );
        
        card.add_attack(attack);
        assert_eq!(card.attacks.len(), 1);
    }
}