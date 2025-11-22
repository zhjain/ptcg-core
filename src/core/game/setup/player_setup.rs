//! Player setup functionality

use crate::core::{
    deck::Deck,
    game::state::{Game, GameState},
    player::{Player, PlayerId},
};

impl Game {
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

        Ok(())
    }

    /// Set a player's deck
    pub fn set_player_deck(&mut self, player_id: PlayerId, deck: Deck) -> Result<(), String> {
        if self.state != GameState::Setup {
            return Err("Cannot set deck after game has started".to_string());
        }

        // Add deck cards to the game's card database
        for &_card_id in deck.cards.keys() {
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
}