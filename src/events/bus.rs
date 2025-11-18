//! Event bus implementation

use crate::events::{GameEvent, EventHandler};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Event bus for managing game events
pub struct EventBus {
    /// Registered event handlers
    handlers: HashMap<String, Box<dyn EventHandler>>,
    /// Event history
    history: Arc<Mutex<Vec<GameEvent>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register an event handler
    pub fn register_handler<H: EventHandler + 'static>(&mut self, handler: H) {
        let name = handler.name().to_string();
        self.handlers.insert(name, Box::new(handler));
    }

    /// Emit an event to all registered handlers
    pub fn emit(&self, event: &GameEvent) {
        // Add event to history
        if let Ok(mut history) = self.history.lock() {
            history.push(event.clone());
        }

        // Notify all handlers
        for handler in self.handlers.values() {
            handler.handle_event(event);
        }
    }

    /// Get the event history
    pub fn get_history(&self) -> Vec<GameEvent> {
        if let Ok(history) = self.history.lock() {
            history.clone()
        } else {
            Vec::new()
        }
    }

    /// Clear the event history
    pub fn clear_history(&self) {
        if let Ok(mut history) = self.history.lock() {
            history.clear();
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}