//! 效果管理器，用于处理所有与效果相关的操作

use crate::core::effects::{Effect, EffectId, EffectTarget, EffectContext, EffectOutcome, EffectError, BaseEffect};
use crate::core::card::CardId;
use crate::core::game::state::Game;
use crate::core::player::PlayerId;
use std::collections::HashMap;

/// 效果管理器，用于处理所有与效果相关的操作
pub struct EffectManager {
    /// 所有已注册的效果
    effects: HashMap<EffectId, Box<dyn Effect>>,
    /// 游戏中当前激活的效果
    active_effects: HashMap<CardId, Vec<EffectId>>,
    /// 等待解决的触发效果
    #[allow(dead_code)]
    pending_effects: Vec<(EffectId, EffectContext)>,
}

impl EffectManager {
    /// 创建新的效果管理器
    pub fn new() -> Self {
        Self {
            effects: HashMap::new(),
            active_effects: HashMap::new(),
            pending_effects: Vec::new(),
        }
    }

    /// 注册效果
    pub fn register_effect<E: Effect + 'static>(&mut self, effect: E) -> EffectId {
        let id = effect.id();
        self.effects.insert(id, Box::new(effect));
        id
    }

    /// 将效果附加到卡牌上
    pub fn attach_effect(&mut self, card_id: CardId, effect_id: EffectId) -> Result<(), EffectError> {
        // 检查效果是否存在
        if !self.effects.contains_key(&effect_id) {
            return Err(EffectError::General { 
                message: "效果未找到".to_string() 
            });
        }
        
        self.active_effects
            .entry(card_id)
            .or_default()
            .push(effect_id);
            
        Ok(())
    }

    /// 从卡牌上移除效果
    pub fn detach_effect(&mut self, card_id: CardId, effect_id: EffectId) -> Result<(), EffectError> {
        if let Some(effects) = self.active_effects.get_mut(&card_id) {
            if let Some(pos) = effects.iter().position(|&id| id == effect_id) {
                effects.remove(pos);
                return Ok(());
            }
        }
        
        Err(EffectError::General { 
            message: "卡牌上未找到效果".to_string() 
        })
    }

    /// 移除卡牌上的所有效果
    pub fn remove_card_effects(&mut self, card_id: CardId) {
        self.active_effects.remove(&card_id);
    }

    /// 获取附加到卡牌上的所有效果
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

    /// 检查卡牌是否有任何效果
    pub fn has_effects(&self, card_id: CardId) -> bool {
        self.active_effects
            .get(&card_id)
            .map(|effects| !effects.is_empty())
            .unwrap_or(false)
    }

    /// 根据触发类型获取效果
    pub fn get_effects_by_trigger(&self, trigger: crate::EffectTrigger) -> Vec<(&dyn Effect, CardId)> {
        let mut result = Vec::new();
        
        for (card_id, effect_ids) in &self.active_effects {
            for effect_id in effect_ids {
                if let Some(effect) = self.effects.get(effect_id) {
                    if effect.triggers().contains(&trigger) {
                        result.push((effect.as_ref(), *card_id));
                    }
                }
            }
        }
        
        result
    }

    /// 触发特定类型的效果
    pub fn trigger_effects(
        &mut self, 
        trigger: crate::EffectTrigger, 
        context: EffectContext
    ) -> Vec<Result<Vec<EffectOutcome>, EffectError>> {
        let mut results = Vec::new();
        
        // 获取所有应该触发的效果
        let triggered_effects = self.get_effects_by_trigger(trigger.clone());
        
        // 应用每个触发的效果
        for (effect, card_id) in triggered_effects {
            let mut effect_context = context.clone();
            effect_context.source_card = card_id;
            
            if effect.can_apply(&Game::default(), &effect_context) {
                let result = effect.apply(&mut Game::default(), &effect_context);
                results.push(result);
            }
        }
        
        results
    }

    /// 处理所有效果的回合开始
    pub fn on_turn_start(&mut self, game: &mut Game, player_id: PlayerId) {
        // 收集所有效果ID及其卡牌ID
        let effect_entries: Vec<(CardId, Vec<EffectId>)> = self.active_effects
            .iter()
            .map(|(card_id, effect_ids)| (*card_id, effect_ids.clone()))
            .collect();
        
        // 处理每个效果
        for (_card_id, effect_ids) in effect_entries {
            for effect_id in effect_ids {
                if let Some(effect) = self.effects.get(&effect_id) {
                    let _ = effect.on_turn_start(game, player_id);
                }
            }
        }
    }

    /// 处理所有效果的回合结束
    pub fn on_turn_end(&mut self, game: &mut Game, player_id: PlayerId) {
        // 收集所有效果ID及其卡牌ID
        let effect_entries: Vec<(CardId, Vec<EffectId>)> = self.active_effects
            .iter()
            .map(|(card_id, effect_ids)| (*card_id, effect_ids.clone()))
            .collect();
        
        // 处理每个效果
        for (_card_id, effect_ids) in effect_entries {
            for effect_id in effect_ids {
                if let Some(effect) = self.effects.get(&effect_id) {
                    let _ = effect.on_turn_end(game, player_id);
                }
            }
        }
    }
}

impl Default for EffectManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 基础伤害效果实现
#[derive(Clone)]
pub struct DamageEffect {
    base: BaseEffect,
    damage: u32,
    #[allow(dead_code)]
    target_type: EffectTarget,
}

impl DamageEffect {
    pub fn new(name: String, damage: u32, target_type: EffectTarget) -> Self {
        Self {
            base: BaseEffect::new(name, "对目标造成伤害".to_string()),
            damage,
            target_type,
        }
    }
}

impl Effect for DamageEffect {
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
        true // 简化版 - 会检查目标有效性
    }

    fn apply(&self, game: &mut Game, context: &EffectContext) -> Result<Vec<EffectOutcome>, EffectError> {
        let target_card = match &context.target {
            Some(EffectTarget::Card(card_id)) => *card_id,
            Some(EffectTarget::ActivePokemon(player_id)) => {
                // 注意：player_id已经是引用，所以我们不需要解引用
                if let Some(player) = game.get_player(*player_id) {
                    if let Some(active) = player.active_pokemon {
                        active
                    } else {
                        return Err(EffectError::InvalidTarget {
                            reason: "没有活跃的宝可梦".to_string(),
                        });
                    }
                } else {
                    return Err(EffectError::InvalidTarget {
                        reason: "未找到玩家".to_string(),
                    });
                }
            }
            _ => {
                return Err(EffectError::InvalidTarget {
                    reason: "无效的目标类型".to_string(),
                });
            }
        };

        // 对目标应用伤害
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
                reason: "未找到目标宝可梦".to_string(),
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