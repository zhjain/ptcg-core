//! 宝可梦卡牌的能力相关结构和功能

use serde::{Deserialize, Serialize};

/// 宝可梦卡牌的能力信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ability {
    /// 能力名称
    pub name: String,
    /// 能力效果描述
    pub effect: String,
    /// 能力类型（能力、宝可梦力量、宝可梦身体等）
    pub ability_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_creation() {
        let ability = Ability {
            name: "Static".to_string(),
            effect: "Whenever this Pokémon is hit by a Lightning attack, the Attacking Pokémon is now Paralyzed.".to_string(),
            ability_type: "Pokémon Power".to_string(),
        };
        
        assert_eq!(ability.name, "Static");
        assert_eq!(ability.ability_type, "Pokémon Power");
    }
}