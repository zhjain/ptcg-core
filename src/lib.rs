//! # PTCG Core Engine
//!
//! A flexible and extensible core engine for Pokemon Trading Card Game simulators.
//!
//! ## Quick Start
//!
//! ```rust
//! use ptcg_core::{Game, Player, Deck};
//!
//! // Create a new game
//! let mut game = Game::new();
//!
//! // Add players
//! let player1 = Player::new("Player 1".to_string());
//! let player2 = Player::new("Player 2".to_string());
//!
//! game.add_player(player1).unwrap();
//! game.add_player(player2).unwrap();
//!
//! // Start the game (after setting decks)
//! // game.start().unwrap();
//! ```
//!
//! ## Features
//!
//! - **Modular Design**: Use only what you need
//! - **Data Import**: Support for various data formats (.pdb, JSON, CSV)
//! - **Rule Extensions**: Easy to add new card effects and rules
//! - **Network Ready**: Built-in support for multiplayer games
//! - **Performance**: Zero-cost abstractions and compile-time optimizations

pub mod core;
pub mod data;
pub mod effects;
pub mod events;
pub mod rules;

#[cfg(feature = "async")]
pub mod network;

// Re-export commonly used types
pub use core::{
    card::{Ability, Attack, Card, CardRarity, CardType, EnergyType},
    deck::{Deck, DeckValidationError},
    game::{Game, GamePhase, GameRules, GameState},
    player::{Player, PlayerId, SpecialCondition},
};

pub use rules::{Rule, RuleEngine, StandardRules};

pub use events::{Event, EventBus, EventHandler};

pub use effects::{Effect, EffectTarget, EffectTrigger};

#[cfg(feature = "json")]
pub use data::json::JsonImporter;

#[cfg(feature = "csv_import")]
pub use data::csv::CsvImporter;

#[cfg(feature = "database")]
pub use data::database::DatabaseImporter;

/// Result type used throughout the library
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the library
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Game error: {0}")]
    Game(String),

    #[error("Rule violation: {0}")]
    Rule(String),

    #[error("Data error: {0}")]
    Data(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "json")]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "database")]
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get library information
pub fn info() -> LibraryInfo {
    LibraryInfo {
        version: VERSION,
        features: get_enabled_features(),
    }
}

/// Library information
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
