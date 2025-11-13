//! 卡牌相关的数据结构和功能

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
    /// Additional damage calculation mode
    pub damage_mode: Option<DamageMode>,
    /// Status effects that this attack can apply
    pub status_effects: Vec<StatusEffect>,
    /// Additional conditions required to use this attack
    pub conditions: Vec<String>,
    /// Target selection for this attack
    pub target_type: AttackTargetType,
}

/// Different modes for calculating damage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageMode {
    /// Damage based on number of energy attached
    PerEnergy { per_energy: u32, energy_type: Option<EnergyType> },
    /// Damage based on coin flips
    CoinFlip { per_heads: u32, flips: u32 },
    /// Damage based on Pokemon in specific locations
    PerPokemon { per_pokemon: u32, location: String },
    /// Variable damage range
    Variable { min: u32, max: u32 },
}

/// Status effects that can be applied by attacks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusEffect {
    /// Type of status condition
    pub condition: StatusCondition,
    /// Probability as percentage (0-100)
    pub probability: u32,
    /// Target of the effect
    pub target: String,
}

/// Status conditions in PTCG
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusCondition {
    Poison,
    Burn,
    Paralysis,
    Sleep,
    Confusion,
}

/// Types of attack targets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackTargetType {
    /// Target the active Pokemon
    Active,
    /// Choose any of opponent's Pokemon
    Choose,
    /// All of opponent's Pokemon
    All,
    /// Specific bench positions
    Bench,
    /// Self (for healing, etc.)
    Self_,
}

impl Attack {
    /// Create a simple attack with fixed damage
    pub fn simple(name: String, cost: Vec<EnergyType>, damage: u32) -> Self {
        Self {
            name,
            cost,
            damage,
            effect: None,
            damage_mode: None,
            status_effects: Vec::new(),
            conditions: Vec::new(),
            target_type: AttackTargetType::Active,
        }
    }

    /// Create an attack with status effect
    pub fn with_status(
        name: String,
        cost: Vec<EnergyType>,
        damage: u32,
        status: StatusCondition,
        probability: u32,
    ) -> Self {
        Self {
            name,
            cost,
            damage,
            effect: None,
            damage_mode: None,
            status_effects: vec![StatusEffect {
                condition: status,
                probability,
                target: "defending".to_string(),
            }],
            conditions: Vec::new(),
            target_type: AttackTargetType::Active,
        }
    }

    /// Create an attack with coin flip damage
    pub fn coin_flip_damage(
        name: String,
        cost: Vec<EnergyType>,
        base_damage: u32,
        damage_per_heads: u32,
        flips: u32,
    ) -> Self {
        Self {
            name,
            cost,
            damage: base_damage,
            effect: None,
            damage_mode: Some(DamageMode::CoinFlip {
                per_heads: damage_per_heads,
                flips,
            }),
            status_effects: Vec::new(),
            conditions: Vec::new(),
            target_type: AttackTargetType::Active,
        }
    }

    /// Add a status effect to this attack
    pub fn add_status_effect(&mut self, effect: StatusEffect) {
        self.status_effects.push(effect);
    }

    /// Add a condition to this attack
    pub fn add_condition(&mut self, condition: String) {
        self.conditions.push(condition);
    }

    /// Set the damage mode for this attack
    pub fn set_damage_mode(&mut self, mode: DamageMode) {
        self.damage_mode = Some(mode);
    }

    /// Set the target type for this attack
    pub fn set_target_type(&mut self, target: AttackTargetType) {
        self.target_type = target;
    }

