//! 牌组管理功能

use crate::core::card::CardId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 表示玩家的牌组
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deck {
    /// 牌组名称
    pub name: String,
    /// 牌组适用的格式（标准、扩展等）
    pub format: String,
    /// 牌组中的卡牌及其数量
    pub cards: HashMap<CardId, u32>,
}

impl Deck {
    /// 创建一个新的空牌组
    pub fn new(name: String, format: String) -> Self {
        Self {
            name,
            format,
            cards: HashMap::new(),
        }
    }

    /// 向牌组添加卡牌
    pub fn add_card(&mut self, card_id: CardId, count: u32) {
        *self.cards.entry(card_id).or_insert(0) += count;
    }

    /// 从牌组移除卡牌
    pub fn remove_card(&mut self, card_id: CardId, count: u32) -> bool {
        if let Some(current_count) = self.cards.get_mut(&card_id) {
            if *current_count >= count {
                *current_count -= count;
                if *current_count == 0 {
                    self.cards.remove(&card_id);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// 获取牌组中特定卡牌的数量
    pub fn get_card_count(&self, card_id: CardId) -> u32 {
        *self.cards.get(&card_id).unwrap_or(&0)
    }

    /// 获取牌组中的卡牌总数
    pub fn total_cards(&self) -> u32 {
        self.cards.values().sum()
    }

    /// 检查牌组是否包含特定卡牌
    pub fn contains_card(&self, card_id: CardId) -> bool {
        self.cards.contains_key(&card_id)
    }

    /// 获取牌组中的所有唯一卡牌
    pub fn unique_cards(&self) -> Vec<CardId> {
        self.cards.keys().cloned().collect()
    }

    /// 洗牌并返回随机顺序的卡牌ID
    pub fn shuffle(&self) -> Vec<CardId> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut cards = Vec::new();
        for (&card_id, &count) in &self.cards {
            for _ in 0..count {
                cards.push(card_id);
            }
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new("Test Deck".to_string(), "Standard".to_string());
        assert_eq!(deck.name, "Test Deck");
        assert_eq!(deck.format, "Standard");
        assert_eq!(deck.total_cards(), 0);
    }

    #[test]
    fn test_add_and_remove_cards() {
        let mut deck = Deck::new("Test Deck".to_string(), "Standard".to_string());
        let card_id = Uuid::new_v4();
        
        // 添加卡牌
        deck.add_card(card_id, 4);
        assert_eq!(deck.get_card_count(card_id), 4);
        assert_eq!(deck.total_cards(), 4);
        
        // 移除部分卡牌
        assert!(deck.remove_card(card_id, 2));
        assert_eq!(deck.get_card_count(card_id), 2);
        assert_eq!(deck.total_cards(), 2);
        
        // 移除所有剩余卡牌
        assert!(deck.remove_card(card_id, 2));
        assert_eq!(deck.get_card_count(card_id), 0);
        assert_eq!(deck.total_cards(), 0);
        assert!(!deck.contains_card(card_id));
    }

    #[test]
    fn test_remove_more_than_available() {
        let mut deck = Deck::new("Test Deck".to_string(), "Standard".to_string());
        let card_id = Uuid::new_v4();
        
        deck.add_card(card_id, 2);
        // 尝试移除比拥有的更多的卡牌应该失败
        assert!(!deck.remove_card(card_id, 3));
        // 卡牌数量应该保持不变
        assert_eq!(deck.get_card_count(card_id), 2);
    }
}