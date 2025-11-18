//! Card-related game actions

use crate::core::game::state::{Game, GameEvent};
use crate::core::player::PlayerId;

impl Game {
    /// Shuffle a player's deck
    pub fn shuffle_deck(&mut self, player_id: PlayerId) -> Result<(), String> {
        // Check if the player exists
        if !self.players.contains_key(&player_id) {
            return Err("Player not found".to_string());
        }

        // Get mutable reference to the player and shuffle their deck
        if let Some(player) = self.players.get_mut(&player_id) {
            player.shuffle_deck();
        }

        // Add event for shuffling deck
        self.add_event(GameEvent::DeckShuffled { player_id });

        Ok(())
    }

    /// Shuffle both players' decks
    pub fn shuffle_both_decks(&mut self) -> Result<(), String> {
        // Collect player IDs first to avoid borrowing issues
        let player_ids: Vec<PlayerId> = self.players.keys().cloned().collect();

        // Shuffle each player's deck
        for player_id in player_ids {
            self.shuffle_deck(player_id)?;
        }

        Ok(())
    }
}