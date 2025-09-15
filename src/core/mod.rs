//! Core game data structures and types
//! 
//! This module contains the fundamental building blocks of the PTCG engine,
//! including cards, players, game state, and deck management.

pub mod card;
pub mod player;
pub mod game;
pub mod deck;

pub use card::*;
pub use player::*;
pub use game::*;
pub use deck::*;