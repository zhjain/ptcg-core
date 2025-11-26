//! # PTCG 核心引擎
//!
//! 一个灵活且可扩展的宝可梦集换式卡牌游戏模拟器核心引擎。
//!
//! ## 快速开始
//!
//! ```rust
//! use ptcg_core::{Game, Player, Deck};
//!
//! // 创建新游戏
//! let mut game = Game::new();
//!
//! // 添加玩家
//! let player1 = Player::new("玩家1".to_string());
//! let player2 = Player::new("玩家2".to_string());
//!
//! game.add_player(player1).unwrap();
//! game.add_player(player2).unwrap();
//!
//! // 开始游戏（设置牌组后）
//! // game.start().unwrap();
//! ```
//!
//! ## 特性
//!
//! - **模块化设计**: 按需使用所需功能
//! - **数据导入**: 支持多种数据格式 (.pdb, JSON, CSV)
//! - **规则扩展**: 轻松添加新的卡牌效果和规则
//! - **网络就绪**: 内置多人游戏支持
//! - **性能优化**: 零成本抽象和编译时优化

/// 核心游戏逻辑模块
pub mod core;
/// 数据导入和管理模块
pub mod data;

/// 网络功能模块（需要async特性）
#[cfg(feature = "async")]
pub mod network;

// 重新导出常用类型
pub use core::{
    card::{Ability, Attack, Card, CardRarity, CardType, EnergyType, TrainerType},
    deck::{Deck, DeckValidationError},
    effects::{
        Effect, EffectContext, EffectError, EffectId, EffectOutcome, EffectTarget, EffectTrigger,
        TargetRequirement, PokemonAbilityEffect, PokemonAttackEffect, TrainerEffect, SpecialEnergyEffect, AbilityType
    },
    events::{EventBus, EventHandler, GameEvent},
    game::{Game, GamePhase, GameRules, GameState},
    player::{CardLocation, Player, PlayerId, SpecialCondition, SpecialConditionInstance},
    rules::{Rule, RuleEngine, StandardRules},
};

#[cfg(feature = "json")]
pub use data::json::JsonImporter;

#[cfg(feature = "csv_import")]
pub use data::csv::CsvImporter;

#[cfg(feature = "database")]
pub use data::database::DatabaseImporter;

/// 库中使用的结果类型
pub type Result<T> = std::result::Result<T, Error>;

/// 库的错误类型
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("游戏错误: {0}")]
    Game(String),

    #[error("规则违反: {0}")]
    Rule(String),

    #[error("数据错误: {0}")]
    Data(String),

    #[error("网络错误: {0}")]
    Network(String),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "json")]
    #[error("JSON错误: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "database")]
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),
}

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 获取库信息
pub fn info() -> LibraryInfo {
    LibraryInfo {
        version: VERSION,
        features: get_enabled_features(),
    }
}

/// 库信息
#[derive(Debug)]
pub struct LibraryInfo {
    pub version: &'static str,
    pub features: Vec<&'static str>,
}

#[allow(clippy::vec_init_then_push)]
fn get_enabled_features() -> Vec<&'static str> {
    let mut features = Vec::new();

    #[cfg(feature = "json")]
    features.push("json");

    #[cfg(feature = "csv_import")]
    features.push("csv_import");

    #[cfg(feature = "database")]
    features.push("database");

    #[cfg(feature = "async")]
    features.push("async");

    features
}
