//! 牌组验证功能

use crate::core::card::{Card, CardId, CardType, EnergyType};
use crate::core::deck::Deck;
use std::collections::HashMap;

/// 牌组统计信息
#[derive(Debug, Clone)]
pub struct DeckStatistics {
    pub total_cards: u32,
    pub unique_cards: usize,
    pub pokemon_count: u32,
    pub energy_count: u32,
    pub trainer_count: u32,
    pub basic_pokemon_count: u32,
    pub energy_distribution: HashMap<EnergyType, u32>,
}

/// 牌组验证错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeckValidationError {
    /// 牌组卡牌数量过少
    TooFewCards { minimum: u32, actual: u32 },
    /// 卡牌副本数量过多（违反4副本规则）
    TooManyCopies { card_id: CardId, maximum: u32, actual: u32 },
    /// 牌组卡牌数量过多
    TooManyCards { maximum: u32, actual: u32 },
    /// 牌组中没有基础宝可梦
    NoBasicPokemon,
    /// 基础宝可梦数量过多
    TooManyBasicPokemon { maximum: u32, actual: u32 },
}

impl Deck {
    /// 获取牌组统计信息
    pub fn get_statistics(&self, card_database: &HashMap<CardId, Card>) -> DeckStatistics {
        let mut stats = DeckStatistics {
            total_cards: 0,
            unique_cards: 0,
            pokemon_count: 0,
            energy_count: 0,
            trainer_count: 0,
            basic_pokemon_count: 0,
            energy_distribution: HashMap::new(),
        };

        for (&card_id, &count) in &self.cards {
            if let Some(card) = card_database.get(&card_id) {
                stats.total_cards += count;
                stats.unique_cards += 1;

                match &card.card_type {
                    CardType::Pokemon { stage, .. } => {
                        stats.pokemon_count += count;
                        if matches!(stage, crate::core::card::EvolutionStage::Basic) {
                            stats.basic_pokemon_count += count;
                        }
                    }
                    CardType::Energy { energy_type, .. } => {
                        stats.energy_count += count;
                        *stats.energy_distribution.entry(energy_type.clone()).or_insert(0) += count;
                    }
                    CardType::Trainer { .. } => {
                        stats.trainer_count += count;
                    }
                }
            }
        }

        stats
    }

