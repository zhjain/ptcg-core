//! Deck setup functionality

use crate::core::{
    game::state::{Game, GameState},
    player::PlayerId,
};

impl Game {
    /// Start the game setup process
    pub fn start_setup(&mut self) -> Result<(), String> {
        if self.state != GameState::Setup {
            return Err("Game is not in setup state".to_string());
        }

        if self.players.len() < 2 {
            return Err("Need at least 2 players to start setup".to_string());
        }

        // Validate all players have decks
        for player in self.players.values() {
            if player.deck.is_empty() {
                return Err("All players must have decks".to_string());
            }
        }

        Ok(())
    }

    /// 阶段2: 抽取初始手牌
    pub fn deal_opening_hands(&mut self) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only deal opening hands during setup phase".to_string());
        }

        // 检查是否已经确定了先后手顺序
        if self.turn_order.is_empty() {
            return Err("Turn order must be determined before dealing hands".to_string());
        }

        // 执行发牌逻辑
        for player in self.players.values_mut() {
            player.draw_cards(7);
        }

        Ok(())
    }

    /// 阶段3: 检查玩家是否拥有基础宝可梦
    pub fn check_for_basic_pokemon(&self) -> Result<Vec<PlayerId>, String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only check for basic Pokemon during setup phase".to_string());
        }

        let mut players_without_basic = Vec::new();

        for (&player_id, player) in &self.players {
            let basic_pokemon = player.find_basic_pokemon_in_hand(&self.card_database);
            if basic_pokemon.is_empty() {
                players_without_basic.push(player_id);
            }
        }

        Ok(players_without_basic)
    }
}