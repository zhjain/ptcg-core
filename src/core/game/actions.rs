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

    /// Execute a game action using the provided rule engine
    ///
    /// # Parameters
    /// * `rule_engine` - The rule engine to validate and apply the action
    /// * `action` - The action to execute
    ///
    /// # Returns
    /// * `Ok(())` if the action was successfully executed
    /// * `Err(Vec<RuleViolation>)` if the action violated any rules
    pub fn execute_action(
        &mut self,
        rule_engine: &crate::rules::RuleEngine,
        action: &crate::rules::GameAction,
    ) -> Result<(), Vec<crate::rules::RuleViolation>> {
        // First validate the action
        let violations = rule_engine.validate_action(self, action);

        // Check if there are any blocking violations
        let has_errors = violations.iter().any(|v| {
            matches!(
                v.severity,
                crate::rules::ViolationSeverity::Error | crate::rules::ViolationSeverity::Fatal
            )
        });

        if has_errors {
            return Err(violations);
        }

        // Apply the action based on its type
        match action {
            crate::rules::GameAction::DrawCard { player_id } => {
                if let Some(player) = self.players.get_mut(player_id) {
                    if let Some(card_id) = player.draw_card() {
                        self.add_event(GameEvent::CardDrawn {
                            player_id: *player_id,
                            card_id: Some(card_id),
                        });
                    } else {
                        self.add_event(GameEvent::CardDrawn {
                            player_id: *player_id,
                            card_id: None,
                        });
                    }
                }
            }
            crate::rules::GameAction::PlayCard {
                player_id,
                card_id,
                target: _,
            } => {
                // TODO: Implement playing cards
                self.add_event(GameEvent::CardPlayed {
                    player_id: *player_id,
                    card_id: *card_id,
                });
            }
            crate::rules::GameAction::AttachEnergy {
                player_id,
                energy_id,
                pokemon_id,
            } => {
                if let Some(player) = self.players.get_mut(player_id)
                    && player.attach_energy(*energy_id, *pokemon_id) {
                        self.add_event(GameEvent::EnergyAttached {
                            player_id: *player_id,
                            energy_id: *energy_id,
                            pokemon_id: *pokemon_id,
                        });
                    }
            }
            crate::rules::GameAction::UseAttack {
                player_id,
                pokemon_id,
                attack_index,
            } => {
                // TODO: Implement attack logic
                self.add_event(GameEvent::AttackUsed {
                    player_id: *player_id,
                    pokemon_id: *pokemon_id,
                    attack_name: format!("Attack {}", attack_index),
                });
            }
            crate::rules::GameAction::Retreat {
                player_id: _,
                pokemon_id: _,
            } => {
                // TODO: Implement retreat logic
            }
            crate::rules::GameAction::EndTurn { player_id } => {
                self.add_event(GameEvent::TurnEnded {
                    player_id: *player_id,
                });
                // Move to next player
                self.current_player_index = (self.current_player_index + 1) % self.turn_order.len();
                self.turn_number += 1;
                // Reset turn-based flags for the next player
                if let Some(player) = self.players.get_mut(player_id) {
                    player.start_turn();
                }
            }
            crate::rules::GameAction::Pass { player_id: _ } => {
                // TODO: Implement pass logic
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::CardId;
    use crate::core::player::Player;
    use uuid::Uuid;

    #[test]
    fn test_shuffle_deck() {
        let mut game = Game::new();
        let player1_id = Uuid::new_v4();
        let player2_id = Uuid::new_v4();

        // Add players to the game
        game.players
            .insert(player1_id, Player::new("Player 1".to_string()));
        game.players
            .insert(player2_id, Player::new("Player 2".to_string()));

        // Set up decks for both players
        let player1_deck: Vec<CardId> = (0..10).map(|_| Uuid::new_v4()).collect();
        let player2_deck: Vec<CardId> = (0..10).map(|_| Uuid::new_v4()).collect();

        game.players
            .get_mut(&player1_id)
            .unwrap()
            .set_deck(player1_deck.clone());
        game.players
            .get_mut(&player2_id)
            .unwrap()
            .set_deck(player2_deck.clone());

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
        game.players
            .insert(player1_id, Player::new("Player 1".to_string()));
        game.players
            .insert(player2_id, Player::new("Player 2".to_string()));

        // Set up decks for both players
        let player1_deck: Vec<CardId> = (0..10).map(|_| Uuid::new_v4()).collect();
        let player2_deck: Vec<CardId> = (0..10).map(|_| Uuid::new_v4()).collect();

        game.players
            .get_mut(&player1_id)
            .unwrap()
            .set_deck(player1_deck.clone());
        game.players
            .get_mut(&player2_id)
            .unwrap()
            .set_deck(player2_deck.clone());

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

    #[test]
    fn test_execute_action_draw_card() {
        use crate::rules::{GameAction, RuleEngine};

        let mut game = Game::new();
        let player_id = Uuid::new_v4();
        let mut player = Player::new("Test Player".to_string());

        // Set up player's deck
        let card_ids: Vec<CardId> = (0..5).map(|_| Uuid::new_v4()).collect();
        player.set_deck(card_ids.clone());

        // Add player to game
        game.players.insert(player_id, player);
        game.turn_order.push(player_id);

        // Set up rule engine
        let rule_engine = RuleEngine::new();

        // Create draw card action
        let action = GameAction::DrawCard { player_id };

        // Execute the action
        assert!(game.execute_action(&rule_engine, &action).is_ok());

        // Check that a card was drawn
        let player = game.players.get(&player_id).unwrap();
        assert_eq!(player.hand.len(), 1);
        assert_eq!(player.deck.len(), 4);
    }

    #[test]
    fn test_execute_action_end_turn() {
        use crate::rules::{GameAction, RuleEngine};

        let mut game = Game::new();
        let player1_id = Uuid::new_v4();
        let player2_id = Uuid::new_v4();

        // Add players to game
        game.players
            .insert(player1_id, Player::new("Player 1".to_string()));
        game.players
            .insert(player2_id, Player::new("Player 2".to_string()));
        game.turn_order.push(player1_id);
        game.turn_order.push(player2_id);
        game.current_player_index = 0;
        game.turn_number = 1;

        // Set up rule engine
        let rule_engine = RuleEngine::new();

        // Create end turn action
        let action = GameAction::EndTurn {
            player_id: player1_id,
        };

        // Execute the action
        assert!(game.execute_action(&rule_engine, &action).is_ok());

        // Check that turn has advanced
        assert_eq!(game.current_player_index, 1);
        assert_eq!(game.turn_number, 2);
    }

    #[test]
    fn test_execute_action_attach_energy() {
        use crate::core::card::{Card, CardType, EnergyType};
        use crate::rules::{GameAction, RuleEngine};

        let mut game = Game::new();
        let player_id = Uuid::new_v4();
        let mut player = Player::new("Test Player".to_string());

        // Create a Pokemon card
        let pokemon_card = Card::new(
            "Pikachu".to_string(),
            CardType::Pokemon {
                species: "Pikachu".to_string(),
                hp: 60,
                retreat_cost: 1,
                weakness: None,
                resistance: None,
                stage: crate::core::card::EvolutionStage::Basic,
                evolves_from: None,
            },
            "Base Set".to_string(),
            "025".to_string(),
            crate::core::card::CardRarity::Common,
        );

        // Create an energy card
        let energy_card = Card::new(
            "Lightning Energy".to_string(),
            CardType::Energy {
                energy_type: EnergyType::Lightning,
                is_basic: true,
            },
            "Base Set".to_string(),
            "100".to_string(),
            crate::core::card::CardRarity::Common,
        );

        let pokemon_id = pokemon_card.id;
        let energy_id = energy_card.id;

        // Add cards to player's hand
        player.hand.push(pokemon_id);
        player.hand.push(energy_id);

        // Set pokemon as active
        player.set_active_pokemon(pokemon_id);

        // Add player to game
        game.players.insert(player_id, player);
        game.turn_order.push(player_id);

        // Add cards to game database
        game.card_database.insert(pokemon_id, pokemon_card);
        game.card_database.insert(energy_id, energy_card);

        // Set up rule engine
        let rule_engine = RuleEngine::new();

        // Create attach energy action
        let action = GameAction::AttachEnergy {
            player_id,
            energy_id,
            pokemon_id,
        };

        // Execute the action
        assert!(game.execute_action(&rule_engine, &action).is_ok());

        // Check that energy was attached
        let player = game.players.get(&player_id).unwrap();
        assert_eq!(player.get_attached_energy_count(pokemon_id), 1);
        assert!(!player.hand.contains(&energy_id)); // Energy should be removed from hand
    }
}