    /// 根据标准PTCG规则验证牌组
    pub fn validate(&self, card_database: &HashMap<CardId, Card>) -> Result<(), Vec<DeckValidationError>> {
        let mut errors = Vec::new();

        // 检查最小牌组大小（通常为60张卡牌）
        let total_cards = self.total_cards();
        if total_cards < 60 {
            errors.push(DeckValidationError::TooFewCards {
                minimum: 60,
                actual: total_cards,
            });
        }

        // 检查最大牌组大小（标准格式通常为60张卡牌）
        if total_cards > 60 {
            errors.push(DeckValidationError::TooManyCards {
                maximum: 60,
                actual: total_cards,
            });
        }

        // 检查4副本规则（除基本能量卡外，任何卡牌最多4张）
        for (&card_id, &count) in &self.cards {
            if let Some(card) = card_database.get(&card_id) {
                // 基本能量卡不受4副本规则限制
                let is_basic_energy = matches!(card.card_type, CardType::Energy { is_basic: true, .. });
                
                if !is_basic_energy && count > 4 {
                    errors.push(DeckValidationError::TooManyCopies {
                        card_id,
                        maximum: 4,
                        actual: count,
                    });
                }
            }
        }

        // 检查是否有基础宝可梦
        let stats = self.get_statistics(card_database);
        if stats.basic_pokemon_count == 0 {
            errors.push(DeckValidationError::NoBasicPokemon);
        }

        // 检查最大基础宝可梦数量（60卡牌牌组中通常为4张）
        if stats.basic_pokemon_count > 4 {
            errors.push(DeckValidationError::TooManyBasicPokemon {
                maximum: 4,
                actual: stats.basic_pokemon_count,
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::{Card, CardType, EvolutionStage, EnergyType, CardRarity, TrainerType};

    #[test]
    fn test_deck_statistics() {
        let mut deck = Deck::new("Test Deck".to_string(), "Standard".to_string());
        let mut card_database = HashMap::new();

        // 创建测试卡牌
        let pokemon_card = Card::new(
            "Pikachu".to_string(),
            CardType::Pokemon {
                species: "Pikachu".to_string(),
                hp: 60,
                retreat_cost: 1,
                weakness: Some(EnergyType::Fighting),
                resistance: None,
                stage: EvolutionStage::Basic,
                evolves_from: None,
            },
            "Base Set".to_string(),
            "025".to_string(),
            CardRarity::Common,
        );

        let energy_card = Card::new(
            "Lightning Energy".to_string(),
            CardType::Energy {
                energy_type: EnergyType::Lightning,
                is_basic: true,
            },
            "Base Set".to_string(),
            "100".to_string(),
            CardRarity::Common,
        );

        let trainer_card = Card::new(
            "Professor Oak".to_string(),
            CardType::Trainer {
                trainer_type: TrainerType::Supporter,
            },
            "Base Set".to_string(),
            "150".to_string(),
            CardRarity::Uncommon,
        );

        let pokemon_id = pokemon_card.id;
        let energy_id = energy_card.id;
        let trainer_id = trainer_card.id;

        card_database.insert(pokemon_id, pokemon_card);
        card_database.insert(energy_id, energy_card);
        card_database.insert(trainer_id, trainer_card);

        // 向牌组添加卡牌
        deck.add_card(pokemon_id, 4);
        deck.add_card(energy_id, 20);
        deck.add_card(trainer_id, 10);

        let stats = deck.get_statistics(&card_database);
        assert_eq!(stats.total_cards, 34);
        assert_eq!(stats.pokemon_count, 4);
        assert_eq!(stats.energy_count, 20);
        assert_eq!(stats.trainer_count, 10);
        assert_eq!(stats.basic_pokemon_count, 4);
    }

    #[test]
    fn test_valid_deck_validation() {
        let mut deck = Deck::new("Valid Deck".to_string(), "Standard".to_string());
        let mut card_database = HashMap::new();

        // 创建测试卡牌
        let pokemon_card = Card::new(
            "Pikachu".to_string(),
            CardType::Pokemon {
                species: "Pikachu".to_string(),
                hp: 60,
                retreat_cost: 1,
                weakness: Some(EnergyType::Fighting),
                resistance: None,
                stage: EvolutionStage::Basic,
                evolves_from: None,
            },
            "Base Set".to_string(),
            "025".to_string(),
            CardRarity::Common,
        );

        let energy_card = Card::new(
            "Lightning Energy".to_string(),
            CardType::Energy {
                energy_type: EnergyType::Lightning,
                is_basic: true,
            },
            "Base Set".to_string(),
            "100".to_string(),
            CardRarity::Common,
        );

        let pokemon_id = pokemon_card.id;
        let energy_id = energy_card.id;

        card_database.insert(pokemon_id, pokemon_card);
        card_database.insert(energy_id, energy_card);

        // 向牌组添加60张卡牌
        deck.add_card(pokemon_id, 4);
        deck.add_card(energy_id, 56);

        // 验证应该成功，因为卡牌数量正好60张且有基础宝可梦
        let result = deck.validate(&card_database);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_deck_validation() {
        let mut deck = Deck::new("Invalid Deck".to_string(), "Standard".to_string());
        let mut card_database = HashMap::new();

        // 创建测试卡牌
        let pokemon_card = Card::new(
            "Pikachu".to_string(),
            CardType::Pokemon {
                species: "Pikachu".to_string(),
                hp: 60,
                retreat_cost: 1,
                weakness: Some(EnergyType::Fighting),
                resistance: None,
                stage: EvolutionStage::Basic,
                evolves_from: None,
            },
            "Base Set".to_string(),
            "025".to_string(),
            CardRarity::Common,
        );

        let energy_card = Card::new(
            "Lightning Energy".to_string(),
            CardType::Energy {
                energy_type: EnergyType::Lightning,
                is_basic: true,
            },
            "Base Set".to_string(),
            "100".to_string(),
            CardRarity::Common,
        );

        let pokemon_id = pokemon_card.id;
        let energy_id = energy_card.id;

        card_database.insert(pokemon_id, pokemon_card);
        card_database.insert(energy_id, energy_card);

        // 向牌组添加少于60张卡牌
        deck.add_card(pokemon_id, 4);
        deck.add_card(energy_id, 50); // 只有54张卡牌

        // 验证应该失败，因为卡牌数量不足60张
        let result = deck.validate(&card_database);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        // 应该有一个错误：卡牌数量不足
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], DeckValidationError::TooFewCards { .. }));
    }
}