//! Player state management

use crate::core::card::{CardId, Card, EnergyType};
use crate::core::player::{SpecialConditionInstance, CardLocation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a player
pub type PlayerId = Uuid;

/// Represents a player in the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    /// Unique identifier for this player
    pub id: PlayerId,
    /// Player's display name
    pub name: String,
    /// Player's current life/prize cards remaining
    pub prize_cards: u32,
    /// Cards currently in hand
    pub hand: Vec<CardId>,
    /// Active Pokemon on the field
    pub active_pokemon: Option<CardId>,
    /// Pokemon on the bench
    pub bench: Vec<CardId>,
    /// Cards in the discard pile
    pub discard_pile: Vec<CardId>,
    /// Cards in the deck
    pub deck: Vec<CardId>,
    /// Energy cards attached to Pokemon
    pub attached_energy: HashMap<CardId, Vec<CardId>>,
    /// Damage counters on Pokemon
    pub damage_counters: HashMap<CardId, u32>,
    /// Player's current turn status
    pub has_attacked: bool,
    /// Whether the player can still play trainer cards this turn
    pub can_play_trainer: bool,
    /// Stadium card in play (if any)
    pub stadium: Option<CardId>,
    /// Special conditions affecting Pokemon
    pub special_conditions: HashMap<CardId, Vec<SpecialConditionInstance>>,
}

