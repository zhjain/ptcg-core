//! Pokemon setup functionality

use crate::core::{
    game::state::{Game, GameState},
    player::PlayerId,
};
use crate::core::card::CardId;

impl Game {
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
                stage: crate::core::card::EvolutionStage::Basic,
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
    pub fn declare_and_perform_mulligan(
        &mut self,
        player_id: PlayerId,
    ) -> Result<MulliganResult, String> {
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
        for &id in self.players.keys() {
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