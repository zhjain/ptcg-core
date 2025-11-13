//! Game state and phase management
//!
//! This module contains the core game structure, state enums, and basic game management.

use crate::core::{
    card::{Card, CardId},
    player::{Player, PlayerId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a game
pub type GameId = Uuid;

/// Represents the current phase of a turn
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    /// Beginning of turn (draw card, flip coins for special conditions)
    BeginningOfTurn,
    /// Main phase (play cards, attach energy, evolve Pokemon)
    Main,
    /// Attack phase
    Attack,
    /// End of turn (apply poison/burn damage, check for win conditions)
    EndOfTurn,
}

/// Represents the overall state of the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    /// Game is being set up
    Setup,
    /// Game is actively being played
    InProgress,
    /// Game has ended
    Finished { winner: Option<PlayerId> },
    /// Game was abandoned or cancelled
    Cancelled,
}

/// Represents the setup phase of the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupPhase {
    /// Initial state - waiting to determine turn order
    WaitingForTurnOrder,
    /// Turn order determined - waiting to deal hands
    WaitingForHands,
    /// Hands dealt - checking for basic Pokemon
    CheckingForBasicPokemon,
    /// Players need to mulligan (no basic Pokemon)
    MulliganRequired,
    /// Players selecting active Pokemon
    SelectingActivePokemon,
    /// Players setting up bench
    SettingUpBench,
    /// Placing prize cards
    PlacingPrizeCards,
    /// Setup complete - ready to start game
    SetupComplete,
}

/// Game rules and settings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameRules {
    /// Format being played (Standard, Expanded, etc.)
    pub format: String,
    /// Number of prize cards each player starts with
    pub prize_cards: u32,
    /// Maximum hand size (usually unlimited in PTCG)
    pub max_hand_size: Option<u32>,
    /// Time limit per turn (in seconds)
    pub turn_time_limit: Option<u32>,
    /// Whether to use automatic deck shuffling
    pub auto_shuffle: bool,
}

/// Main game structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Unique identifier for this game
    pub id: GameId,
    /// Current state of the game
    pub state: GameState,
    /// Current phase of the turn
    pub phase: GamePhase,
    /// All players in the game
    pub players: HashMap<PlayerId, Player>,
    /// Player turn order
    pub turn_order: Vec<PlayerId>,
    /// Index of the current player in turn_order
    pub current_player_index: usize,
    /// All cards used in this game
    pub card_database: HashMap<CardId, Card>,
    /// Turn counter
    pub turn_number: u32,
    /// Game rules and settings
    pub rules: GameRules,
    /// Game history/log
    pub history: Vec<GameEvent>,
    /// Players waiting for mulligan after opponent completes setup
    pub players_waiting_for_mulligan: Vec<PlayerId>,
    /// Record of mulligan counts for each player
    pub mulligan_counts: Vec<usize>,
}

/// Events that can occur during a game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    /// Game started
    GameStarted,
    /// Turn started
    TurnStarted {
        player_id: PlayerId,
        turn_number: u32,
    },
    /// Card was drawn
    CardDrawn {
        player_id: PlayerId,
        card_id: Option<CardId>,
    },
    /// Card was played
    CardPlayed {
        player_id: PlayerId,
        card_id: CardId,
    },
    /// Pokemon was played to bench
    PokemonBenched {
        player_id: PlayerId,
        card_id: CardId,
    },
    /// Energy was attached
    EnergyAttached {
        player_id: PlayerId,
        energy_id: CardId,
        pokemon_id: CardId,
    },
    /// Attack was used
    AttackUsed {
        player_id: PlayerId,
        pokemon_id: CardId,
        attack_name: String,
    },
    /// Damage was dealt
    DamageDealt {
        player_id: PlayerId,
        pokemon_id: CardId,
        damage: u32,
    },
    /// Pokemon was knocked out
    PokemonKnockedOut {
        player_id: PlayerId,
        pokemon_id: CardId,
    },
    /// Prize card was taken
    PrizeTaken { player_id: PlayerId },
    /// Deck was shuffled
    DeckShuffled { player_id: PlayerId },
    /// Turn ended
    TurnEnded { player_id: PlayerId },
    /// Game ended
    GameEnded { winner: Option<PlayerId> },
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            format: "Standard".to_string(),
            prize_cards: 6,
            max_hand_size: None,
            turn_time_limit: None,
            auto_shuffle: true,
        }
    }
}

