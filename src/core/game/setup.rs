//! Game setup logic
//!
//! This module contains all the functions needed to set up a game, including:
//! - Player setup
//! - Deck assignment
//! - Turn order determination
//! - Initial hand dealing
//! - Mulligan handling

use crate::core::{
    card::{CardId, EvolutionStage},
    deck::Deck,
    game::state::{Game, GameState},
    player::{Player, PlayerId},
};

/// 穆勒规则重抽结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MulliganResult {
    /// 双方都没有基础宝可梦
    AllWithoutBasic,
    /// 双方都有基础宝可梦
    AllWithBasic,
    /// 其中一方没有基础宝可梦，包含该玩家ID
    OneWithoutBasic(PlayerId),
}

impl Game {
    /// Add a player to the game
    pub fn add_player(&mut self, mut player: Player) -> Result<(), String> {
        if self.state != GameState::Setup {
            return Err("Cannot add players after game has started".to_string());
        }

        if self.players.len() >= 2 {
            return Err("Maximum of 2 players allowed".to_string());
        }

        // Set prize cards according to game rules
        player.prize_cards = self.rules.prize_cards;

        let player_id = player.id;
        self.players.insert(player_id, player);

        Ok(())
    }

    /// Set a player's deck
    pub fn set_player_deck(&mut self, player_id: PlayerId, deck: Deck) -> Result<(), String> {
        if self.state != GameState::Setup {
            return Err("Cannot set deck after game has started".to_string());
        }

        // Add deck cards to the game's card database
        for &_card_id in deck.cards.keys() {
            // In a real implementation, you'd load the card data here
            // For now, we'll assume the cards are already in the database
        }

        if let Some(player) = self.players.get_mut(&player_id) {
            let shuffled_cards = deck.shuffle();
            player.set_deck(shuffled_cards);
            Ok(())
        } else {
            Err("Player not found".to_string())
        }
    }

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

