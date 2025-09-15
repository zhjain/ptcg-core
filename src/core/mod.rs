//! Core game data structures and types
//!
//! This module contains the fundamental building blocks of the PTCG engine,
//! including cards, players, game state, and deck management.

pub mod card;
pub mod deck;
pub mod game;
pub mod player;

pub use card::*;
pub use deck::*;
pub use game::*;
pub use player::*;
