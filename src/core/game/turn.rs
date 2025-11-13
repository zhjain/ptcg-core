//! Turn processing and flow control
//!
//! This module contains functions for managing game turns, including:
//! - Starting and ending turns
//! - Phase advancement
//! - Win condition checking

use crate::core::game::state::{Game, GameEvent, GamePhase, GameState};

impl Game {
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

        self.add_event(GameEvent::TurnEnded {
            player_id: current_player_id,
        });

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
            let opponent_lost = self
                .players
                .values()
                .any(|p| p.id != player_id && p.has_lost());

            if opponent_lost {
                winner = Some(player_id);
                break;
            }
        }

        if let Some(winner_id) = winner {
            self.state = GameState::Finished {
                winner: Some(winner_id),
            };
            self.add_event(GameEvent::GameEnded {
                winner: Some(winner_id),
            });
            return Ok(true);
        }

        Ok(false)
    }
}