impl Player {
    /// Create a new player with the given name
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            prize_cards: 6, // Standard game starts with 6 prize cards
            hand: Vec::new(),
            active_pokemon: None,
            bench: Vec::new(),
            discard_pile: Vec::new(),
            deck: Vec::new(),
            attached_energy: HashMap::new(),
            damage_counters: HashMap::new(),
            has_attacked: false,
            can_play_trainer: true,
            stadium: None,
            special_conditions: HashMap::new(),
        }
    }

    /// Set the player's deck
    pub fn set_deck(&mut self, deck: Vec<CardId>) {
        self.deck = deck;
    }

    /// Draw a card from the deck to hand
    pub fn draw_card(&mut self) -> Option<CardId> {
        if let Some(card_id) = self.deck.pop() {
            self.hand.push(card_id);
            Some(card_id)
        } else {
            None
        }
    }

    /// Draw multiple cards from deck
    pub fn draw_cards(&mut self, count: usize) -> Vec<CardId> {
        let mut drawn = Vec::new();
        for _ in 0..count {
            if let Some(card_id) = self.draw_card() {
                drawn.push(card_id);
            } else {
                break;
            }
        }
        drawn
    }

    /// Shuffle the player's deck
    pub fn shuffle_deck(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Simple shuffle algorithm (in a real implementation, you'd use a proper RNG)
        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        let seed = hasher.finish();

        // Fisher-Yates shuffle with simple PRNG
        for i in (1..self.deck.len()).rev() {
            let j = (seed.wrapping_mul(i as u64 + 1)) % (i as u64 + 1);
            self.deck.swap(i, j as usize);
        }
    }

    /// Move a card from hand to discard pile
    pub fn discard_from_hand(&mut self, card_id: CardId) -> bool {
        if let Some(pos) = self.hand.iter().position(|&id| id == card_id) {
            self.hand.remove(pos);
            self.discard_pile.push(card_id);
            true
        } else {
            false
        }
    }

    /// Set the active Pokemon
    pub fn set_active_pokemon(&mut self, card_id: CardId) -> bool {
        if self.hand.contains(&card_id) || self.bench.contains(&card_id) {
            // Remove from current location
            self.hand.retain(|&id| id != card_id);
            self.bench.retain(|&id| id != card_id);

            // Set as active
            if let Some(old_active) = self.active_pokemon {
                self.bench.push(old_active);
            }
            self.active_pokemon = Some(card_id);
            true
        } else {
            false
        }
    }

    /// Add a Pokemon to the bench
    pub fn bench_pokemon(&mut self, card_id: CardId) -> bool {
        if self.bench.len() < 5 && self.hand.contains(&card_id) {
            if let Some(pos) = self.hand.iter().position(|&id| id == card_id) {
                self.hand.remove(pos);
                self.bench.push(card_id);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Attach energy to a Pokemon
    pub fn attach_energy(&mut self, energy_id: CardId, pokemon_id: CardId) -> bool {
        if self.hand.contains(&energy_id)
            && (Some(pokemon_id) == self.active_pokemon || self.bench.contains(&pokemon_id))
        {
            // Remove energy from hand
            if let Some(pos) = self.hand.iter().position(|&id| id == energy_id) {
                self.hand.remove(pos);

                // Attach to Pokemon
                self.attached_energy
                    .entry(pokemon_id)
                    .or_default()
                    .push(energy_id);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Add damage to a Pokemon
    pub fn add_damage(&mut self, pokemon_id: CardId, damage: u32) {
        let current_damage = self.damage_counters.get(&pokemon_id).unwrap_or(&0);
        self.damage_counters
            .insert(pokemon_id, current_damage + damage);
    }

    /// Heal damage from a Pokemon
    pub fn heal_damage(&mut self, pokemon_id: CardId, amount: u32) {
        if let Some(current_damage) = self.damage_counters.get_mut(&pokemon_id) {
            *current_damage = current_damage.saturating_sub(amount);
            if *current_damage == 0 {
                self.damage_counters.remove(&pokemon_id);
            }
        }
    }

    /// Check if a Pokemon is knocked out
    pub fn is_pokemon_knocked_out(&self, pokemon_id: CardId, card: &Card) -> bool {
        if let Some(hp) = card.get_hp() {
            let damage = self.damage_counters.get(&pokemon_id).unwrap_or(&0);
            *damage >= hp
        } else {
            false
        }
    }

    /// Get the total energy attached to a Pokemon
    pub fn get_attached_energy_count(&self, pokemon_id: CardId) -> usize {
        self.attached_energy
            .get(&pokemon_id)
            .map(|energy| energy.len())
            .unwrap_or(0)
    }

    /// Take a prize card
    pub fn take_prize_card(&mut self) -> bool {
        if self.prize_cards > 0 {
            self.prize_cards -= 1;
            // In a real implementation, you'd move a specific card from prizes to hand
            true
        } else {
            false
        }
    }

    /// Reset turn-based flags
    pub fn start_turn(&mut self) {
        self.has_attacked = false;
        self.can_play_trainer = true;
    }

    /// End turn
    pub fn end_turn(&mut self) {
        // Any end-of-turn effects would go here
    }

    /// Check if the player has lost (no active Pokemon and no bench)
    pub fn has_lost(&self) -> bool {
        self.active_pokemon.is_none() && self.bench.is_empty()
    }

    /// Check if the player has won (no prize cards left)
    pub fn has_won(&self) -> bool {
        self.prize_cards == 0
    }

    /// Get the location of a specific card
    pub fn find_card_location(&self, card_id: CardId) -> Option<CardLocation> {
        if self.hand.contains(&card_id) {
            Some(CardLocation::Hand)
        } else if self.deck.contains(&card_id) {
            Some(CardLocation::Deck)
        } else if self.discard_pile.contains(&card_id) {
            Some(CardLocation::DiscardPile)
        } else if Some(card_id) == self.active_pokemon {
            Some(CardLocation::Active)
        } else if let Some(index) = self.bench.iter().position(|&id| id == card_id) {
            Some(CardLocation::Bench(index))
        } else {
            // Check if it's attached energy
            for (pokemon_id, energy_cards) in &self.attached_energy {
                if energy_cards.contains(&card_id) {
                    return Some(CardLocation::AttachedEnergy(*pokemon_id));
                }
            }
            None
        }
    }

    /// Find all basic Pokemon cards in the player's hand
    pub fn find_basic_pokemon_in_hand(&self, card_database: &HashMap<CardId, Card>) -> Vec<CardId> {
        let mut basic_pokemon = Vec::new();

        for &card_id in &self.hand {
            if let Some(card) = card_database.get(&card_id) {
                // 检查是否是宝可梦卡并且是基础阶段
                if let crate::core::card::CardType::Pokemon {
                    stage: crate::core::card::EvolutionStage::Basic,
                    ..
                } = card.card_type
                {
                    basic_pokemon.push(card_id);
                }
            }
        }

        basic_pokemon
    }

    /// 从牌库顶部抽取指定数量的卡牌作为奖赏卡
    pub fn draw_prize_cards(&mut self, count: usize) -> Vec<CardId> {
        let mut prize_cards = Vec::new();

        for _ in 0..count {
            if let Some(card_id) = self.deck.pop() {
                prize_cards.push(card_id);
            } else {
                break;
            }
        }

        prize_cards
    }

    /// 获取指定宝可梦的附加能量类型列表
    ///
    /// # 参数
    /// * `pokemon_id` - 宝可梦的ID
    /// * `card_database` - 卡牌数据库，用于查找能量卡的类型
    ///
    /// # 返回值
    /// 返回附加到该宝可梦的所有能量类型的列表
    pub fn get_attached_energy_types(
        &self,
        pokemon_id: CardId,
        card_database: &std::collections::HashMap<CardId, Card>,
    ) -> Vec<EnergyType> {
        let mut energy_types = Vec::new();

        if let Some(energy_cards) = self.attached_energy.get(&pokemon_id) {
            for energy_id in energy_cards {
                if let Some(energy_card) = card_database.get(energy_id)
                    && let Some(energy_type) = energy_card.get_energy_type() {
                        energy_types.push(energy_type.clone());
                    }
            }
        }

        energy_types
    }
}