//! 数据导入和导出功能
//!
//! 该模块提供了从各种格式导入卡牌数据的支持，
//! 包括 JSON、CSV 和数据库文件。

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "csv_import")]
pub mod csv;

#[cfg(feature = "database")]
pub mod database;

use crate::core::Card;
use std::collections::HashMap;

/// Common trait for data importers
pub trait DataImporter {
    /// Import cards from the data source
    fn import_cards(&self) -> Result<Vec<Card>, ImportError>;

    /// Import a specific card by ID or name
    fn import_card(&self, identifier: &str) -> Result<Option<Card>, ImportError>;

    /// Get the source information
    fn source_info(&self) -> SourceInfo;
}

/// Common trait for data exporters
pub trait DataExporter {
    /// Export cards to the data format
    fn export_cards(&self, cards: &[Card]) -> Result<(), ExportError>;

    /// Export a single card
    fn export_card(&self, card: &Card) -> Result<(), ExportError>;
}

/// Information about a data source
#[derive(Debug, Clone)]
pub struct SourceInfo {
    pub name: String,
    pub format: String,
    pub version: String,
    pub card_count: Option<usize>,
}

/// Errors that can occur during import
#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Source not found: {0}")]
    SourceNotFound(String),

    #[cfg(feature = "json")]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "database")]
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
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

/// Batch importer for handling multiple data sources
pub struct BatchImporter {
    importers: Vec<Box<dyn DataImporter>>,
}

impl BatchImporter {
    /// Create a new batch importer
    pub fn new() -> Self {
        Self {
            importers: Vec::new(),
        }
    }

    /// Add an importer to the batch
    pub fn add_importer<I: DataImporter + 'static>(&mut self, importer: I) {
        self.importers.push(Box::new(importer));
    }

    /// Import cards from all sources
    pub fn import_all(&self) -> Result<HashMap<String, Vec<Card>>, ImportError> {
        let mut results = HashMap::new();

        for importer in &self.importers {
            let source_info = importer.source_info();
            let cards = importer.import_cards()?;
            results.insert(source_info.name, cards);
        }

        Ok(results)
    }

    /// Get information about all sources
    pub fn get_sources(&self) -> Vec<SourceInfo> {
        self.importers.iter().map(|i| i.source_info()).collect()
    }
}

impl Default for BatchImporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_importer_creation() {
        let importer = BatchImporter::new();
        assert_eq!(importer.get_sources().len(), 0);
    }
}
