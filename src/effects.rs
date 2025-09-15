//! Card effect system for implementing custom card behaviors
//! 
//! This module provides a flexible system for defining and applying
//! card effects, triggers, and conditions.

use crate::core::{Game, Player, Card, CardId};
use crate::core::player::PlayerId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use dyn_clone::DynClone;

/// Unique identifier for an effect
pub type EffectId = Uuid;

/// Trait for implementing card effects
pub trait Effect: DynClone + Send + Sync {
    /// Get the effect's unique identifier
    fn id(&self) -> EffectId;
    
    /// Get the effect's name
    fn name(&self) -> &str;
    
    /// Get the effect's description
    fn description(&self) -> &str;
    
    /// Check if this effect can be applied in the current game state
    fn can_apply(&self, game: &Game, context: &EffectContext) -> bool;
    
    /// Apply the effect to the game state
    fn apply(&self, game: &mut Game, context: &EffectContext) -> EffectResult;
    
    /// Get the effect's trigger conditions
    fn triggers(&self) -> Vec<EffectTrigger>;
    
    /// Get the effect's target requirements
    fn target_requirements(&self) -> Vec<TargetRequirement>;
}

dyn_clone::clone_trait_object!(Effect);

/// Context information for effect application
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectContext {
    /// The card that owns this effect
    pub source_card: CardId,
    /// The player who controls the source card
    pub controller: PlayerId,
    /// The target of the effect (if any)
    pub target: Option<EffectTarget>,
    /// Additional parameters for the effect
    pub parameters: HashMap<String, String>,
    /// The trigger that caused this effect to activate
    pub trigger: Option<EffectTrigger>,
}

/// Result of applying an effect
pub type EffectResult = Result<Vec<EffectOutcome>, EffectError>;

/// Possible outcomes of an effect
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectOutcome {
    /// Damage was dealt
    DamageDealt { target: CardId, amount: u32 },
    /// Healing was applied
    Healing { target: CardId, amount: u32 },
    /// Cards were drawn
    CardsDrawn { player: PlayerId, count: u32 },
    /// Energy was attached
    EnergyAttached { energy: CardId, target: CardId },
    /// A card was moved
    CardMoved { card: CardId, from: String, to: String },
    /// A special condition was applied
    SpecialConditionApplied { target: CardId, condition: String },
    /// A special condition was removed
    SpecialConditionRemoved { target: CardId, condition: String },
    /// Custom effect outcome
    Custom { description: String, data: HashMap<String, String> },
}

/// Errors that can occur when applying effects
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectError {
    /// Invalid target for the effect
    InvalidTarget { reason: String },
    /// Insufficient resources (energy, cards, etc.)
    InsufficientResources { resource: String, required: u32, available: u32 },
    /// Effect cannot be applied due to game state
    InvalidGameState { reason: String },
    /// Effect requirements not met
    RequirementsNotMet { requirement: String },
    /// General effect error
    General { message: String },
}

/// Different types of effect triggers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectTrigger {
    /// Triggered when the card is played
    OnPlay,
    /// Triggered when the Pokemon comes into play
    OnEnterPlay,
    /// Triggered when the Pokemon leaves play
    OnLeavePlay,
    /// Triggered when the Pokemon is knocked out
    OnKnockOut,
    /// Triggered at the beginning of each turn
    OnTurnStart,
    /// Triggered at the end of each turn
    OnTurnEnd,
    /// Triggered when the Pokemon takes damage
    OnTakeDamage,
    /// Triggered when the Pokemon deals damage
    OnDealDamage,
    /// Triggered when an attack is used
    OnAttack,
    /// Triggered when energy is attached
    OnEnergyAttach,
    /// Triggered when a card is drawn
    OnCardDraw,
    /// Triggered by a specific game event
    OnGameEvent { event_type: String },
    /// Manually triggered (by player action)
    Manual,
    /// Triggered once when conditions are met
    Once { condition: String },
}

