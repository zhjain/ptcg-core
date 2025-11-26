//! 训练家卡效果

use crate::core::effects::{Effect, EffectId, EffectContext, EffectOutcome, EffectError, BaseEffect};
use crate::core::game::state::Game;
use crate::core::card::{CardId, TrainerType};
use std::collections::HashMap;

/// 训练家卡效果实现
#[derive(Clone)]
pub struct TrainerEffect {
    base: BaseEffect,
    #[allow(dead_code)]
    trainer_type: TrainerType,
    #[allow(dead_code)]
    one_time_effect: bool,
    target_requirements: Vec<crate::TargetRequirement>,
}

impl TrainerEffect {
    pub fn new(
        name: String,
        description: String,
        trainer_type: TrainerType,
        one_time_effect: bool,
        target_requirements: Vec<crate::TargetRequirement>,
    ) -> Self {
        Self {
            base: BaseEffect::new(name, description),
            trainer_type,
            one_time_effect,
            target_requirements,
        }
    }
}

impl Effect for TrainerEffect {
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
        // 在实际实现中，这会应用训练家卡的效果
        // 现在，我们只返回一个通用结果
        Ok(vec![EffectOutcome::Custom {
            description: format!("应用了训练家效果：{}", self.name()),
            data: HashMap::new(),
        }])
    }

    fn triggers(&self) -> Vec<crate::EffectTrigger> {
        vec![crate::EffectTrigger::OnPlay]
    }

    fn target_requirements(&self) -> Vec<crate::TargetRequirement> {
        self.target_requirements.clone()
    }
    
    fn on_attach(&self, _game: &mut Game, _card_id: CardId) -> Result<Vec<EffectOutcome>, EffectError> {
        // 当附加训练家效果时，我们可能想要做一些特殊的事情
        // 对于一次性效果，我们可能会立即应用它们
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::TrainerType;

    #[test]
    fn test_trainer_effect_creation() {
        let trainer_effect = TrainerEffect::new(
            "大木博士".to_string(),
            "丢弃你的手牌，然后抽7张卡。".to_string(),
            TrainerType::Supporter,
            true, // 一次性效果
            vec![],
        );

        assert_eq!(trainer_effect.name(), "大木博士");
        assert_eq!(trainer_effect.triggers(), vec![crate::EffectTrigger::OnPlay]);
        assert_eq!(trainer_effect.trainer_type, TrainerType::Supporter);
    }
}