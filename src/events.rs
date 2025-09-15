//! Event system for game state changes and notifications
//!
//! This module provides a flexible event system that allows tracking
//! game state changes, implementing reactive behavior, and logging game history.

use crate::core::{CardId, player::PlayerId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for an event
pub type EventId = Uuid;

/// Main event trait that all game events must implement
pub trait Event: Send + Sync {
    /// Get the event type name
    fn event_type(&self) -> &str;

    /// Get the timestamp when this event occurred
    fn timestamp(&self) -> u64;

    /// Get the player associated with this event (if any)
    fn player_id(&self) -> Option<PlayerId>;

    /// Convert this event to a serializable format
    fn to_serializable(&self) -> SerializableEvent;
}

/// Serializable representation of events for storage and network transmission
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerializableEvent {
    pub id: EventId,
    pub event_type: String,
    pub timestamp: u64,
    pub player_id: Option<PlayerId>,
    pub data: HashMap<String, String>,
}

/// Event handler trait for responding to events
pub trait EventHandler: Send + Sync {
    /// Handle an event
    fn handle_event(&mut self, event: &dyn Event);

    /// Check if this handler is interested in a specific event type
    fn handles_event_type(&self, event_type: &str) -> bool;
}

/// Event bus for managing event distribution
pub struct EventBus {
    /// Registered event handlers
    handlers: Vec<Box<dyn EventHandler>>,
    /// Event history
    history: Vec<SerializableEvent>,
    /// Maximum number of events to keep in history
    max_history: usize,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            history: Vec::new(),
            max_history: 1000,
        }
    }

    /// Create a new event bus with custom history limit
    pub fn with_history_limit(max_history: usize) -> Self {
        Self {
            handlers: Vec::new(),
            history: Vec::new(),
            max_history,
        }
    }

    /// Register an event handler
    pub fn register_handler<H: EventHandler + 'static>(&mut self, handler: H) {
        self.handlers.push(Box::new(handler));
    }

    /// Emit an event to all registered handlers
    pub fn emit(&mut self, event: &dyn Event) {
        // Store in history
        let serializable = event.to_serializable();
        self.history.push(serializable);

        // Trim history if necessary
        if self.history.len() > self.max_history {
            self.history.drain(0..self.history.len() - self.max_history);
        }

        // Notify handlers
        for handler in &mut self.handlers {
            if handler.handles_event_type(event.event_type()) {
                handler.handle_event(event);
            }
        }
    }

    /// Get event history
    pub fn get_history(&self) -> &[SerializableEvent] {
        &self.history
    }

    /// Get events for a specific player
    pub fn get_player_events(&self, player_id: PlayerId) -> Vec<&SerializableEvent> {
        self.history
            .iter()
            .filter(|event| event.player_id == Some(player_id))
            .collect()
    }

    /// Get events of a specific type
    pub fn get_events_by_type(&self, event_type: &str) -> Vec<&SerializableEvent> {
        self.history
            .iter()
            .filter(|event| event.event_type == event_type)
            .collect()
    }

    /// Clear event history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get the number of registered handlers
    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Common game events
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    /// Game started
    GameStarted {
        timestamp: u64,
        players: Vec<PlayerId>,
    },
    /// Game ended
    GameEnded {
        timestamp: u64,
        winner: Option<PlayerId>,
        reason: String,
    },
    /// Turn started
    TurnStarted {
        timestamp: u64,
        player_id: PlayerId,
        turn_number: u32,
    },
    /// Turn ended
    TurnEnded { timestamp: u64, player_id: PlayerId },
    /// Card drawn
    CardDrawn {
        timestamp: u64,
        player_id: PlayerId,
        card_id: Option<CardId>, // None if card is hidden
    },
    /// Card played
    CardPlayed {
        timestamp: u64,
        player_id: PlayerId,
        card_id: CardId,
        from_location: String,
        to_location: String,
    },
}

