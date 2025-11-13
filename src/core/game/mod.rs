//! Game module - contains all game-related functionality
//!
//! This module is organized into several submodules:
//! - `state`: Game state and phase management
//! - `setup`: Game setup logic
//! - `turn`: Turn processing and flow control
//! - `actions`: Game actions and operations

pub mod state;
pub mod setup;
pub mod turn;
pub mod actions;

// Re-export the main game struct and important types
pub use state::{Game, GameId, GamePhase, GameState, GameRules, GameEvent};