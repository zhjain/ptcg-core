//! Event handlers

use crate::core::events::GameEvent;

/// Trait for event handlers
pub trait EventHandler: Send + Sync {
    /// Get the handler's name
    fn name(&self) -> &str;

    /// Handle an event
    fn handle_event(&self, event: &GameEvent);
}

/// Console event handler that prints events to stdout
pub struct ConsoleEventHandler {
    /// Whether to show timestamps
    show_timestamps: bool,
}

impl ConsoleEventHandler {
    /// Create a new console event handler
    pub fn new(show_timestamps: bool) -> Self {
        Self { show_timestamps }
    }
}

impl EventHandler for ConsoleEventHandler {
    fn name(&self) -> &str {
        "ConsoleHandler"
    }

    fn handle_event(&self, event: &GameEvent) {
        match event {
            GameEvent::GameStarted { timestamp, players } => {
                if self.show_timestamps {
                    println!("[{}] Game started with {} players", timestamp, players.len());
                } else {
                    println!("Game started with {} players", players.len());
                }
            }
            GameEvent::TurnStarted { timestamp, player_id, turn_number } => {
                if self.show_timestamps {
                    println!("[{}] Turn {} started for player {:?}", timestamp, turn_number, player_id);
                } else {
                    println!("Turn {} started for player {:?}", turn_number, player_id);
                }
            }
            GameEvent::CardDrawn { timestamp, player_id, card_id } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} drew a card: {:?}", timestamp, player_id, card_id);
                } else {
                    println!("Player {:?} drew a card: {:?}", player_id, card_id);
                }
            }
            GameEvent::CardPlayed { timestamp, player_id, card_id } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} played card: {:?}", timestamp, player_id, card_id);
                } else {
                    println!("Player {:?} played card: {:?}", player_id, card_id);
                }
            }
            GameEvent::PokemonBenched { timestamp, player_id, card_id } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} benched Pokemon: {:?}", timestamp, player_id, card_id);
                } else {
                    println!("Player {:?} benched Pokemon: {:?}", player_id, card_id);
                }
            }
            GameEvent::EnergyAttached { timestamp, player_id, energy_id, pokemon_id } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} attached energy {:?} to Pokemon {:?}", timestamp, player_id, energy_id, pokemon_id);
                } else {
                    println!("Player {:?} attached energy {:?} to Pokemon {:?}", player_id, energy_id, pokemon_id);
                }
            }
            GameEvent::AttackUsed { timestamp, player_id, pokemon_id, attack_name } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} used attack '{}' with Pokemon {:?}", timestamp, player_id, attack_name, pokemon_id);
                } else {
                    println!("Player {:?} used attack '{}' with Pokemon {:?}", player_id, attack_name, pokemon_id);
                }
            }
            GameEvent::DamageDealt { timestamp, player_id, pokemon_id, damage } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} dealt {} damage to Pokemon {:?}", timestamp, player_id, damage, pokemon_id);
                } else {
                    println!("Player {:?} dealt {} damage to Pokemon {:?}", player_id, damage, pokemon_id);
                }
            }
            GameEvent::PokemonKnockedOut { timestamp, player_id, pokemon_id } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} knocked out Pokemon {:?}", timestamp, player_id, pokemon_id);
                } else {
                    println!("Player {:?} knocked out Pokemon {:?}", player_id, pokemon_id);
                }
            }
            GameEvent::PrizeTaken { timestamp, player_id } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} took a prize card", timestamp, player_id);
                } else {
                    println!("Player {:?} took a prize card", player_id);
                }
            }
            GameEvent::DeckShuffled { timestamp, player_id } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} shuffled their deck", timestamp, player_id);
                } else {
                    println!("Player {:?} shuffled their deck", player_id);
                }
            }
            GameEvent::TurnEnded { timestamp, player_id } => {
                if self.show_timestamps {
                    println!("[{}] Player {:?} ended their turn", timestamp, player_id);
                } else {
                    println!("Player {:?} ended their turn", player_id);
                }
            }
            GameEvent::GameEnded { timestamp, winner } => {
                if self.show_timestamps {
                    println!("[{}] Game ended. Winner: {:?}", timestamp, winner);
                } else {
                    println!("Game ended. Winner: {:?}", winner);
                }
            }
        }
    }
}