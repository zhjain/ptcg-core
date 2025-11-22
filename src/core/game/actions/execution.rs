//! Game action execution

use crate::core::game::state::{Game, GameEvent};

impl Game {
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