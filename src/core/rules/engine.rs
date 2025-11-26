//! Rule engine core functionality

use crate::core::card::CardId;
use crate::core::game::state::Game;
use crate::core::player::PlayerId;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};

/// Trait for defining game rules
pub trait Rule: DynClone + Send + Sync {
    /// Name of the rule
    fn name(&self) -> &str;

    /// Check if a game action is valid according to this rule
    fn validate_action(&self, game: &Game, action: &GameAction) -> RuleResult;

    /// Apply any effects when this rule is triggered
    fn apply_effect(&self, game: &mut Game, action: &GameAction) -> RuleResult;
}

dyn_clone::clone_trait_object!(Rule);

/// Result of a rule validation or application
pub type RuleResult = Result<(), RuleViolation>;

/// Represents a violation of a game rule
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleViolation {
    /// Name of the rule that was violated
    pub rule_name: String,
    /// Description of the violation
    pub message: String,
    /// Severity of the violation
    pub severity: ViolationSeverity,
}

/// Severity levels for rule violations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Warning - action is allowed but discouraged
    Warning,
    /// Error - action is not allowed
    Error,
    /// Fatal - game state is corrupted
    Fatal,
}

/// Represents an action that can be taken in the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameAction {
    /// Draw a card
    DrawCard { player_id: PlayerId },
    /// Play a card from hand
    PlayCard {
        player_id: PlayerId,
        card_id: CardId,
        target: Option<CardId>,
    },
    /// Attach energy to a Pokemon
    AttachEnergy {
        player_id: PlayerId,
        energy_id: CardId,
        pokemon_id: CardId,
    },
    /// Use a Pokemon's attack
    UseAttack {
        player_id: PlayerId,
        pokemon_id: CardId,
        attack_index: usize,
    },
    /// Retreat a Pokemon
    Retreat {
        player_id: PlayerId,
        pokemon_id: CardId,
    },
    /// End turn
    EndTurn { player_id: PlayerId },
    /// Pass turn without action
    Pass { player_id: PlayerId },
}

/// Main rule engine that manages and applies all rules
#[derive(Clone)]
pub struct RuleEngine {
    /// List of active rules
    rules: Vec<Box<dyn Rule>>,
    /// Rule configuration
    config: RuleConfig,
}

/// Configuration for the rule engine
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleConfig {
    /// Whether to stop on first rule violation
    pub stop_on_first_violation: bool,
    /// Whether to apply rule effects automatically
    pub auto_apply_effects: bool,
    /// Minimum severity level to report
    pub min_severity: ViolationSeverity,
}

impl Default for RuleConfig {
    fn default() -> Self {
        Self {
            stop_on_first_violation: false,
            auto_apply_effects: true,
            min_severity: ViolationSeverity::Warning,
        }
    }
}

impl RuleEngine {
    /// Create a new rule engine with default configuration
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            config: RuleConfig::default(),
        }
    }

    /// Create a new rule engine with custom configuration
    pub fn with_config(config: RuleConfig) -> Self {
        Self {
            rules: Vec::new(),
            config,
        }
    }

    /// Add a rule to the engine
    pub fn add_rule<R: Rule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }

    /// Remove a rule by name
    pub fn remove_rule(&mut self, rule_name: &str) {
        self.rules.retain(|rule| rule.name() != rule_name);
    }

    /// Validate an action against all rules
    pub fn validate_action(&self, game: &Game, action: &GameAction) -> Vec<RuleViolation> {
        let mut violations = Vec::new();

        for rule in &self.rules {
            match rule.validate_action(game, action) {
                Ok(()) => continue,
                Err(violation) => {
                    if violation.severity as u8 >= self.config.min_severity as u8 {
                        violations.push(violation);

                        if self.config.stop_on_first_violation {
                            break;
                        }
                    }
                }
            }
        }

        violations
    }

    /// Apply an action if it passes all rule validations
    pub fn apply_action(
        &self,
        game: &mut Game,
        action: &GameAction,
    ) -> Result<(), Vec<RuleViolation>> {
        // First validate the action
        let violations = self.validate_action(game, action);

        // Check if there are any blocking violations
        let has_errors = violations.iter().any(|v| {
            matches!(
                v.severity,
                ViolationSeverity::Error | ViolationSeverity::Fatal
            )
        });

        if has_errors {
            return Err(violations);
        }

        // Apply rule effects if auto-apply is enabled
        if self.config.auto_apply_effects {
            for rule in &self.rules {
                if let Err(violation) = rule.apply_effect(game, action) {
                    return Err(vec![violation]);
                }
            }
        }

        Ok(())
    }

    /// Get all rule names
    pub fn get_rule_names(&self) -> Vec<String> {
        self.rules
            .iter()
            .map(|rule| rule.name().to_string())
            .collect()
    }

    /// Check if a specific rule is active
    pub fn has_rule(&self, rule_name: &str) -> bool {
        self.rules.iter().any(|rule| rule.name() == rule_name)
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}