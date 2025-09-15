//! JSON data import/export functionality

#[cfg(feature = "json")]
use serde_json;

#[cfg(feature = "json")]
use crate::data::{DataExporter, DataImporter, ExportError, ImportError, SourceInfo};

#[cfg(feature = "json")]
use crate::core::Card;

#[cfg(feature = "json")]
use std::path::Path;

/// JSON importer for card data
#[cfg(feature = "json")]
pub struct JsonImporter {
    file_path: String,
}

#[cfg(feature = "json")]
impl JsonImporter {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_string_lossy().to_string(),
        }
    }
}

#[cfg(feature = "json")]
impl DataImporter for JsonImporter {
    fn import_cards(&self) -> Result<Vec<Card>, ImportError> {
        let content = std::fs::read_to_string(&self.file_path)?;
        let cards: Vec<Card> = serde_json::from_str(&content)?;
        Ok(cards)
    }

    fn import_card(&self, _identifier: &str) -> Result<Option<Card>, ImportError> {
        // Simplified implementation
        Ok(None)
    }

    fn source_info(&self) -> SourceInfo {
        SourceInfo {
            name: self.file_path.clone(),
            format: "JSON".to_string(),
            version: "1.0".to_string(),
            card_count: None,
        }
    }
}

#[cfg(feature = "json")]
pub struct JsonExporter {
    file_path: String,
}

#[cfg(feature = "json")]
impl JsonExporter {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_string_lossy().to_string(),
        }
    }
}

#[cfg(feature = "json")]
impl DataExporter for JsonExporter {
    fn export_cards(&self, cards: &[Card]) -> Result<(), ExportError> {
        let json = serde_json::to_string_pretty(cards)?;
        std::fs::write(&self.file_path, json)?;
        Ok(())
    }

    fn export_card(&self, card: &Card) -> Result<(), ExportError> {
        self.export_cards(std::slice::from_ref(card))
    }
}