    /// 阶段5a: 玩家宣告没有基础宝可梦
    /// 返回值：(需要重抽的玩家列表, 是否双方都没有基础宝可梦)
    pub fn declare_no_basic_pokemon(&mut self) -> Result<(Vec<PlayerId>, bool), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only declare no basic Pokemon during setup phase".to_string());
        }

        let players_without_basic = self.check_for_basic_pokemon()?;
        let all_players: Vec<PlayerId> = self.players.keys().cloned().collect();

        // 检查是否所有玩家都没有基础宝可梦
        let all_without_basic = players_without_basic.len() == all_players.len();

        Ok((players_without_basic, all_without_basic))
    }

    /// 阶段5b: 记录需要等待重抽的玩家
    /// 当只有一方没有基础宝可梦时调用此方法
    pub fn mark_player_for_mulligan(&mut self, player_id: PlayerId) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only mark player for mulligan during setup phase".to_string());
        }

        // 检查玩家是否存在
        if !self.players.contains_key(&player_id) {
            return Err("Player not found".to_string());
        }

        // 记录需要等待重抽的玩家
        self.player_waiting_for_mulligan = Some(player_id);

        Ok(())
    }

    /// 在对手完成设置后调用此方法
    pub fn perform_pending_mulligans(&mut self) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only perform mulligans during setup phase".to_string());
        }

        // 记录执行重抽的次数，用于奖赏卡补偿
        let mulligan_count = if self.player_waiting_for_mulligan.is_some() {
            1
        } else {
            0
        };

        // 为等待重抽的玩家执行重抽
        if let Some(player_id) = self.player_waiting_for_mulligan {
            // 将手牌放回牌库底部
            if let Some(player) = self.players.get_mut(&player_id) {
                for card_id in player.hand.drain(..) {
                    player.deck.push(card_id);
                }
                player.shuffle_deck();

                // 重新抽取7张牌
                player.draw_cards(7);
            }
        }

        // 清空等待列表
        self.player_waiting_for_mulligan = None;

        // 记录重抽次数，用于奖赏卡补偿
        self.mulligan_count += mulligan_count;

        Ok(())
    }

    /// 对指定玩家执行重抽并检查是否包含基础宝可梦
    pub fn perform_mulligan_and_check_basic_pokemon(
        &mut self,
        player_id: PlayerId,
    ) -> Result<bool, String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only perform mulligan during setup phase".to_string());
        }

        // 检查玩家是否存在
        if !self.players.contains_key(&player_id) {
            return Err("Player not found".to_string());
        }

        // 执行重抽
        self.perform_mulligan(player_id)?;

        // 记录重抽次数
        self.mulligan_count += 1;

        // 检查玩家是否已有基础宝可梦
        if let Some(player) = self.players.get(&player_id) {
            let basic_pokemon = player.find_basic_pokemon_in_hand(&self.card_database);
            Ok(!basic_pokemon.is_empty())
        } else {
            Ok(false)
        }
    }

    /// 对双方玩家执行重抽并检查基础宝可梦状态
    /// 返回值:
    /// - Ok(MulliganResult::AllWithoutBasic): 双方都没有基础宝可梦
    /// - Ok(MulliganResult::AllWithBasic): 双方都有基础宝可梦
    /// - Ok(MulliganResult::OneWithoutBasic(player_id)): 其中一方没有基础宝可梦，返回该玩家ID
    pub fn perform_mulligan_for_both_and_check_basic_pokemon(
        &mut self,
    ) -> Result<MulliganResult, String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only perform mulligan during setup phase".to_string());
        }

        // 获取所有玩家ID
        let player_ids: Vec<PlayerId> = self.players.keys().cloned().collect();
        let mut player_without_basic_pokemon = None;
        let mut all_without_basic = false;

        // 对所有玩家执行重抽
        for &player_id in &player_ids {
            self.perform_mulligan(player_id)?;

            // 检查玩家是否已有基础宝可梦
            if let Some(player) = self.players.get(&player_id) {
                let basic_pokemon = player.find_basic_pokemon_in_hand(&self.card_database);
                if basic_pokemon.is_empty() {
                    if player_without_basic_pokemon == None {
                        player_without_basic_pokemon = Some(player_id);
                    } else {
                        all_without_basic = true;
                    }
                }
            }
        }

        // 根据检查结果返回不同状态
        if all_without_basic {
            // 所有玩家都没有基础宝可梦
            Ok(MulliganResult::AllWithoutBasic)
        } else if player_without_basic_pokemon.is_none() {
            // 所有玩家都有基础宝可梦
            Ok(MulliganResult::AllWithBasic)
        } else {
            // 部分玩家有基础宝可梦，返回没有基础宝可梦的玩家ID
            Ok(MulliganResult::OneWithoutBasic(
                player_without_basic_pokemon.unwrap(),
            ))
        }
    }

    /// 获取玩家可以声明的穆勒补偿卡牌数量上限
    /// 这个数量等于对手执行重新抽取手牌的次数
    pub fn get_mulligan_compensation_limit(&self, _player_id: PlayerId) -> Result<usize, String> {
        // 在实际实现中，这里应该跟踪每个玩家执行重新抽取手牌的次数
        // 简化处理，返回一个固定值
        Ok(self.mulligan_count)
    }

    /// 处理穆勒规则中的奖赏卡补偿
    /// 当对手执行了重新抽取手牌操作后，可以抽取相应数量的卡牌作为补偿
    pub fn mulligan_compensation(
        &mut self,
        player_id: PlayerId,
        card_count: usize,
    ) -> Result<Vec<CardId>, String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only perform mulligan compensation during setup phase".to_string());
        }

        // 检查声明的卡牌数量是否超过上限
        let limit = self.get_mulligan_compensation_limit(player_id)?;
        if card_count > limit {
            return Err(format!(
                "Declared card count {} exceeds limit {}",
                card_count, limit
            ));
        }

        // 获取玩家
        let player = self
            .players
            .get_mut(&player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        // 抽取指定数量的卡牌
        let drawn_cards = player.draw_cards(card_count);

        Ok(drawn_cards)
    }

    /// 阶段4: 玩家执行重新抽取手牌操作（穆勒规则）
    pub fn perform_mulligan(&mut self, player_id: PlayerId) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only perform mulligan during setup phase".to_string());
        }

        // 获取玩家
        let player = self
            .players
            .get_mut(&player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        // 将手牌放回牌库底部（简化处理）
        for card_id in player.hand.drain(..) {
            player.deck.push(card_id);
        }

        player.shuffle_deck();

        // 重新抽取7张牌
        player.draw_cards(7);

        Ok(())
    }

    /// 阶段5: 玩家选择活跃宝可梦
    pub fn select_active_pokemon(
        &mut self,
        player_id: PlayerId,
        pokemon_id: CardId,
    ) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only select active Pokemon during setup phase".to_string());
        }

        // 获取玩家
        let player = self
            .players
            .get_mut(&player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        // 检查选择的卡牌是否在玩家手牌中
        if !player.hand.contains(&pokemon_id) {
            return Err("Selected Pokemon is not in player's hand".to_string());
        }

        // 检查选择的卡牌是否是基础宝可梦
        if let Some(card) = self.card_database.get(&pokemon_id) {
            if !card.is_pokemon() {
                return Err("Selected card is not a Pokemon".to_string());
            }

            // 检查是否是基础宝可梦
            if let crate::core::card::CardType::Pokemon {
                stage: EvolutionStage::Basic,
                ..
            } = card.card_type
            {
                // 设置为活跃宝可梦
                player.set_active_pokemon(pokemon_id);
            } else {
                return Err("Selected Pokemon is not a Basic Pokemon".to_string());
            }
        } else {
            return Err("Card not found in database".to_string());
        }

        Ok(())
    }

    /// 阶段6: 玩家设置备战区宝可梦
    pub fn setup_bench(
        &mut self,
        player_id: PlayerId,
        pokemon_ids: Vec<CardId>,
    ) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only setup bench during setup phase".to_string());
        }

        // 获取玩家
        let player = self
            .players
            .get_mut(&player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        // 设置备战区宝可梦
        for &pokemon_id in &pokemon_ids {
            // 检查卡牌是否在玩家手牌中
            if !player.hand.contains(&pokemon_id) {
                return Err("Selected Pokemon is not in player's hand".to_string());
            }

            // 检查卡牌是否是宝可梦
            if let Some(card) = self.card_database.get(&pokemon_id) {
                if !card.is_pokemon() {
                    return Err("Selected card is not a Pokemon".to_string());
                }

                // 尝试将宝可梦放到备战区
                if !player.bench_pokemon(pokemon_id) {
                    return Err("Failed to place Pokemon on bench".to_string());
                }
            } else {
                return Err("Card not found in database".to_string());
            }
        }

        Ok(())
    }

    /// 阶段7: 放置奖赏卡
    pub fn place_prize_cards(&mut self) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only place prize cards during setup phase".to_string());
        }

        // 为每个玩家放置6张奖赏卡
        for player in self.players.values_mut() {
            // 从牌库顶部拿6张卡作为奖赏卡
            let prize_cards = player.draw_prize_cards(6);
            // 在实际实现中，这些卡牌会被放置在奖赏卡区域
            // 这里简化处理，只是设置奖赏卡数量
            player.prize_cards = prize_cards.len() as u32;
        }

        Ok(())
    }

    /// 打印玩家手牌，用于穆勒规则重抽时让对手查看
    pub fn print_player_hand(&self, player_id: PlayerId) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only print player hand during setup phase".to_string());
        }

        // 获取玩家
        if let Some(player) = self.players.get(&player_id) {
            println!("Player {}'s hand:", player.name);
            for (index, card_id) in player.hand.iter().enumerate() {
                if let Some(card) = self.card_database.get(card_id) {
                    println!("  {}. {} ({})", index + 1, card.name, card_id);
                } else {
                    println!("  {}. Unknown card ({})", index + 1, card_id);
                }
            }
            Ok(())
        } else {
            Err("Player not found".to_string())
        }
    }

    /// 宣告没有基础宝可梦并执行穆勒规则重抽流程
    /// 这个方法会打印双方手牌，让对手确认
    pub fn declare_and_perform_mulligan(&mut self, player_id: PlayerId) -> Result<MulliganResult, String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only declare mulligan during setup phase".to_string());
        }

        // 检查玩家是否存在
        if !self.players.contains_key(&player_id) {
            return Err("Player not found".to_string());
        }

        // 打印宣告重抽的玩家手牌
        println!("Player declared no basic Pokemon. Showing hands to opponent:");
        self.print_player_hand(player_id)?;

        // 打印对手手牌
        for (&id, _player) in &self.players {
            if id != player_id {
                println!("Opponent's hand:");
                self.print_player_hand(id)?;
                break;
            }
        }

        // 执行重抽
        self.perform_mulligan(player_id)?;

        // 检查重抽后是否已有基础宝可梦
        if let Some(player) = self.players.get(&player_id) {
            let basic_pokemon = player.find_basic_pokemon_in_hand(&self.card_database);
            if basic_pokemon.is_empty() {
                // 仍然没有基础宝可梦
                Ok(MulliganResult::OneWithoutBasic(player_id))
            } else {
                // 现在有了基础宝可梦
                Ok(MulliganResult::AllWithBasic)
            }
        } else {
            Err("Player not found after mulligan".to_string())
        }
    }

    /// 阶段8: 完成设置，开始游戏
    pub fn complete_setup(&mut self) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only complete setup during setup phase".to_string());
        }

        // 验证所有玩家都已完成设置
        for player in self.players.values() {
            // 检查每个玩家都有活跃宝可梦
            if player.active_pokemon.is_none() {
                return Err("All players must have an active Pokemon".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::player::Player;
    use uuid::Uuid;

    #[test]
    fn test_mark_player_for_mulligan() {
        let mut game = Game::new();
        let player = Player::new("Alice".to_string());
        let player_id = player.id;
        assert!(game.add_player(player).is_ok());

        // Test marking player for mulligan during setup phase
        assert!(game.mark_player_for_mulligan(player_id).is_ok());
        assert_eq!(game.player_waiting_for_mulligan, Some(player_id));

        // Test marking the same player again (should overwrite)
        assert!(game.mark_player_for_mulligan(player_id).is_ok());
        assert_eq!(game.player_waiting_for_mulligan, Some(player_id));
    }

    #[test]
    fn test_mark_player_for_mulligan_wrong_phase() {
        let mut game = Game::new();
        let player = Player::new("Alice".to_string());
        let player_id = player.id;
        assert!(game.add_player(player).is_ok());

        // Move game to in-progress state
        game.state = GameState::InProgress;

        // Test marking player for mulligan when not in setup phase
        assert!(game.mark_player_for_mulligan(player_id).is_err());
    }

    #[test]
    fn test_mark_player_for_mulligan_nonexistent_player() {
        let mut game = Game::new();
        let fake_player_id = Uuid::new_v4();

        // Test marking non-existent player for mulligan
        assert!(game.mark_player_for_mulligan(fake_player_id).is_err());
    }

    #[test]
    fn test_perform_pending_mulligans() {
        let mut game = Game::new();
        let player = Player::new("Alice".to_string());
        let player_id = player.id;
        assert!(game.add_player(player).is_ok());

        // Mark player for mulligan
        assert!(game.mark_player_for_mulligan(player_id).is_ok());

        // Perform pending mulligans
        assert!(game.perform_pending_mulligans().is_ok());

        // Check that player is no longer waiting for mulligan
        assert_eq!(game.player_waiting_for_mulligan, None);
        assert_eq!(game.mulligan_count, 1);
    }

    #[test]
    fn test_perform_mulligan_and_check_basic_pokemon() {
        let mut game = Game::new();
        let player = Player::new("Alice".to_string());
        let player_id = player.id;
        assert!(game.add_player(player).is_ok());

        // Test performing mulligan and checking for basic pokemon
        let result = game.perform_mulligan_and_check_basic_pokemon(player_id);
        assert!(result.is_ok());
        // mulligan_count should be 1
        assert_eq!(game.mulligan_count, 1);
    }

    #[test]
    fn test_perform_mulligan_for_both_and_check_basic_pokemon() {
        let mut game = Game::new();
        let player1 = Player::new("Alice".to_string());
        let player2 = Player::new("Bob".to_string());
        assert!(game.add_player(player1).is_ok());
        assert!(game.add_player(player2).is_ok());

        // Test performing mulligan for both and checking for basic pokemon
        let result = game.perform_mulligan_for_both_and_check_basic_pokemon();
        assert!(result.is_ok());
    }
}
