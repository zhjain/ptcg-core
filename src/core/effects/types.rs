//! 效果类型和特征

use crate::core::card::CardId;
use crate::core::game::state::Game;
use crate::core::player::PlayerId;
use crate::{EffectTarget, EffectTrigger, TargetRequirement};
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 效果的唯一标识符
pub type EffectId = Uuid;

/// 不同的能力类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbilityType {
    /// 主动能力 - 玩家可以激活它
    Active,
    /// 被动能力 - 自动激活
    Passive,
    /// 宝可梦之力 - 每回合一次的能力（旧卡牌）
    PokePower,
    /// 宝可梦之身 - 总是激活的能力（旧卡牌）
    PokeBody,
}

/// 效果类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectType {
    /// 伤害效果
    Damage { amount: u32 },
    /// 治疗效果
    Heal { amount: u32 },
    /// 状态效果
    Status { condition: String, probability: u32 },
    /// 抽卡效果
    Draw { count: u32 },
    /// 能量附加效果
    AttachEnergy { energy_type: String },
    /// 特殊条件应用效果
    ApplySpecialCondition { condition: String },
    /// 自定义效果
    Custom { logic: String },
}

/// 实现卡牌效果的特征
pub trait Effect: DynClone + Send + Sync {
    /// 获取效果的唯一标识符
    fn id(&self) -> EffectId;

    /// 获取效果的名称
    fn name(&self) -> &str;

    /// 获取效果的描述
    fn description(&self) -> &str;

    /// 检查此效果是否可以在当前游戏状态下应用
    fn can_apply(&self, game: &Game, context: &EffectContext) -> bool;

    /// 将效果应用于游戏状态
    fn apply(&self, game: &mut Game, context: &EffectContext) -> EffectResult;

    /// 获取效果的触发条件
    fn triggers(&self) -> Vec<EffectTrigger>;

    /// 获取效果的目标要求
    fn target_requirements(&self) -> Vec<TargetRequirement>;
    
    /// 当效果附加到卡牌时调用
    fn on_attach(&self, _game: &mut Game, _card_id: CardId) -> EffectResult {
        Ok(vec![])
    }
    
    /// 当效果从卡牌上移除时调用
    fn on_detach(&self, _game: &mut Game, _card_id: CardId) -> EffectResult {
        Ok(vec![])
    }
    
    /// 在每回合开始时调用
    fn on_turn_start(&self, _game: &mut Game, _player_id: PlayerId) -> EffectResult {
        Ok(vec![])
    }
    
    /// 在每回合结束时调用
    fn on_turn_end(&self, _game: &mut Game, _player_id: PlayerId) -> EffectResult {
        Ok(vec![])
    }
}

dyn_clone::clone_trait_object!(Effect);

/// 效果应用的上下文信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectContext {
    /// 拥有此效果的卡牌
    pub source_card: CardId,
    /// 控制源卡牌的玩家
    pub controller: PlayerId,
    /// 效果的目标（如果有）
    pub target: Option<EffectTarget>,
    /// 效果的附加参数
    pub parameters: HashMap<String, String>,
    /// 触发此效果激活的触发器
    pub trigger: Option<EffectTrigger>,
}

/// 应用效果的结果
pub type EffectResult = Result<Vec<EffectOutcome>, EffectError>;

/// 效果的可能结果
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectOutcome {
    /// 造成了伤害
    DamageDealt { target: CardId, amount: u32 },
    /// 应用了治疗
    Healing { target: CardId, amount: u32 },
    /// 抽取了卡牌
    CardsDrawn { player: PlayerId, count: u32 },
    /// 附加了能量
    EnergyAttached { energy: CardId, target: CardId },
    /// 移动了卡牌
    CardMoved {
        card: CardId,
        from: String,
        to: String,
    },
    /// 应用了特殊状态
    SpecialConditionApplied { target: CardId, condition: String },
    /// 移除了特殊状态
    SpecialConditionRemoved { target: CardId, condition: String },
    /// 自定义效果结果
    Custom {
        description: String,
        data: HashMap<String, String>,
    },
}

/// 应用效果时可能发生的错误
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectError {
    /// 效果的目标无效
    InvalidTarget { reason: String },
    /// 资源不足（能量、卡牌等）
    InsufficientResources {
        resource: String,
        required: u32,
        available: u32,
    },
    /// 由于游戏状态无法应用效果
    InvalidGameState { reason: String },
    /// 未满足效果要求
    RequirementsNotMet { requirement: String },
    /// 一般效果错误
    General { message: String },
}

/// 所有效果实现的基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEffect {
    pub id: EffectId,
    pub name: String,
    pub description: String,
}

impl BaseEffect {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
        }
    }
}