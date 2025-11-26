//! Event types and definitions

use crate::core::card::CardId;
use crate::core::player::PlayerId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for an event
pub type EventId = Uuid;

/// Events that can occur during a game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    /// Game started
    GameStarted {
        timestamp: u64,
        players: Vec<PlayerId>,
    },
    /// Turn started
    TurnStarted {
        timestamp: u64,
        player_id: PlayerId,
        turn_number: u32,
    },
    /// Card was drawn
    CardDrawn {
        timestamp: u64,
        player_id: PlayerId,
        card_id: Option<CardId>,
    },
    /// Card was played
    CardPlayed {
        timestamp: u64,
        player_id: PlayerId,
        card_id: CardId,
    },
    /// Pokemon was played to bench
    PokemonBenched {
        timestamp: u64,
        player_id: PlayerId,
        card_id: CardId,
    },
    /// Energy was attached
    EnergyAttached {
        timestamp: u64,
        player_id: PlayerId,
        energy_id: CardId,
        pokemon_id: CardId,
    },
    /// Attack was used
    AttackUsed {
        timestamp: u64,
        player_id: PlayerId,
        pokemon_id: CardId,
        attack_name: String,
    },
    /// Damage was dealt
    DamageDealt {
        timestamp: u64,
        player_id: PlayerId,
        pokemon_id: CardId,
        damage: u32,
    },
    /// Pokemon was knocked out
    PokemonKnockedOut {
        timestamp: u64,
        player_id: PlayerId,
        pokemon_id: CardId,
    },
    /// Prize card was taken
    PrizeTaken { 
        timestamp: u64,
        player_id: PlayerId 
    },
    /// Deck was shuffled
    DeckShuffled { 
        timestamp: u64,
        player_id: PlayerId 
    },
    /// Turn ended
    TurnEnded { 
        timestamp: u64,
        player_id: PlayerId 
    },
    /// Game ended
    GameEnded { 
        timestamp: u64,
        winner: Option<PlayerId> 
    },
}