//! Card-related data structures and functionality

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a card
pub type CardId = Uuid;

/// Represents different types of cards in PTCG
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    /// Pokemon cards that can be played on the field
    Pokemon {
        /// Pokemon species (e.g., "Pikachu")
        species: String,
        /// HP (Health Points)
        hp: u32,
        /// Retreat cost (energy required to retreat)
        retreat_cost: u32,
        /// Weakness (type that deals double damage)
        weakness: Option<EnergyType>,
        /// Resistance (type that deals less damage)
        resistance: Option<EnergyType>,
        /// Evolution stage (Basic, Stage 1, Stage 2, etc.)
        stage: EvolutionStage,
        /// Previous evolution (if applicable)
        evolves_from: Option<String>,
    },
    /// Energy cards used to power attacks
    Energy {
        /// Type of energy
        energy_type: EnergyType,
        /// Is this a basic energy card?
        is_basic: bool,
    },
    /// Trainer cards with various effects
    Trainer {
        /// Type of trainer card
        trainer_type: TrainerType,
    },
}

/// Different types of energy in PTCG
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnergyType {
    Grass,
    Fire,
    Water,
    Lightning,
    Psychic,
    Fighting,
    Darkness,
    Metal,
    Fairy,
    Dragon,
    Colorless,
}

/// Evolution stages for Pokemon
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionStage {
    Basic,
    Stage1,
    Stage2,
    Mega,
    GX,
    EX,
    V,
    VMax,
}

/// Types of trainer cards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainerType {
    Item,
    Supporter,
    Stadium,
    Tool,
}

/// Card rarity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
    RareHolo,
    UltraRare,
    SecretRare,
    Promo,
}

/// Attack information for Pokemon cards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attack {
    /// Name of the attack
    pub name: String,
    /// Energy cost required to use this attack
    pub cost: Vec<EnergyType>,
    /// Base damage dealt by this attack
    pub damage: u32,
    /// Special effects of this attack
    pub effect: Option<String>,
}

/// Ability information for Pokemon cards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ability {
    /// Name of the ability
    pub name: String,
    /// Description of what the ability does
    pub effect: String,
    /// Type of ability (Ability, Poke-Power, Poke-Body, etc.)
    pub ability_type: String,
}

/// Main card structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    /// Unique identifier for this card
    pub id: CardId,
    /// Name of the card
    pub name: String,
    /// Type of card (Pokemon, Energy, Trainer)
    pub card_type: CardType,
    /// Set information
    pub set_name: String,
    /// Card number in the set
    pub set_number: String,
    /// Rarity of the card
    pub rarity: CardRarity,
    /// Attacks (for Pokemon cards)
    pub attacks: Vec<Attack>,
    /// Abilities (for Pokemon cards)
    pub abilities: Vec<Ability>,
    /// Card rule text
    pub rules: Vec<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Card {
    /// Create a new card with the given parameters
    pub fn new(
        name: String,
        card_type: CardType,
        set_name: String,
        set_number: String,
        rarity: CardRarity,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            card_type,
            set_name,
            set_number,
            rarity,
            attacks: Vec::new(),
            abilities: Vec::new(),
            rules: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Check if this is a Pokemon card
    pub fn is_pokemon(&self) -> bool {
        matches!(self.card_type, CardType::Pokemon { .. })
    }

    /// Check if this is an Energy card
    pub fn is_energy(&self) -> bool {
        matches!(self.card_type, CardType::Energy { .. })
    }

    /// Check if this is a Trainer card
    pub fn is_trainer(&self) -> bool {
        matches!(self.card_type, CardType::Trainer { .. })
    }

    /// Get the HP of a Pokemon card (returns None for non-Pokemon cards)
    pub fn get_hp(&self) -> Option<u32> {
        match &self.card_type {
            CardType::Pokemon { hp, .. } => Some(*hp),
            _ => None,
        }
    }

    /// Get the energy type of an Energy card
    pub fn get_energy_type(&self) -> Option<&EnergyType> {
        match &self.card_type {
            CardType::Energy { energy_type, .. } => Some(energy_type),
            _ => None,
        }
    }

    /// Add an attack to a Pokemon card
    pub fn add_attack(&mut self, attack: Attack) {
        if self.is_pokemon() {
            self.attacks.push(attack);
        }
    }

    /// Add an ability to a Pokemon card
    pub fn add_ability(&mut self, ability: Ability) {
        if self.is_pokemon() {
            self.abilities.push(ability);
        }
    }

    /// Add a rule to the card
    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    /// Add metadata to the card
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pokemon_card() {
        let pikachu = Card::new(
            "Pikachu".to_string(),
            CardType::Pokemon {
                species: "Pikachu".to_string(),
                hp: 60,
                retreat_cost: 1,
                weakness: Some(EnergyType::Fighting),
                resistance: None,
                stage: EvolutionStage::Basic,
                evolves_from: None,
            },
            "Base Set".to_string(),
            "025".to_string(),
            CardRarity::Common,
        );

        assert!(pikachu.is_pokemon());
        assert_eq!(pikachu.get_hp(), Some(60));
        assert_eq!(pikachu.name, "Pikachu");
    }

    #[test]
    fn test_create_energy_card() {
        let lightning_energy = Card::new(
            "Lightning Energy".to_string(),
            CardType::Energy {
                energy_type: EnergyType::Lightning,
                is_basic: true,
            },
            "Base Set".to_string(),
            "100".to_string(),
            CardRarity::Common,
        );

        assert!(lightning_energy.is_energy());
        assert_eq!(lightning_energy.get_energy_type(), Some(&EnergyType::Lightning));
    }

    #[test]
    fn test_add_attack() {
        let mut pikachu = Card::new(
            "Pikachu".to_string(),
            CardType::Pokemon {
                species: "Pikachu".to_string(),
                hp: 60,
                retreat_cost: 1,
                weakness: Some(EnergyType::Fighting),
                resistance: None,
                stage: EvolutionStage::Basic,
                evolves_from: None,
            },
            "Base Set".to_string(),
            "025".to_string(),
            CardRarity::Common,
        );

        let thundershock = Attack {
            name: "Thundershock".to_string(),
            cost: vec![EnergyType::Lightning, EnergyType::Colorless],
            damage: 30,
            effect: Some("Flip a coin. If heads, the Defending Pokémon is now Paralyzed.".to_string()),
        };

        pikachu.add_attack(thundershock);
        assert_eq!(pikachu.attacks.len(), 1);
        assert_eq!(pikachu.attacks[0].name, "Thundershock");
    }
}