//! Effect targeting system

use crate::core::card::{CardId, EnergyType};
use crate::core::player::PlayerId;
use serde::{Deserialize, Serialize};

/// Different types of effect triggers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectTrigger {
    /// Triggered when the card is played
    OnPlay,
    /// Triggered when the Pokemon comes into play
    OnEnterPlay,
    /// Triggered when the Pokemon leaves play
    OnLeavePlay,
    /// Triggered when the Pokemon is knocked out
    OnKnockOut,
    /// Triggered at the beginning of each turn
    OnTurnStart,
    /// Triggered at the end of each turn
    OnTurnEnd,
    /// Triggered when the Pokemon takes damage
    OnTakeDamage,
    /// Triggered when the Pokemon deals damage
    OnDealDamage,
    /// Triggered when an attack is used
    OnAttack,
    /// Triggered when energy is attached
    OnEnergyAttach,
    /// Triggered when a card is drawn
    OnCardDraw,
    /// Triggered by a specific game event
    OnGameEvent { event_type: String },
    /// Manually triggered (by player action)
    Manual,
    /// Triggered once when conditions are met
    Once { condition: String },
}

/// Different types of effect targets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectTarget {
    /// No target required
    None,
    /// Target is the source card itself
    Self_,
    /// Target is a specific card
    Card(CardId),
    /// Target is a specific player
    Player(PlayerId),
    /// Target is all Pokemon of a player
    AllPlayerPokemon(PlayerId),
    /// Target is all Pokemon in play
    AllPokemon,
    /// Target is the active Pokemon of a player
    ActivePokemon(PlayerId),
    /// Target is a random card/Pokemon
    Random { filter: String },
    /// Target chosen by player
    Choice { options: Vec<CardId> },
}

/// Requirements for effect targets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetRequirement {
    /// Must be a Pokemon card
    Pokemon,
    /// Must be an Energy card
    Energy,
    /// Must be a Trainer card
    Trainer,
    /// Must be in play (on field)
    InPlay,
    /// Must be in hand
    InHand,
    /// Must be in discard pile
    InDiscard,
    /// Must be owned by specific player
    OwnedBy(PlayerId),
    /// Must have specific energy type attached
    HasEnergyType(EnergyType),
    /// Must have at least specified HP
    MinHP(u32),
    /// Must have at least specified damage
    MinDamage(u32),
    /// Custom requirement
    Custom(String),
}