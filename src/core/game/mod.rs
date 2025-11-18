//! Game module - contains all game-related functionality
//!
//! This module is organized into several submodules:
//! - `state`: Game state and phase management
//! - `setup`: Game setup logic
//! - `turn`: Turn processing and flow control
//! - `actions`: Game actions and operations

pub mod actions;
pub mod setup;
pub mod state;
pub mod turn;

// Re-export the main game struct and important types
pub use state::{Game, GameEvent, GameId, GamePhase, GameRules, GameState};
