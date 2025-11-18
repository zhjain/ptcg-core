//! Data export functionality

use crate::core::card::Card;

/// Common trait for data exporters
pub trait DataExporter {
    /// Export cards to the data format
    fn export_cards(&self, cards: &[Card]) -> Result<(), ExportError>;

    /// Export a single card
    fn export_card(&self, card: &Card) -> Result<(), ExportError>;
}

/// Errors that can occur during export
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[cfg(feature = "json")]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "database")]
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}