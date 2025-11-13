//! Game actions and operations
//!
//! This module contains functions for performing various game actions, including:
//! - Playing cards
//! - Attaching energy
//! - Using attacks
//! - Retreating Pokemon

// TODO: Implement game actions such as:
// - play_card
// - attach_energy
// - use_attack
// - retreat_pokemon
// - draw_card
// etc.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::player::Player;
    use crate::core::card::CardId;
    use uuid::Uuid;

    #[test]
    fn test_shuffle_deck() {
        let mut game = Game::new();
        let player1_id = Uuid::new_v4();
        let player2_id = Uuid::new_v4();
        
        // Add players to the game
        game.players.insert(player1_id, Player::new("Player 1".to_string()));
        game.players.insert(player2_id, Player::new("Player 2".to_string()));
        
        // Set up decks for both players
        let player1_deck: Vec<CardId> = (0..10).map(|_| Uuid::new_v4()).collect();
        let player2_deck: Vec<CardId> = (0..10).map(|_| Uuid::new_v4()).collect();
        
        game.players.get_mut(&player1_id).unwrap().set_deck(player1_deck.clone());
        game.players.get_mut(&player2_id).unwrap().set_deck(player2_deck.clone());
        
        // Save original orders
        let original_player1_order = game.players.get(&player1_id).unwrap().deck.clone();
        let original_player2_order = game.players.get(&player2_id).unwrap().deck.clone();
        
        // Shuffle one player's deck
        assert!(game.shuffle_deck(player1_id).is_ok());
        
        // Check that only player1's deck was shuffled
        let player1_deck_after = &game.players.get(&player1_id).unwrap().deck;
        let player2_deck_after = &game.players.get(&player2_id).unwrap().deck;
        
        assert_eq!(player2_deck_after, &original_player2_order);
        assert_eq!(player1_deck_after.len(), original_player1_order.len());
        
        // All original cards should still be present in player1's deck
        for card_id in &original_player1_order {
            assert!(player1_deck_after.contains(card_id));
        }
    }
    
    #[test]
    fn test_shuffle_both_decks() {
        let mut game = Game::new();
        let player1_id = Uuid::new_v4();
        let player2_id = Uuid::new_v4();
        
        // Add players to the game
        game.players.insert(player1_id, Player::new("Player 1".to_string()));
        game.players.insert(player2_id, Player::new("Player 2".to_string()));
        
        // Set up decks for both players
        let player1_deck: Vec<CardId> = (0..10).map(|_| Uuid::new_v4()).collect();
        let player2_deck: Vec<CardId> = (0..10).map(|_| Uuid::new_v4()).collect();
        
        game.players.get_mut(&player1_id).unwrap().set_deck(player1_deck.clone());
        game.players.get_mut(&player2_id).unwrap().set_deck(player2_deck.clone());
        
        // Save original orders
        let original_player1_order = game.players.get(&player1_id).unwrap().deck.clone();
        let original_player2_order = game.players.get(&player2_id).unwrap().deck.clone();
        
        // Shuffle both players' decks
        assert!(game.shuffle_both_decks().is_ok());
        
        // Check that both decks were shuffled
        let player1_deck_after = &game.players.get(&player1_id).unwrap().deck;
        let player2_deck_after = &game.players.get(&player2_id).unwrap().deck;
        
        assert_eq!(player1_deck_after.len(), original_player1_order.len());
        assert_eq!(player2_deck_after.len(), original_player2_order.len());
        
        // All original cards should still be present in both decks
        for card_id in &original_player1_order {
            assert!(player1_deck_after.contains(card_id));
        }
        
        for card_id in &original_player2_order {
            assert!(player2_deck_after.contains(card_id));
        }
    }
}