impl Game {
    /// Create a new game with default rules
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            state: GameState::Setup,
            phase: GamePhase::BeginningOfTurn,
            players: HashMap::new(),
            turn_order: Vec::new(),
            current_player_index: 0,
            card_database: HashMap::new(),
            turn_number: 1,
            rules: GameRules::default(),
            history: Vec::new(),
            players_waiting_for_mulligan: Vec::new(),
            mulligan_counts: Vec::new(),
        }
    }

    /// Create a new game with custom rules
    pub fn with_rules(rules: GameRules) -> Self {
        let mut game = Self::new();
        game.rules = rules;
        game
    }

    /// Add a card to the game's database
    pub fn add_card_to_database(&mut self, card: Card) {
        self.card_database.insert(card.id, card);
    }

    /// Get a card from the database
    pub fn get_card(&self, card_id: CardId) -> Option<&Card> {
        self.card_database.get(&card_id)
    }

    /// Add an event to the game history
    pub fn add_event(&mut self, event: GameEvent) {
        self.history.push(event);
    }

    /// Get the game history
    pub fn get_history(&self) -> &[GameEvent] {
        &self.history
    }

    /// Check if it's a specific player's turn
    pub fn is_player_turn(&self, player_id: PlayerId) -> bool {
        self.get_current_player_id()
            .map(|id| id == player_id)
            .unwrap_or(false)
    }

    /// Get all players
    pub fn get_players(&self) -> &HashMap<PlayerId, Player> {
        &self.players
    }

    /// Get a specific player
    pub fn get_player(&self, player_id: PlayerId) -> Option<&Player> {
        self.players.get(&player_id)
    }

    /// Get a specific player (mutable)
    pub fn get_player_mut(&mut self, player_id: PlayerId) -> Option<&mut Player> {
        self.players.get_mut(&player_id)
    }

    /// Get the current player's ID
    pub fn get_current_player_id(&self) -> Result<PlayerId, String> {
        self.turn_order
            .get(self.current_player_index)
            .copied()
            .ok_or_else(|| "No current player".to_string())
    }

    /// Get the current player
    pub fn get_current_player(&self) -> Result<&Player, String> {
        let player_id = self.get_current_player_id()?;
        self.players
            .get(&player_id)
            .ok_or_else(|| "Current player not found".to_string())
    }

    /// Get a mutable reference to the current player
    pub fn get_current_player_mut(&mut self) -> Result<&mut Player, String> {
        let player_id = self.get_current_player_id()?;
        self.players
            .get_mut(&player_id)
            .ok_or_else(|| "Current player not found".to_string())
    }

    /// Force end the game
    pub fn end_game(&mut self, winner: Option<PlayerId>) {
        self.state = GameState::Finished { winner };
        self.add_event(GameEvent::GameEnded { winner });
    }

    /// Cancel the game
    pub fn cancel_game(&mut self) {
        self.state = GameState::Cancelled;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_game_creation() {
        let game = Game::new();
        assert_eq!(game.state, GameState::Setup);
        assert_eq!(game.phase, GamePhase::BeginningOfTurn);
        assert_eq!(game.turn_number, 1);
    }

    #[test]
    fn test_game_with_rules() {
        let rules = GameRules {
            format: "Expanded".to_string(),
            prize_cards: 6,
            max_hand_size: Some(7),
            turn_time_limit: Some(50),
            auto_shuffle: false,
        };
        
        let game = Game::with_rules(rules.clone());
        assert_eq!(game.rules, rules);
    }
    
    #[test]
    fn test_add_player() {
        let mut game = Game::new();
        let player = Player::new("Alice".to_string());
        let player_id = player.id;
        assert!(game.add_player(player).is_ok());
        
        assert!(game.players.contains_key(&player_id));
        assert_eq!(game.players.get(&player_id).unwrap().name, "Alice");
    }
    
    #[test]
    fn test_set_turn_order() {
        let mut game = Game::new();
        let player1 = Player::new("Alice".to_string());
        let player1_id = player1.id;
        let player2 = Player::new("Bob".to_string());
        let player2_id = player2.id;
        
        assert!(game.add_player(player1).is_ok());
        assert!(game.add_player(player2).is_ok());
        
        assert!(game.determine_turn_order().is_ok());
        
        assert_eq!(game.turn_order.len(), 2);
        assert_eq!(game.current_player_index, 0);
    }
}