    /// Calculate the actual damage this attack would deal
    pub fn calculate_damage(&self, energy_count: u32, coin_results: &[bool]) -> u32 {
        let mut total_damage = self.damage;
        
        if let Some(ref mode) = self.damage_mode {
            match mode {
                DamageMode::PerEnergy { per_energy, .. } => {
                    total_damage += per_energy * energy_count;
                }
                DamageMode::CoinFlip { per_heads, .. } => {
                    let heads_count = coin_results.iter().filter(|&&result| result).count() as u32;
                    total_damage += per_heads * heads_count;
                }
                DamageMode::PerPokemon { per_pokemon, .. } => {
                    // TODO: Implement when game state is available
                    total_damage += per_pokemon * 2; // Placeholder
                }
                DamageMode::Variable { min, .. } => {
                    total_damage = *min; // Default to minimum
                }
            }
        }
        
        total_damage
    }
}
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

    /// 计算能量类型计数
    fn count_energy_types(energy_list: &[crate::core::card::EnergyType]) -> std::collections::HashMap<crate::core::card::EnergyType, usize> {
        let mut counts = std::collections::HashMap::new();
        for energy_type in energy_list {
            *counts.entry(energy_type.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// 获取满足能量需求的攻击数组
    /// 
    /// # 参数
    /// * `attached_energy` - 附加到宝可梦的能量类型列表
    /// 
    /// # 返回值
    /// 返回可以使用的攻击列表及其索引
    pub fn get_usable_attacks(&self, attached_energy: &[crate::core::card::EnergyType]) -> Vec<(usize, &Attack)> {
        if !self.is_pokemon() {
            return Vec::new();
        }

        let attached_counts = Self::count_energy_types(attached_energy);
        let mut usable_attacks = Vec::new();

        for (index, attack) in self.attacks.iter().enumerate() {
            let required_counts = Self::count_energy_types(&attack.cost);
            
            let mut can_use = true;
            for (energy_type, &required_count) in &required_counts {
                let attached_count = attached_counts.get(energy_type).cloned().unwrap_or(0);
                if attached_count < required_count {
                    can_use = false;
                    break;
                }
            }

            if can_use {
                usable_attacks.push((index, attack));
            }
        }

        usable_attacks
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
        assert_eq!(
            lightning_energy.get_energy_type(),
            Some(&EnergyType::Lightning)
        );
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
            effect: Some(
                "Flip a coin. If heads, the Defending Pokémon is now Paralyzed.".to_string(),
            ),
            damage_mode: None,
            status_effects: vec![StatusEffect {
                condition: StatusCondition::Paralysis,
                probability: 50,
                target: "defending".to_string(),
            }],
            conditions: Vec::new(),
            target_type: AttackTargetType::Active,
        };

        pikachu.add_attack(thundershock);
        assert_eq!(pikachu.attacks.len(), 1);
        assert_eq!(pikachu.attacks[0].name, "Thundershock");
        assert_eq!(pikachu.attacks[0].status_effects.len(), 1);
        assert_eq!(pikachu.attacks[0].status_effects[0].condition, StatusCondition::Paralysis);
    }

    #[test]
    fn test_attack_simple_constructor() {
        let attack = Attack::simple(
            "Quick Attack".to_string(),
            vec![EnergyType::Colorless],
            20,
        );
        
        assert_eq!(attack.name, "Quick Attack");
        assert_eq!(attack.damage, 20);
        assert_eq!(attack.status_effects.len(), 0);
        assert_eq!(attack.target_type, AttackTargetType::Active);
    }

    #[test]
    fn test_attack_with_status() {
        let attack = Attack::with_status(
            "Poison Sting".to_string(),
            vec![EnergyType::Grass],
            10,
            StatusCondition::Poison,
            75,
        );
        
        assert_eq!(attack.name, "Poison Sting");
        assert_eq!(attack.damage, 10);
        assert_eq!(attack.status_effects.len(), 1);
        assert_eq!(attack.status_effects[0].condition, StatusCondition::Poison);
        assert_eq!(attack.status_effects[0].probability, 75);
    }

    #[test]
    fn test_coin_flip_attack() {
        let attack = Attack::coin_flip_damage(
            "Double Slap".to_string(),
            vec![EnergyType::Colorless, EnergyType::Colorless],
            0,
            20,
            2,
        );
        
        assert_eq!(attack.name, "Double Slap");
        assert_eq!(attack.damage, 0);
        
        if let Some(DamageMode::CoinFlip { per_heads, flips }) = &attack.damage_mode {
            assert_eq!(*per_heads, 20);
            assert_eq!(*flips, 2);
        } else {
            panic!("Expected CoinFlip damage mode");
        }
    }

    #[test]
    fn test_damage_calculation() {
        // Test fixed damage
        let simple_attack = Attack::simple(
            "Tackle".to_string(),
            vec![EnergyType::Colorless],
            30,
        );
        assert_eq!(simple_attack.calculate_damage(0, &[]), 30);
        
        // Test coin flip damage
        let coin_attack = Attack::coin_flip_damage(
            "Double Slap".to_string(),
            vec![EnergyType::Colorless],
            10,
            15,
            2,
        );
        // Two heads
        assert_eq!(coin_attack.calculate_damage(0, &[true, true]), 40);
        // One head
        assert_eq!(coin_attack.calculate_damage(0, &[true, false]), 25);
        // No heads
        assert_eq!(coin_attack.calculate_damage(0, &[false, false]), 10);
    }

    #[test]
    fn test_status_condition_types() {
        let conditions = vec![
            StatusCondition::Poison,
            StatusCondition::Burn,
            StatusCondition::Paralysis,
            StatusCondition::Sleep,
            StatusCondition::Confusion,
        ];
        
        for condition in conditions {
            let effect = StatusEffect {
                condition: condition.clone(),
                probability: 100,
                target: "defending".to_string(),
            };
            assert_eq!(effect.condition, condition);
        }
    }

    #[test]
    fn test_attack_target_types() {
        let targets = vec![
            AttackTargetType::Active,
            AttackTargetType::Choose,
            AttackTargetType::All,
            AttackTargetType::Bench,
            AttackTargetType::Self_,
        ];
        
        for target in targets {
            let mut attack = Attack::simple(
                "Test Attack".to_string(),
                vec![EnergyType::Colorless],
                10,
            );
            attack.set_target_type(target.clone());
            assert_eq!(attack.target_type, target);
        }
    }
}
