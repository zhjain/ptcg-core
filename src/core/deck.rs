//! Deck management and construction functionality

use crate::core::card::{Card, CardId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a deck
pub type DeckId = Uuid;

/// Represents a deck of cards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deck {
    /// Unique identifier for this deck
    pub id: DeckId,
    /// Name of the deck
    pub name: String,
    /// Cards in the deck with their quantities
    pub cards: HashMap<CardId, u32>,
    /// Format this deck is legal for
    pub format: String,
    /// Metadata about the deck
    pub metadata: HashMap<String, String>,
}

/// Deck validation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeckValidationError {
    /// Too few cards in deck
    TooFewCards { current: usize, minimum: usize },
    /// Too many cards in deck
    TooManyCards { current: usize, maximum: usize },
    /// Too many copies of a single card
    TooManyCopies { card_name: String, copies: u32, limit: u32 },
    /// Invalid card for the format
    InvalidCard { card_name: String, format: String },
    /// Missing required cards (e.g., no basic Pokemon)
    MissingRequired { requirement: String },
}

impl Deck {
    /// Create a new empty deck
    pub fn new(name: String, format: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            cards: HashMap::new(),
            format,
            metadata: HashMap::new(),
        }
    }

    /// Add a card to the deck
    pub fn add_card(&mut self, card_id: CardId, quantity: u32) {
        if quantity > 0 {
            *self.cards.entry(card_id).or_insert(0) += quantity;
        }
    }

    /// Remove cards from the deck
    pub fn remove_card(&mut self, card_id: CardId, quantity: u32) {
        if let Some(current_quantity) = self.cards.get_mut(&card_id) {
            if *current_quantity <= quantity {
                self.cards.remove(&card_id);
            } else {
                *current_quantity -= quantity;
            }
        }
    }

    /// Set the exact quantity of a card in the deck
    pub fn set_card_quantity(&mut self, card_id: CardId, quantity: u32) {
        if quantity == 0 {
            self.cards.remove(&card_id);
        } else {
            self.cards.insert(card_id, quantity);
        }
    }

    /// Get the quantity of a specific card in the deck
    pub fn get_card_quantity(&self, card_id: CardId) -> u32 {
        self.cards.get(&card_id).copied().unwrap_or(0)
    }

    /// Get the total number of cards in the deck
    pub fn total_cards(&self) -> u32 {
        self.cards.values().sum()
    }

    /// Get the number of unique cards in the deck
    pub fn unique_cards(&self) -> usize {
        self.cards.len()
    }

    /// Check if the deck contains a specific card
    pub fn contains_card(&self, card_id: CardId) -> bool {
        self.cards.contains_key(&card_id)
    }

    /// Get all cards in the deck as a flat list (for shuffling)
    pub fn to_card_list(&self) -> Vec<CardId> {
        let mut cards = Vec::new();
        for (&card_id, &quantity) in &self.cards {
            for _ in 0..quantity {
                cards.push(card_id);
            }
        }
        cards
    }

    /// Create a deck from a list of cards
    pub fn from_card_list(name: String, format: String, cards: Vec<CardId>) -> Self {
        let mut deck = Deck::new(name, format);
        for card_id in cards {
            deck.add_card(card_id, 1);
        }
        deck
    }

    /// Validate the deck according to standard PTCG rules
    pub fn validate(&self, card_database: &HashMap<CardId, Card>) -> Result<(), Vec<DeckValidationError>> {
        let mut errors = Vec::new();
        let total = self.total_cards();

        // Check deck size (standard format: exactly 60 cards)
        match self.format.as_str() {
            "Standard" | "Expanded" => {
                if total != 60 {
                    if total < 60 {
                        errors.push(DeckValidationError::TooFewCards { current: total as usize, minimum: 60 });
                    } else {
                        errors.push(DeckValidationError::TooManyCards { current: total as usize, maximum: 60 });
                    }
                }
            }
            "Limited" => {
                if total < 40 {
                    errors.push(DeckValidationError::TooFewCards { current: total as usize, minimum: 40 });
                }
            }
            _ => {
                // Custom format, skip size validation
            }
        }

        // Check card copy limits and basic Pokemon requirement
        let mut has_basic_pokemon = false;
        
        for (&card_id, &quantity) in &self.cards {
            if let Some(card) = card_database.get(&card_id) {
                // Check copy limit (standard: 4 copies max, except basic energy)
                let max_copies = if let Some(energy_type) = card.get_energy_type() {
                    // Basic energy cards have no limit
                    if matches!(card.card_type, crate::core::card::CardType::Energy { is_basic: true, .. }) {
                        u32::MAX
                    } else {
                        4
                    }
                } else {
                    4
                };

                if quantity > max_copies {
                    errors.push(DeckValidationError::TooManyCopies {
                        card_name: card.name.clone(),
                        copies: quantity,
                        limit: max_copies,
                    });
                }

                // Check for basic Pokemon
                if let crate::core::card::CardType::Pokemon { stage: crate::core::card::EvolutionStage::Basic, .. } = card.card_type {
                    has_basic_pokemon = true;
                }
            }
        }

        // Ensure deck has at least one basic Pokemon
        if !has_basic_pokemon {
            errors.push(DeckValidationError::MissingRequired {
                requirement: "At least one Basic Pokemon".to_string(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Shuffle the deck and return a randomized card list
    pub fn shuffle(&self) -> Vec<CardId> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut cards = self.to_card_list();
        
        // Simple shuffle algorithm (in a real implementation, you'd use a proper RNG)
        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        let seed = hasher.finish();
        
        // Fisher-Yates shuffle with simple PRNG
        for i in (1..cards.len()).rev() {
            let j = (seed.wrapping_mul(i as u64 + 1)) % (i as u64 + 1);
            cards.swap(i, j as usize);
        }
        
        cards
    }

    /// Get deck statistics
    pub fn get_statistics(&self, card_database: &HashMap<CardId, Card>) -> DeckStatistics {
        let mut stats = DeckStatistics::default();
        
        for (&card_id, &quantity) in &self.cards {
            if let Some(card) = card_database.get(&card_id) {
                match &card.card_type {
                    crate::core::card::CardType::Pokemon { .. } => {
                        stats.pokemon_count += quantity;
                    }
                    crate::core::card::CardType::Energy { .. } => {
                        stats.energy_count += quantity;
                    }
                    crate::core::card::CardType::Trainer { .. } => {
                        stats.trainer_count += quantity;
                    }
                }
            }
        }
        
        stats.total_cards = self.total_cards();
        stats.unique_cards = self.unique_cards() as u32;
        stats
    }

    /// Add metadata to the deck
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Export deck to a simple text format
    pub fn export_text(&self, card_database: &HashMap<CardId, Card>) -> String {
        let mut output = Vec::new();
        output.push(format!("Deck: {}", self.name));
        output.push(format!("Format: {}", self.format));
        output.push(String::new());

        // Group cards by type
        let mut pokemon = Vec::new();
        let mut trainers = Vec::new();
        let mut energy = Vec::new();

        for (&card_id, &quantity) in &self.cards {
            if let Some(card) = card_database.get(&card_id) {
                let line = format!("{} {}", quantity, card.name);
                match &card.card_type {
                    crate::core::card::CardType::Pokemon { .. } => pokemon.push(line),
                    crate::core::card::CardType::Trainer { .. } => trainers.push(line),
                    crate::core::card::CardType::Energy { .. } => energy.push(line),
                }
            }
        }

        if !pokemon.is_empty() {
            output.push("Pokemon:".to_string());
            output.extend(pokemon);
            output.push(String::new());
        }

        if !trainers.is_empty() {
            output.push("Trainers:".to_string());
            output.extend(trainers);
            output.push(String::new());
        }

        if !energy.is_empty() {
            output.push("Energy:".to_string());
            output.extend(energy);
        }

        output.join("\n")
    }
}

/// Statistics about a deck
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeckStatistics {
    pub total_cards: u32,
    pub unique_cards: u32,
    pub pokemon_count: u32,
    pub trainer_count: u32,
    pub energy_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::{Card, CardType, CardRarity, EnergyType, EvolutionStage};

    fn create_test_card(name: &str, card_type: CardType) -> Card {
        Card::new(
            name.to_string(),
            card_type,
            "Test Set".to_string(),
            "001".to_string(),
            CardRarity::Common,
        )
    }

    #[test]
    fn test_create_deck() {
        let deck = Deck::new("Test Deck".to_string(), "Standard".to_string());
        assert_eq!(deck.name, "Test Deck");
        assert_eq!(deck.format, "Standard");
        assert_eq!(deck.total_cards(), 0);
    }

    #[test]
    fn test_add_remove_cards() {
        let mut deck = Deck::new("Test Deck".to_string(), "Standard".to_string());
        let card_id = Uuid::new_v4();

        deck.add_card(card_id, 4);
        assert_eq!(deck.get_card_quantity(card_id), 4);
        assert_eq!(deck.total_cards(), 4);

        deck.remove_card(card_id, 2);
        assert_eq!(deck.get_card_quantity(card_id), 2);
        assert_eq!(deck.total_cards(), 2);

        deck.remove_card(card_id, 3);
        assert_eq!(deck.get_card_quantity(card_id), 0);
        assert_eq!(deck.total_cards(), 0);
    }

    #[test]
    fn test_deck_validation() {
        let mut deck = Deck::new("Test Deck".to_string(), "Standard".to_string());
        let mut card_db = HashMap::new();

        // Create a basic Pokemon
        let basic_pokemon = create_test_card("Pikachu", CardType::Pokemon {
            species: "Pikachu".to_string(),
            hp: 60,
            retreat_cost: 1,
            weakness: None,
            resistance: None,
            stage: EvolutionStage::Basic,
            evolves_from: None,
        });
        let basic_id = basic_pokemon.id;
        card_db.insert(basic_id, basic_pokemon);

        // Create basic energy
        let energy = create_test_card("Lightning Energy", CardType::Energy {
            energy_type: EnergyType::Lightning,
            is_basic: true,
        });
        let energy_id = energy.id;
        card_db.insert(energy_id, energy);

        // Add cards to make a valid 60-card deck
        deck.add_card(basic_id, 4);
        deck.add_card(energy_id, 56);

        let result = deck.validate(&card_db);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deck_shuffle() {
        let mut deck = Deck::new("Test Deck".to_string(), "Standard".to_string());
        let card_ids: Vec<CardId> = (0..10).map(|_| Uuid::new_v4()).collect();

        for &card_id in &card_ids {
            deck.add_card(card_id, 1);
        }

        let shuffled = deck.shuffle();
        assert_eq!(shuffled.len(), 10);
        
        // All original cards should be present
        for &card_id in &card_ids {
            assert!(shuffled.contains(&card_id));
        }
    }

    #[test]
    fn test_deck_statistics() {
        let mut deck = Deck::new("Test Deck".to_string(), "Standard".to_string());
        let mut card_db = HashMap::new();

        // Add a Pokemon
        let pokemon = create_test_card("Pikachu", CardType::Pokemon {
            species: "Pikachu".to_string(),
            hp: 60,
            retreat_cost: 1,
            weakness: None,
            resistance: None,
            stage: EvolutionStage::Basic,
            evolves_from: None,
        });
        let pokemon_id = pokemon.id;
        card_db.insert(pokemon_id, pokemon);
        deck.add_card(pokemon_id, 4);

        // Add energy
        let energy = create_test_card("Lightning Energy", CardType::Energy {
            energy_type: EnergyType::Lightning,
            is_basic: true,
        });
        let energy_id = energy.id;
        card_db.insert(energy_id, energy);
        deck.add_card(energy_id, 10);

        let stats = deck.get_statistics(&card_db);
        assert_eq!(stats.pokemon_count, 4);
        assert_eq!(stats.energy_count, 10);
        assert_eq!(stats.total_cards, 14);
        assert_eq!(stats.unique_cards, 2);
    }
}