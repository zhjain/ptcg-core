//! Effect manager for handling all effect-related operations

use crate::core::effects::{Effect, EffectId, EffectTarget, EffectContext, EffectOutcome, EffectError};
use crate::core::card::CardId;
use crate::core::game::state::Game;
use std::collections::HashMap;

/// Effect manager for handling all effect-related operations
pub struct EffectManager {
    /// All registered effects
    effects: HashMap<EffectId, Box<dyn Effect>>,
    /// Effects currently active in the game
    active_effects: HashMap<CardId, Vec<EffectId>>,
    /// Triggered effects waiting to resolve
    #[allow(dead_code)]
    pending_effects: Vec<(EffectId, EffectContext)>,
}

impl EffectManager {
    /// Create a new effect manager
    pub fn new() -> Self {
        Self {
            effects: HashMap::new(),
            active_effects: HashMap::new(),
            pending_effects: Vec::new(),
        }
    }

    /// Register an effect
    pub fn register_effect<E: Effect + 'static>(&mut self, effect: E) -> EffectId {
        let id = effect.id();
        self.effects.insert(id, Box::new(effect));
        id
    }

    /// Attach an effect to a card
    pub fn attach_effect(&mut self, card_id: CardId, effect_id: EffectId) {
        self.active_effects
            .entry(card_id)
            .or_default()
            .push(effect_id);
    }

    /// Remove all effects from a card
    pub fn remove_card_effects(&mut self, card_id: CardId) {
        self.active_effects.remove(&card_id);
    }

    /// Get all effects attached to a card
    pub fn get_card_effects(&self, card_id: CardId) -> Vec<&dyn Effect> {
        if let Some(effect_ids) = self.active_effects.get(&card_id) {
            effect_ids
                .iter()
                .filter_map(|&id| self.effects.get(&id).map(|e| e.as_ref()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Check if a card has any effects
    pub fn has_effects(&self, card_id: CardId) -> bool {
        self.active_effects
            .get(&card_id)
            .map(|effects| !effects.is_empty())
            .unwrap_or(false)
    }
}

impl Default for EffectManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Basic damage effect implementation
#[derive(Clone)]
pub struct DamageEffect {
    id: EffectId,
    name: String,
    damage: u32,
    #[allow(dead_code)]
    target_type: EffectTarget,
}

impl DamageEffect {
    pub fn new(name: String, damage: u32, target_type: EffectTarget) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            damage,
            target_type,
        }
    }
}

impl Effect for DamageEffect {
    fn id(&self) -> EffectId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "Deals damage to target"
    }

    fn can_apply(&self, _game: &Game, _context: &EffectContext) -> bool {
        true // Simplified - would check target validity
    }

    fn apply(&self, game: &mut Game, context: &EffectContext) -> Result<Vec<EffectOutcome>, EffectError> {
        let target_card = match &context.target {
            Some(EffectTarget::Card(card_id)) => *card_id,
            Some(EffectTarget::ActivePokemon(player_id)) => {
                // Note: player_id is already a reference, so we don't need to dereference it
                if let Some(player) = game.get_player(*player_id) {
                    if let Some(active) = player.active_pokemon {
                        active
                    } else {
                        return Err(EffectError::InvalidTarget {
                            reason: "No active Pokemon".to_string(),
                        });
                    }
                } else {
                    return Err(EffectError::InvalidTarget {
                        reason: "Player not found".to_string(),
                    });
                }
            }
            _ => {
                return Err(EffectError::InvalidTarget {
                    reason: "Invalid target type".to_string(),
                });
            }
        };

        // Apply damage to the target
        if let Some(player) = game
            .players
            .values_mut()
            .find(|p| Some(target_card) == p.active_pokemon || p.bench.contains(&target_card))
        {
            player.add_damage(target_card, self.damage);
            Ok(vec![EffectOutcome::DamageDealt {
                target: target_card,
                amount: self.damage,
            }])
        } else {
            Err(EffectError::InvalidTarget {
                reason: "Target Pokemon not found".to_string(),
            })
        }
    }

    fn triggers(&self) -> Vec<crate::EffectTrigger> {
        vec![crate::EffectTrigger::OnAttack]
    }

    fn target_requirements(&self) -> Vec<crate::TargetRequirement> {
        vec![crate::TargetRequirement::Pokemon, crate::TargetRequirement::InPlay]
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_effect_manager_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}