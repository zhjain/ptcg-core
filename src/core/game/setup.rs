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
    pub fn mark_player_for_mulligan(&mut self, _player_id: PlayerId) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only mark player for mulligan during setup phase".to_string());
        }

        // 检查玩家是否存在
        // if !self.players.contains_key(&player_id) {
        //     return Err("Player not found".to_string());
        // }

        // 记录需要等待重抽的玩家
        // if !self.players_waiting_for_mulligan.contains(&player_id) {
        //     self.players_waiting_for_mulligan.push(player_id);
        // }
        
        Ok(())
    }

    /// 执行等待中的重抽操作
    /// 在对手完成设置后调用此方法
    pub fn perform_pending_mulligans(&mut self) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only perform mulligans during setup phase".to_string());
        }

        // 记录执行重抽的次数，用于奖赏卡补偿
        let mulligan_count = self.players_waiting_for_mulligan.len();

        // 为每个等待重抽的玩家执行重抽
        for &player_id in &self.players_waiting_for_mulligan {
            // 将手牌放回牌库底部
            if let Some(player) = self.players.get_mut(&player_id) {
                for card_id in player.hand.drain(..) {
                    player.deck.push(card_id);
                }
                
                // 重新抽取7张牌
                player.draw_cards(7);
            }
        }

        // 清空等待列表
        self.players_waiting_for_mulligan.clear();
        
        // 记录重抽次数，用于奖赏卡补偿
        self.mulligan_counts.push(mulligan_count);
        
        Ok(())
    }

    /// 获取玩家可以声明的穆勒补偿卡牌数量上限
    /// 这个数量等于对手执行重新抽取手牌的次数
    pub fn get_mulligan_compensation_limit(&self, _player_id: PlayerId) -> Result<usize, String> {
        // 在实际实现中，这里应该跟踪每个玩家执行重新抽取手牌的次数
        // 简化处理，返回一个固定值
        Ok(self.mulligan_counts.iter().sum())
    }

    /// 处理穆勒规则中的奖赏卡补偿
    /// 当对手执行了重新抽取手牌操作后，可以抽取相应数量的卡牌作为补偿
    pub fn mulligan_compensation(&mut self, player_id: PlayerId, card_count: usize) -> Result<Vec<CardId>, String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only perform mulligan compensation during setup phase".to_string());
        }

        // 检查声明的卡牌数量是否超过上限
        let limit = self.get_mulligan_compensation_limit(player_id)?;
        if card_count > limit {
            return Err(format!("Declared card count {} exceeds limit {}", card_count, limit));
        }

        // 获取玩家
        let player = self.players.get_mut(&player_id)
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
        let player = self.players.get_mut(&player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        // 将手牌放回牌库底部（简化处理）
        for card_id in player.hand.drain(..) {
            player.deck.push(card_id);
        }

        // 重新抽取7张牌
        player.draw_cards(7);

        Ok(())
    }

    /// 阶段5: 玩家选择活跃宝可梦
    pub fn select_active_pokemon(&mut self, player_id: PlayerId, pokemon_id: CardId) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only select active Pokemon during setup phase".to_string());
        }

        // 获取玩家
        let player = self.players.get_mut(&player_id)
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
            if let crate::core::card::CardType::Pokemon { stage: EvolutionStage::Basic, .. } = card.card_type {
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
    pub fn setup_bench(&mut self, player_id: PlayerId, pokemon_ids: Vec<CardId>) -> Result<(), String> {
        // 检查当前是否处于设置阶段
        if self.state != GameState::Setup {
            return Err("Can only setup bench during setup phase".to_string());
        }

        // 获取玩家
        let player = self.players.get_mut(&player_id)
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