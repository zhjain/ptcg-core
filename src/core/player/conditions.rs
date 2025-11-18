//! Special conditions and status effects for Pokemon

use crate::core::card::CardId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Special conditions that can affect Pokemon
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialConditionInstance {
    /// Type of condition
    pub condition: SpecialCondition,
    /// Duration in turns (-1 for permanent until cured)
    pub duration: i32,
    /// When this condition was applied (turn number)
    pub applied_turn: u32,
    /// Additional data for the condition
    pub data: HashMap<String, String>,
}

/// Effects that can be triggered by special conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionEffect {
    /// Deal damage to a Pokemon
    Damage {
        pokemon_id: CardId,
        amount: u32,
        source: String,
    },
    /// Requires a coin flip
    CoinFlip {
        pokemon_id: CardId,
        condition: String,
        on_success: String,
    },
    /// Condition was removed
    ConditionRemoved {
        pokemon_id: CardId,
        condition: String,
    },
    /// Prevent an action
    PreventAction { pokemon_id: CardId, action: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialCondition {
    /// Pokemon is poisoned (takes damage between turns)
    Poisoned {
        /// Damage per turn
        damage_per_turn: u32,
    },
    /// Pokemon is burned (flip for damage and removal)
    Burned {
        /// Damage per turn
        damage_per_turn: u32,
    },
    /// Pokemon cannot attack next turn
    Paralyzed,
    /// Pokemon cannot attack (flip to wake up)
    Asleep,
    /// Pokemon may attack itself
    Confused,
    /// Pokemon cannot retreat
    Trapped,
    /// Custom condition with description
    Custom { name: String, description: String },
}

/// Represents where a card is located for a player
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardLocation {
    Hand,
    Deck,
    DiscardPile,
    Active,
    Bench(usize), // Index on the bench
    Prizes,
    AttachedEnergy(CardId), // Attached to the specified Pokemon
}