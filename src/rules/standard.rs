//! Standard PTCG rules implementation

use crate::core::game::state::Game;
use crate::core::player::PlayerId;
use crate::rules::{Rule, RuleEngine, RuleResult, RuleViolation, ViolationSeverity, GameAction};

/// Standard PTCG rules implementation
pub struct StandardRules;

impl StandardRules {
    /// Create a rule engine with standard PTCG rules
    pub fn create_engine() -> RuleEngine {
        let mut engine = RuleEngine::new();

        engine.add_rule(TurnOrderRule);
        engine.add_rule(HandLimitRule);
        engine.add_rule(EnergyAttachmentRule);

        engine
    }
}

/// Rule: Players must take actions only on their turn
#[derive(Clone)]
pub struct TurnOrderRule;

impl Rule for TurnOrderRule {
    fn name(&self) -> &str {
        "TurnOrder"
    }

    fn validate_action(&self, game: &Game, action: &GameAction) -> RuleResult {
        let action_player_id = match action {
            GameAction::DrawCard { player_id, .. }
            | GameAction::PlayCard { player_id, .. }
            | GameAction::AttachEnergy { player_id, .. }
            | GameAction::UseAttack { player_id, .. }
            | GameAction::Retreat { player_id, .. }
            | GameAction::EndTurn { player_id, .. }
            | GameAction::Pass { player_id, .. } => *player_id,
        };

        if !game.is_player_turn(action_player_id) {
            return Err(RuleViolation {
                rule_name: self.name().to_string(),
                message: "Not your turn".to_string(),
                severity: ViolationSeverity::Error,
            });
        }

        Ok(())
    }

    fn apply_effect(&self, _game: &mut Game, _action: &GameAction) -> RuleResult {
        Ok(())
    }
}

/// Rule: Hand size limit (typically unlimited in PTCG, but can be configured)
#[derive(Clone)]
pub struct HandLimitRule;

impl Rule for HandLimitRule {
    fn name(&self) -> &str {
        "HandLimit"
    }

    fn validate_action(&self, game: &Game, action: &GameAction) -> RuleResult {
        if let GameAction::DrawCard { player_id } = action
            && let Some(player) = game.get_player(*player_id)
            && let Some(max_hand_size) = game.rules.max_hand_size
            && player.hand.len() >= max_hand_size as usize
        {
            return Err(RuleViolation {
                rule_name: self.name().to_string(),
                message: format!("Hand size limit exceeded ({})", max_hand_size),
                severity: ViolationSeverity::Error,
            });
        }
        Ok(())
    }

    fn apply_effect(&self, _game: &mut Game, _action: &GameAction) -> RuleResult {
        Ok(())
    }
}

/// Rule: Energy attachment limitations (one per turn)
#[derive(Clone)]
pub struct EnergyAttachmentRule;

impl Rule for EnergyAttachmentRule {
    fn name(&self) -> &str {
        "EnergyAttachment"
    }

    fn validate_action(&self, game: &Game, action: &GameAction) -> RuleResult {
        if let GameAction::AttachEnergy {
            player_id,
            energy_id,
            pokemon_id,
        } = action
            && let Some(player) = game.get_player(*player_id)
        {
            // Check if energy card is in hand
            if !player.hand.contains(energy_id) {
                return Err(RuleViolation {
                    rule_name: self.name().to_string(),
                    message: "Energy card not in hand".to_string(),
                    severity: ViolationSeverity::Error,
                });
            }

            // Check if target Pokemon exists
            if Some(*pokemon_id) != player.active_pokemon && !player.bench.contains(pokemon_id) {
                return Err(RuleViolation {
                    rule_name: self.name().to_string(),
                    message: "Target Pokemon not found".to_string(),
                    severity: ViolationSeverity::Error,
                });
            }

            // Check if energy card is actually an energy
            if let Some(card) = game.get_card(*energy_id)
                && !card.is_energy()
            {
                return Err(RuleViolation {
                    rule_name: self.name().to_string(),
                    message: "Card is not an energy".to_string(),
                    severity: ViolationSeverity::Error,
                });
            }
        }
        Ok(())
    }

    fn apply_effect(&self, _game: &mut Game, _action: &GameAction) -> RuleResult {
        Ok(())
    }
}