/// Different types of effect targets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectTarget {
    /// No target required
    None,
    /// Target is the source card itself
    Self_,
    /// Target is a specific card
    Card(CardId),
    /// Target is a specific player
    Player(PlayerId),
    /// Target is all Pokemon of a player
    AllPlayerPokemon(PlayerId),
    /// Target is all Pokemon in play
    AllPokemon,
    /// Target is the active Pokemon of a player
    ActivePokemon(PlayerId),
    /// Target is a random card/Pokemon
    Random { filter: String },
    /// Target chosen by player
    Choice { options: Vec<CardId> },
}

/// Requirements for effect targets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetRequirement {
    /// Must be a Pokemon card
    Pokemon,
    /// Must be an Energy card
    Energy,
    /// Must be a Trainer card
    Trainer,
    /// Must be in play (on field)
    InPlay,
    /// Must be in hand
    InHand,
    /// Must be in discard pile
    InDiscard,
    /// Must be owned by specific player
    OwnedBy(PlayerId),
    /// Must have specific energy type attached
    HasEnergyType(crate::core::card::EnergyType),
    /// Must have at least specified HP
    MinHP(u32),
    /// Must have at least specified damage
    MinDamage(u32),
    /// Custom requirement
    Custom(String),
}

/// Effect manager for handling all effect-related operations
pub struct EffectManager {
    /// All registered effects
    effects: HashMap<EffectId, Box<dyn Effect>>,
    /// Effects currently active in the game
    active_effects: HashMap<CardId, Vec<EffectId>>,
    /// Triggered effects waiting to resolve
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
            .or_insert_with(Vec::new)
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
    target_type: EffectTarget,
}

impl DamageEffect {
    pub fn new(name: String, damage: u32, target_type: EffectTarget) -> Self {
        Self {
            id: Uuid::new_v4(),
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

    fn apply(&self, game: &mut Game, context: &EffectContext) -> EffectResult {
        let target_card = match &context.target {
            Some(EffectTarget::Card(card_id)) => *card_id,
            Some(EffectTarget::ActivePokemon(player_id)) => {
                if let Some(player) = game.get_player(*player_id) {
                    if let Some(active) = player.active_pokemon {
                        active
                    } else {
                        return Err(EffectError::InvalidTarget { 
                            reason: "No active Pokemon".to_string() 
                        });
                    }
                } else {
                    return Err(EffectError::InvalidTarget { 
                        reason: "Player not found".to_string() 
                    });
                }
            }
            _ => return Err(EffectError::InvalidTarget { 
                reason: "Invalid target type".to_string() 
            }),
        };

        // Apply damage to the target
        if let Some(player) = game.players.values_mut().find(|p| 
            Some(target_card) == p.active_pokemon || p.bench.contains(&target_card)
        ) {
            player.add_damage(target_card, self.damage);
            Ok(vec![EffectOutcome::DamageDealt { 
                target: target_card, 
                amount: self.damage 
            }])
        } else {
            Err(EffectError::InvalidTarget { 
                reason: "Target Pokemon not found".to_string() 
            })
        }
    }

    fn triggers(&self) -> Vec<EffectTrigger> {
        vec![EffectTrigger::OnAttack]
    }

    fn target_requirements(&self) -> Vec<TargetRequirement> {
        vec![TargetRequirement::Pokemon, TargetRequirement::InPlay]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_manager_creation() {
        let manager = EffectManager::new();
        assert_eq!(manager.effects.len(), 0);
        assert_eq!(manager.active_effects.len(), 0);
    }

    #[test]
    fn test_register_effect() {
        let mut manager = EffectManager::new();
        let effect = DamageEffect::new("Test Damage".to_string(), 30, EffectTarget::None);
        let id = manager.register_effect(effect);
        
        assert!(manager.effects.contains_key(&id));
    }

    #[test]
    fn test_damage_effect() {
        let effect = DamageEffect::new("Thunder Shock".to_string(), 30, EffectTarget::None);
        assert_eq!(effect.name(), "Thunder Shock");
        assert_eq!(effect.triggers(), vec![EffectTrigger::OnAttack]);
    }
}