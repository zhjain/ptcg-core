//! Player actions and operations

use crate::core::{
    card::CardId,
    player::{Player, ConditionEffect},
};
use crate::{SpecialCondition, SpecialConditionInstance};

impl Player {
    /// Add a special condition to a Pokemon
    pub fn add_special_condition(
        &mut self,
        pokemon_id: CardId,
        condition: SpecialCondition,
        duration: i32,
        _current_turn: u32,
    ) {
        let instance = SpecialConditionInstance {
            condition,
            duration,
            applied_turn: _current_turn,
            data: std::collections::HashMap::new(),
        };

        self.special_conditions
            .entry(pokemon_id)
            .or_default()
            .push(instance);
    }

    /// Add a special condition with additional data
    pub fn add_special_condition_with_data(
        &mut self,
        pokemon_id: CardId,
        condition: SpecialCondition,
        duration: i32,
        current_turn: u32,
        data: std::collections::HashMap<String, String>,
    ) {
        let instance = SpecialConditionInstance {
            condition,
            duration,
            applied_turn: current_turn,
            data,
        };

        self.special_conditions
            .entry(pokemon_id)
            .or_default()
            .push(instance);
    }

    /// Remove a specific type of special condition from a Pokemon
    pub fn remove_special_condition_type(
        &mut self,
        pokemon_id: CardId,
        condition_type: &SpecialCondition,
    ) {
        if let Some(conditions) = self.special_conditions.get_mut(&pokemon_id) {
            conditions.retain(|instance| {
                std::mem::discriminant(&instance.condition)
                    != std::mem::discriminant(condition_type)
            });
            if conditions.is_empty() {
                self.special_conditions.remove(&pokemon_id);
            }
        }
    }

    /// Remove all special conditions from a Pokemon
    pub fn clear_special_conditions(&mut self, pokemon_id: CardId) {
        self.special_conditions.remove(&pokemon_id);
    }

    /// Check if a Pokemon has a specific type of special condition
    pub fn has_special_condition_type(
        &self,
        pokemon_id: CardId,
        condition_type: &SpecialCondition,
    ) -> bool {
        self.special_conditions
            .get(&pokemon_id)
            .map(|conditions| {
                conditions.iter().any(|instance| {
                    std::mem::discriminant(&instance.condition)
                        == std::mem::discriminant(condition_type)
                })
            })
            .unwrap_or(false)
    }

    /// Get all special conditions for a Pokemon
    pub fn get_special_conditions(&self, pokemon_id: CardId) -> Vec<SpecialConditionInstance> {
        self.special_conditions
            .get(&pokemon_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Update special condition durations and apply effects
    pub fn update_special_conditions(&mut self, _current_turn: u32) -> Vec<ConditionEffect> {
        let mut effects = Vec::new();

        for (pokemon_id, conditions) in self.special_conditions.iter_mut() {
            let mut to_remove = Vec::new();

            for (index, condition) in conditions.iter_mut().enumerate() {
                // Apply condition effects
                match &condition.condition {
                    SpecialCondition::Poisoned { damage_per_turn } => {
                        effects.push(ConditionEffect::Damage {
                            pokemon_id: *pokemon_id,
                            amount: *damage_per_turn,
                            source: "Poison".to_string(),
                        });
                    }
                    SpecialCondition::Burned { damage_per_turn } => {
                        effects.push(ConditionEffect::Damage {
                            pokemon_id: *pokemon_id,
                            amount: *damage_per_turn,
                            source: "Burn".to_string(),
                        });
                        // Burn has a chance to be removed
                        effects.push(ConditionEffect::CoinFlip {
                            pokemon_id: *pokemon_id,
                            condition: "Burn removal".to_string(),
                            on_success: "Remove burn condition".to_string(),
                        });
                    }
                    SpecialCondition::Asleep => {
                        effects.push(ConditionEffect::CoinFlip {
                            pokemon_id: *pokemon_id,
                            condition: "Wake up".to_string(),
                            on_success: "Remove sleep condition".to_string(),
                        });
                    }
                    _ => {} // Other conditions don't have automatic effects
                }

                // Update duration
                if condition.duration > 0 {
                    condition.duration -= 1;
                    if condition.duration == 0 {
                        to_remove.push(index);
                        effects.push(ConditionEffect::ConditionRemoved {
                            pokemon_id: *pokemon_id,
                            condition: format!("{:?}", condition.condition),
                        });
                    }
                }
            }

            // Remove expired conditions
            for &index in to_remove.iter().rev() {
                conditions.remove(index);
            }
        }

        // Clean up empty condition lists
        self.special_conditions
            .retain(|_, conditions| !conditions.is_empty());

        effects
    }

    /// Check if a Pokemon can attack (not paralyzed or asleep)
    pub fn can_pokemon_attack(&self, pokemon_id: CardId) -> bool {
        if let Some(conditions) = self.special_conditions.get(&pokemon_id) {
            for condition in conditions {
                match &condition.condition {
                    SpecialCondition::Paralyzed | SpecialCondition::Asleep => return false,
                    _ => {}
                }
            }
        }
        true
    }

    /// Check if a Pokemon can retreat (not trapped)
    pub fn can_pokemon_retreat(&self, pokemon_id: CardId) -> bool {
        if let Some(conditions) = self.special_conditions.get(&pokemon_id) {
            for condition in conditions {
                if matches!(condition.condition, SpecialCondition::Trapped) {
                    return false;
                }
            }
        }
        true
    }
}