//! Effect types and traits

use crate::core::card::CardId;
use crate::core::game::state::Game;
use crate::core::player::PlayerId;
use crate::{EffectTarget, EffectTrigger, TargetRequirement};
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
    CardMoved {
        card: CardId,
        from: String,
        to: String,
    },
    /// A special condition was applied
    SpecialConditionApplied { target: CardId, condition: String },
    /// A special condition was removed
    SpecialConditionRemoved { target: CardId, condition: String },
    /// Custom effect outcome
    Custom {
        description: String,
        data: HashMap<String, String>,
    },
}

/// Errors that can occur when applying effects
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectError {
    /// Invalid target for the effect
    InvalidTarget { reason: String },
    /// Insufficient resources (energy, cards, etc.)
    InsufficientResources {
        resource: String,
        required: u32,
        available: u32,
    },
    /// Effect cannot be applied due to game state
    InvalidGameState { reason: String },
    /// Effect requirements not met
    RequirementsNotMet { requirement: String },
    /// General effect error
    General { message: String },
}