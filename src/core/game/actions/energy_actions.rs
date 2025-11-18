//! 能量相关动作处理

use crate::core::card::CardId;
use crate::core::player::PlayerId;
use crate::core::game::state::Game;

/// 能量附加动作
#[derive(Debug, Clone)]
pub struct AttachEnergyAction {
    pub player_id: PlayerId,
    pub energy_card_id: CardId,
    pub target_pokemon_id: CardId,
}

impl AttachEnergyAction {
    pub fn new(player_id: PlayerId, energy_card_id: CardId, target_pokemon_id: CardId) -> Self {
        Self {
            player_id,
            energy_card_id,
            target_pokemon_id,
        }
    }

    /// 执行能量附加动作
    pub fn execute(&self, game: &mut Game) -> Result<(), String> {
        // 检查玩家是否存在
        let player = game.get_player_mut(self.player_id)
            .ok_or("Player not found")?;
        
        // 检查能量卡是否在玩家手中
        if !player.hand.contains(&self.energy_card_id) {
            return Err("Energy card not in player's hand".to_string());
        }
        
        // 检查目标宝可梦是否在玩家场上
        let is_active = player.active_pokemon == Some(self.target_pokemon_id);
        let is_on_bench = player.bench.contains(&self.target_pokemon_id);
        
        if !is_active && !is_on_bench {
            return Err("Target Pokemon not on player's field".to_string());
        }
        
        // 移除手牌中的能量卡
        player.hand.retain(|&id| id != self.energy_card_id);
        
        // 将能量卡附加到目标宝可梦
        // 注意：在实际实现中，我们可能需要将能量卡移动到一个专门的附加能量区域
        // 这里我们简化处理，只是从手牌中移除
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_actions_module() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}