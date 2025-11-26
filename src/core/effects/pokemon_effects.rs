//! 宝可梦专用效果和能力

use crate::core::effects::{Effect, EffectId, EffectContext, EffectOutcome, EffectError, BaseEffect, AbilityType};
use crate::core::game::state::Game;
use crate::core::card::CardId;
use std::collections::HashMap;

/// 宝可梦能力效果实现
#[derive(Clone)]
pub struct PokemonAbilityEffect {
    base: BaseEffect,
    #[allow(dead_code)]
    ability_type: AbilityType,
    trigger_conditions: Vec<crate::EffectTrigger>,
    target_requirements: Vec<crate::TargetRequirement>,
}

impl PokemonAbilityEffect {
    pub fn new(
        name: String, 
        description: String, 
        ability_type: AbilityType,
        trigger_conditions: Vec<crate::EffectTrigger>,
        target_requirements: Vec<crate::TargetRequirement>,
    ) -> Self {
        Self {
            base: BaseEffect::new(name, description),
            ability_type,
            trigger_conditions,
            target_requirements,
        }
    }
}

impl Effect for PokemonAbilityEffect {
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
        // 在实际实现中，这会根据当前游戏状态和上下文检查能力是否可以应用
        true
    }

    fn apply(&self, _game: &mut Game, _context: &EffectContext) -> Result<Vec<EffectOutcome>, EffectError> {
        // 在实际实现中，这会将能力的效果应用到游戏中
        Ok(vec![EffectOutcome::Custom {
            description: format!("应用了能力：{}", self.name()),
            data: HashMap::new(),
        }])
    }

    fn triggers(&self) -> Vec<crate::EffectTrigger> {
        self.trigger_conditions.clone()
    }

    fn target_requirements(&self) -> Vec<crate::TargetRequirement> {
        self.target_requirements.clone()
    }
}

/// 宝可梦攻击效果实现
#[derive(Clone)]
pub struct PokemonAttackEffect {
    base: BaseEffect,
    damage: u32,
    target_requirements: Vec<crate::TargetRequirement>,
}

impl PokemonAttackEffect {
    pub fn new(
        name: String,
        description: String,
        damage: u32,
        target_requirements: Vec<crate::TargetRequirement>,
    ) -> Self {
        Self {
            base: BaseEffect::new(name, description),
            damage,
            target_requirements,
        }
    }
}

impl Effect for PokemonAttackEffect {
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
        Ok(vec![EffectOutcome::DamageDealt {
            target: CardId::new_v4(), // 在实际实现中，这将是实际目标
            amount: self.damage,
        }])
    }

    fn triggers(&self) -> Vec<crate::EffectTrigger> {
        vec![crate::EffectTrigger::OnAttack]
    }

    fn target_requirements(&self) -> Vec<crate::TargetRequirement> {
        self.target_requirements.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pokemon_ability_effect_creation() {
        let ability = PokemonAbilityEffect::new(
            "避雷针".to_string(),
            "每当这只宝可梦受到电属性攻击时，攻击的宝可梦变为麻痹状态。".to_string(),
            AbilityType::Passive,
            vec![crate::EffectTrigger::OnTakeDamage],
            vec![crate::TargetRequirement::Pokemon],
        );

        assert_eq!(ability.name(), "避雷针");
        assert_eq!(ability.triggers(), vec![crate::EffectTrigger::OnTakeDamage]);
    }

    #[test]
    fn test_pokemon_attack_effect_creation() {
        let attack_effect = PokemonAttackEffect::new(
            "十万伏特".to_string(),
            "抛硬币，如果正面，此攻击造成50点伤害。".to_string(),
            50,
            vec![crate::TargetRequirement::Pokemon, crate::TargetRequirement::InPlay],
        );

        assert_eq!(attack_effect.name(), "十万伏特");
        assert_eq!(attack_effect.triggers(), vec![crate::EffectTrigger::OnAttack]);
    }
}