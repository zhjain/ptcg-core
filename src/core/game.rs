//! Main game logic and state management

use crate::core::{player::Player, card::{Card, CardId}, deck::Deck};
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
    Finished { winner: Option<crate::core::player::PlayerId> },
    /// Game was abandoned or cancelled
    Cancelled,
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
    pub players: HashMap<crate::core::player::PlayerId, Player>,
    /// Player turn order
    pub turn_order: Vec<crate::core::player::PlayerId>,
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

/// Events that can occur during a game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    /// Game started
    GameStarted,
    /// Turn started
    TurnStarted { player_id: crate::core::player::PlayerId, turn_number: u32 },
    /// Card was drawn
    CardDrawn { player_id: crate::core::player::PlayerId, card_id: Option<CardId> },
    /// Card was played
    CardPlayed { player_id: crate::core::player::PlayerId, card_id: CardId },
    /// Pokemon was played to bench
    PokemonBenched { player_id: crate::core::player::PlayerId, card_id: CardId },
    /// Energy was attached
    EnergyAttached { player_id: crate::core::player::PlayerId, energy_id: CardId, pokemon_id: CardId },
    /// Attack was used
    AttackUsed { player_id: crate::core::player::PlayerId, pokemon_id: CardId, attack_name: String },
    /// Damage was dealt
    DamageDealt { player_id: crate::core::player::PlayerId, pokemon_id: CardId, damage: u32 },
    /// Pokemon was knocked out
    PokemonKnockedOut { player_id: crate::core::player::PlayerId, pokemon_id: CardId },
    /// Prize card was taken
    PrizeTaken { player_id: crate::core::player::PlayerId },
    /// Turn ended
    TurnEnded { player_id: crate::core::player::PlayerId },
    /// Game ended
    GameEnded { winner: Option<crate::core::player::PlayerId> },
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
        }
    }

    /// Create a new game with custom rules
    pub fn with_rules(rules: GameRules) -> Self {
        let mut game = Self::new();
        game.rules = rules;
        game
    }

    /// Add a player to the game
    pub fn add_player(&mut self, mut player: Player) -> Result<(), String> {
        if self.state != GameState::Setup {
            return Err("Cannot add players after game has started".to_string());
        }

        if self.players.len() >= 2 {
            return Err("Maximum of 2 players allowed".to_string());
        }

        // Set prize cards according to game rules
        player.prize_cards = self.rules.prize_cards;
        
        let player_id = player.id;
        self.players.insert(player_id, player);
        self.turn_order.push(player_id);

        Ok(())
    }

    /// Set a player's deck
    pub fn set_player_deck(&mut self, player_id: crate::core::player::PlayerId, deck: Deck) -> Result<(), String> {
        if self.state != GameState::Setup {
            return Err("Cannot set deck after game has started".to_string());
        }

        // Add deck cards to the game's card database
        for &card_id in deck.cards.keys() {
            // In a real implementation, you'd load the card data here
            // For now, we'll assume the cards are already in the database
        }

        if let Some(player) = self.players.get_mut(&player_id) {
            let shuffled_cards = deck.shuffle();
            player.set_deck(shuffled_cards);
            Ok(())
        } else {
            Err("Player not found".to_string())
        }
    }

    /// Start the game
    pub fn start(&mut self) -> Result<(), String> {
        if self.state != GameState::Setup {
            return Err("Game is not in setup state".to_string());
        }

        if self.players.len() < 2 {
            return Err("Need at least 2 players to start".to_string());
        }

        // Validate all players have decks
        for player in self.players.values() {
            if player.deck.is_empty() {
                return Err("All players must have decks".to_string());
            }
        }

        self.state = GameState::InProgress;
        self.add_event(GameEvent::GameStarted);

        // Deal opening hands
        for player in self.players.values_mut() {
            player.draw_cards(7);
        }

        // Start the first turn
        self.start_turn()?;

        Ok(())
    }

    /// Start a new turn
    pub fn start_turn(&mut self) -> Result<(), String> {
        if self.state != GameState::InProgress {
            return Err("Game is not in progress".to_string());
        }

        let current_player_id = self.get_current_player_id()?;
        
        if let Some(player) = self.players.get_mut(&current_player_id) {
            player.start_turn();
            player.draw_card(); // Draw card at beginning of turn
        }

        self.phase = GamePhase::BeginningOfTurn;
        self.add_event(GameEvent::TurnStarted {
            player_id: current_player_id,
            turn_number: self.turn_number,
        });

        self.add_event(GameEvent::CardDrawn {
            player_id: current_player_id,
            card_id: None, // In a real game, you'd track which card was drawn
        });

        Ok(())
    }

    /// End the current turn and move to the next player
    pub fn end_turn(&mut self) -> Result<(), String> {
        if self.state != GameState::InProgress {
            return Err("Game is not in progress".to_string());
        }

        let current_player_id = self.get_current_player_id()?;
        
        if let Some(player) = self.players.get_mut(&current_player_id) {
            player.end_turn();
        }

        self.add_event(GameEvent::TurnEnded { player_id: current_player_id });

        // Check for win conditions
        if self.check_win_conditions()? {
            return Ok(());
        }

        // Move to next player
        self.current_player_index = (self.current_player_index + 1) % self.turn_order.len();
        
        // Increment turn number when we complete a full round
        if self.current_player_index == 0 {
            self.turn_number += 1;
        }

        self.start_turn()?;

        Ok(())
    }

    /// Get the current player's ID
    pub fn get_current_player_id(&self) -> Result<crate::core::player::PlayerId, String> {
        self.turn_order.get(self.current_player_index)
            .copied()
            .ok_or_else(|| "No current player".to_string())
    }

    /// Get the current player
    pub fn get_current_player(&self) -> Result<&Player, String> {
        let player_id = self.get_current_player_id()?;
        self.players.get(&player_id)
            .ok_or_else(|| "Current player not found".to_string())
    }

    /// Get a mutable reference to the current player
    pub fn get_current_player_mut(&mut self) -> Result<&mut Player, String> {
        let player_id = self.get_current_player_id()?;
        self.players.get_mut(&player_id)
            .ok_or_else(|| "Current player not found".to_string())
    }

    /// Advance to the next phase
    pub fn next_phase(&mut self) -> Result<(), String> {
        self.phase = match self.phase {
            GamePhase::BeginningOfTurn => GamePhase::Main,
            GamePhase::Main => GamePhase::Attack,
            GamePhase::Attack => GamePhase::EndOfTurn,
            GamePhase::EndOfTurn => {
                self.end_turn()?;
                return Ok(());
            }
        };
        Ok(())
    }

    /// Check for win conditions
    pub fn check_win_conditions(&mut self) -> Result<bool, String> {
        let mut winner = None;

        for (&player_id, player) in &self.players {
            if player.has_won() {
                winner = Some(player_id);
                break;
            }
            
            // Check if opponent has lost
            let opponent_lost = self.players.values()
                .any(|p| p.id != player_id && p.has_lost());
            
            if opponent_lost {
                winner = Some(player_id);
                break;
            }
        }

        if let Some(winner_id) = winner {
            self.state = GameState::Finished { winner: Some(winner_id) };
            self.add_event(GameEvent::GameEnded { winner: Some(winner_id) });
            return Ok(true);
        }

        Ok(false)
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
    pub fn is_player_turn(&self, player_id: crate::core::player::PlayerId) -> bool {
        self.get_current_player_id().map(|id| id == player_id).unwrap_or(false)
    }

    /// Get all players
    pub fn get_players(&self) -> &HashMap<crate::core::player::PlayerId, Player> {
        &self.players
    }

    /// Get a specific player
    pub fn get_player(&self, player_id: crate::core::player::PlayerId) -> Option<&Player> {
        self.players.get(&player_id)
    }

    /// Get a specific player (mutable)
    pub fn get_player_mut(&mut self, player_id: crate::core::player::PlayerId) -> Option<&mut Player> {
        self.players.get_mut(&player_id)
    }

    /// Force end the game
    pub fn end_game(&mut self, winner: Option<crate::core::player::PlayerId>) {
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
    use crate::core::deck::Deck;

    #[test]
    fn test_create_game() {
        let game = Game::new();
        assert_eq!(game.state, GameState::Setup);
        assert_eq!(game.players.len(), 0);
        assert_eq!(game.turn_number, 1);
    }

    #[test]
    fn test_add_players() {
        let mut game = Game::new();
        let player1 = Player::new("Player 1".to_string());
        let player2 = Player::new("Player 2".to_string());

        assert!(game.add_player(player1).is_ok());
        assert!(game.add_player(player2).is_ok());
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.turn_order.len(), 2);
    }

    #[test]
    fn test_too_many_players() {
        let mut game = Game::new();
        let player1 = Player::new("Player 1".to_string());
        let player2 = Player::new("Player 2".to_string());
        let player3 = Player::new("Player 3".to_string());

        assert!(game.add_player(player1).is_ok());
        assert!(game.add_player(player2).is_ok());
        assert!(game.add_player(player3).is_err());
    }

    #[test]
    fn test_game_phases() {
        let mut game = Game::new();
        assert_eq!(game.phase, GamePhase::BeginningOfTurn);

        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, GamePhase::Main);

        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, GamePhase::Attack);

        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, GamePhase::EndOfTurn);
    }

    #[test]
    fn test_custom_rules() {
        let rules = GameRules {
            format: "Expanded".to_string(),
            prize_cards: 4,
            max_hand_size: Some(10),
            turn_time_limit: Some(300),
            auto_shuffle: false,
        };

        let game = Game::with_rules(rules.clone());
        assert_eq!(game.rules, rules);
    }

    #[test]
    fn test_game_events() {
        let mut game = Game::new();
        
        game.add_event(GameEvent::GameStarted);
        assert_eq!(game.history.len(), 1);
        
        match &game.history[0] {
            GameEvent::GameStarted => (),
            _ => panic!("Expected GameStarted event"),
        }
    }
}