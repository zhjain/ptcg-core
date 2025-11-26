//! 特殊能量卡效果

use crate::core::effects::{Effect, EffectId, EffectContext, EffectOutcome, EffectError, BaseEffect};
use crate::core::game::state::Game;
use crate::core::player::PlayerId;
use crate::core::card::{CardId, EnergyType};
use std::collections::HashMap;

/// 特殊能量效果实现
#[derive(Clone)]
pub struct SpecialEnergyEffect {
    base: BaseEffect,
    energy_type: EnergyType,
    attachment_effect: Option<Box<dyn Effect>>,
    persistent_effect: Option<Box<dyn Effect>>,
    target_requirements: Vec<crate::TargetRequirement>,
}

impl SpecialEnergyEffect {
    pub fn new(
        name: String,
        description: String,
        energy_type: EnergyType,
        attachment_effect: Option<Box<dyn Effect>>,
        persistent_effect: Option<Box<dyn Effect>>,
        target_requirements: Vec<crate::TargetRequirement>,
    ) -> Self {
        Self {
            base: BaseEffect::new(name, description),
            energy_type,
            attachment_effect,
            persistent_effect,
            target_requirements,
        }
    }
}

impl Effect for SpecialEnergyEffect {
    fn id(&self) -> EffectId {
        self.base.id
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    fn can_apply(&self, _game: &Game, _context: &EffectContext) -> bool {
        true
    }

    fn apply(&self, _game: &mut Game, _context: &EffectContext) -> Result<Vec<EffectOutcome>, EffectError> {
        // 对于特殊能量卡，主要效果可能是提供能量类型
        // 附加效果将由attachment_effect或persistent_effect处理
        Ok(vec![EffectOutcome::Custom {
            description: format!("提供了特殊能量：{:?}", self.energy_type),
            data: HashMap::new(),
        }])
    }

    fn triggers(&self) -> Vec<crate::EffectTrigger> {
        vec![crate::EffectTrigger::OnEnergyAttach]
    }

    fn target_requirements(&self) -> Vec<crate::TargetRequirement> {
        self.target_requirements.clone()
    }
    
    fn on_attach(&self, game: &mut Game, card_id: CardId) -> Result<Vec<EffectOutcome>, EffectError> {
        let mut outcomes = vec![];
        
        // 如果存在附加效果则应用它
        if let Some(ref attachment_effect) = self.attachment_effect {
            let context = EffectContext {
                source_card: card_id,
                controller: PlayerId::new_v4(), // 实际上，这将是实际的玩家
                target: None,
                parameters: HashMap::new(),
                trigger: Some(crate::EffectTrigger::OnEnergyAttach),
            };
            
            match attachment_effect.apply(game, &context) {
                Ok(effect_outcomes) => outcomes.extend(effect_outcomes),
                Err(e) => return Err(e),
            }
        }
        
        Ok(outcomes)
    }
    
    fn on_turn_start(&self, game: &mut Game, player_id: PlayerId) -> Result<Vec<EffectOutcome>, EffectError> {
        // 如果存在持续效果，则在每回合开始时应用它
        if let Some(ref persistent_effect) = self.persistent_effect {
            let context = EffectContext {
                source_card: CardId::new_v4(), // 实际上，这将是实际的卡牌ID
                controller: player_id,
                target: None,
                parameters: HashMap::new(),
                trigger: Some(crate::EffectTrigger::OnTurnStart),
            };
            
            persistent_effect.apply(game, &context)
        } else {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::EnergyType;

    #[test]
    fn test_special_energy_effect_creation() {
        let energy_effect = SpecialEnergyEffect::new(
            "双重无色能量".to_string(),
            "提供无色无色能量。如果附加此卡的宝可梦使用攻击，则该攻击对活跃宝可梦造成20点额外伤害。".to_string(),
            EnergyType::Colorless,
            None,
            None,
            vec![crate::TargetRequirement::Pokemon],
        );

        assert_eq!(energy_effect.name(), "双重无色能量");
        assert_eq!(energy_effect.triggers(), vec![crate::EffectTrigger::OnEnergyAttach]);
        assert_eq!(energy_effect.energy_type, EnergyType::Colorless);
    }
}