impl Event for GameEvent {
    fn event_type(&self) -> &str {
        match self {
            GameEvent::GameStarted { .. } => "GameStarted",
            GameEvent::GameEnded { .. } => "GameEnded",
            GameEvent::TurnStarted { .. } => "TurnStarted",
            GameEvent::TurnEnded { .. } => "TurnEnded",
            GameEvent::CardDrawn { .. } => "CardDrawn",
            GameEvent::CardPlayed { .. } => "CardPlayed",
        }
    }

    fn timestamp(&self) -> u64 {
        match self {
            GameEvent::GameStarted { timestamp, .. } => *timestamp,
            GameEvent::GameEnded { timestamp, .. } => *timestamp,
            GameEvent::TurnStarted { timestamp, .. } => *timestamp,
            GameEvent::TurnEnded { timestamp, .. } => *timestamp,
            GameEvent::CardDrawn { timestamp, .. } => *timestamp,
            GameEvent::CardPlayed { timestamp, .. } => *timestamp,
        }
    }

    fn player_id(&self) -> Option<PlayerId> {
        match self {
            GameEvent::GameStarted { .. } => None,
            GameEvent::GameEnded { .. } => None,
            GameEvent::TurnStarted { player_id, .. } => Some(*player_id),
            GameEvent::TurnEnded { player_id, .. } => Some(*player_id),
            GameEvent::CardDrawn { player_id, .. } => Some(*player_id),
            GameEvent::CardPlayed { player_id, .. } => Some(*player_id),
        }
    }

    fn to_serializable(&self) -> SerializableEvent {
        let mut data = HashMap::new();

        match self {
            GameEvent::GameStarted { players, .. } => {
                data.insert("players".to_string(), format!("{:?}", players));
            }
            GameEvent::GameEnded { winner, reason, .. } => {
                data.insert("winner".to_string(), format!("{:?}", winner));
                data.insert("reason".to_string(), reason.clone());
            }
            GameEvent::TurnStarted { turn_number, .. } => {
                data.insert("turn_number".to_string(), turn_number.to_string());
            }
            GameEvent::CardDrawn { card_id, .. } => {
                data.insert("card_id".to_string(), format!("{:?}", card_id));
            }
            GameEvent::CardPlayed {
                card_id,
                from_location,
                to_location,
                ..
            } => {
                data.insert("card_id".to_string(), card_id.to_string());
                data.insert("from_location".to_string(), from_location.clone());
                data.insert("to_location".to_string(), to_location.clone());
            }
            _ => {}
        }

        SerializableEvent {
            id: Uuid::new_v4(),
            event_type: self.event_type().to_string(),
            timestamp: self.timestamp(),
            player_id: self.player_id(),
            data,
        }
    }
}

/// Event handler that logs events to console
pub struct ConsoleEventHandler {
    pub verbose: bool,
}

impl ConsoleEventHandler {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl EventHandler for ConsoleEventHandler {
    fn handle_event(&mut self, event: &dyn Event) {
        if self.verbose {
            println!(
                "[{}] {}: {:?}",
                event.timestamp(),
                event.event_type(),
                event.to_serializable()
            );
        } else {
            println!("[{}] {}", event.timestamp(), event.event_type());
        }
    }

    fn handles_event_type(&self, _event_type: &str) -> bool {
        true // Handle all events
    }
}

/// Helper function to get current timestamp
pub fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_bus_creation() {
        let bus = EventBus::new();
        assert_eq!(bus.handler_count(), 0);
        assert_eq!(bus.get_history().len(), 0);
    }

    #[test]
    fn test_register_handler() {
        let mut bus = EventBus::new();
        let handler = ConsoleEventHandler::new(false);

        bus.register_handler(handler);
        assert_eq!(bus.handler_count(), 1);
    }

    #[test]
    fn test_emit_event() {
        let mut bus = EventBus::new();
        let handler = ConsoleEventHandler::new(false);
        bus.register_handler(handler);

        let event = GameEvent::TurnStarted {
            timestamp: current_timestamp(),
            player_id: Uuid::new_v4(),
            turn_number: 1,
        };

        bus.emit(&event);
        assert_eq!(bus.get_history().len(), 1);
    }
}
