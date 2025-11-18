//! Turn setup functionality

use crate::core::{
    game::state::{Game, GameState},
    player::PlayerId,
};

impl Game {
    /// 阶段1: 通过猜拳决定先后手顺序
    pub fn determine_turn_order(&mut self) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only determine turn order during setup phase".to_string());
        }

        // 在实际实现中，这里应该有一个随机化过程来决定先后手
        // 简单起见，我们保持当前顺序，但在真实游戏中应该通过抛硬币等方式决定
        for &player_id in self.players.keys() {
            self.turn_order.push(player_id);
        }

        self.turn_order.swap(0, 1); // 示例：交换两名玩家的顺序

        Ok(())
    }
}