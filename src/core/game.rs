//! 主要游戏逻辑和状态管理

use crate::core::{
    card::{Card, CardId, CardType, EvolutionStage},
    deck::Deck,
    player::{Player, PlayerId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a game
pub type GameId = Uuid;

/// Represents the current phase of a turn
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    /// Beginning of turn (draw card, flip coins for special conditions)
    BeginningOfTurn,
    /// Main phase (play cards, attach energy, evolve Pokemon)
    Main,
    /// Attack phase
    Attack,
    /// End of turn (apply poison/burn damage, check for win conditions)
    EndOfTurn,
}

/// Represents the overall state of the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    /// Game is being set up
    Setup,
    /// Game is actively being played
    InProgress,
    /// Game has ended
    Finished { winner: Option<PlayerId> },
    /// Game was abandoned or cancelled
    Cancelled,
}

/// Represents the setup phase of the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupPhase {
    /// Initial state - waiting to determine turn order
    WaitingForTurnOrder,
    /// Turn order determined - waiting to deal hands
    WaitingForHands,
    /// Hands dealt - checking for basic Pokemon
    CheckingForBasicPokemon,
    /// Players need to mulligan (no basic Pokemon)
    MulliganRequired,
    /// Players selecting active Pokemon
    SelectingActivePokemon,
    /// Players setting up bench
    SettingUpBench,
    /// Placing prize cards
    PlacingPrizeCards,
    /// Setup complete - ready to start game
    SetupComplete,
}

/// Main game structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Unique identifier for this game
    pub id: GameId,
    /// Current state of the game
    pub state: GameState,
    /// Current phase of the turn
    pub phase: GamePhase,
    /// All players in the game
    pub players: HashMap<PlayerId, Player>,
    /// Player turn order
    pub turn_order: Vec<PlayerId>,
    /// Index of the current player in turn_order
    pub current_player_index: usize,
    /// All cards used in this game
    pub card_database: HashMap<CardId, Card>,
    /// Turn counter
    pub turn_number: u32,
    /// Game rules and settings
    pub rules: GameRules,
    /// Game history/log
    pub history: Vec<GameEvent>,
    /// Players waiting for mulligan after opponent completes setup
    pub players_waiting_for_mulligan: Vec<PlayerId>,
    /// Record of mulligan counts for each player
    pub mulligan_counts: Vec<usize>,
}

/// Game rules and settings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameRules {
    /// Format being played (Standard, Expanded, etc.)
    pub format: String,
    /// Number of prize cards each player starts with
    pub prize_cards: u32,
    /// Maximum hand size (usually unlimited in PTCG)
    pub max_hand_size: Option<u32>,
    /// Time limit per turn (in seconds)
    pub turn_time_limit: Option<u32>,
    /// Whether to use automatic deck shuffling
    pub auto_shuffle: bool,
}

/// Events that can occur during a game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    /// Game started
    GameStarted,
    /// Turn started
    TurnStarted {
        player_id: PlayerId,
        turn_number: u32,
    },
    /// Card was drawn
    CardDrawn {
        player_id: PlayerId,
        card_id: Option<CardId>,
    },
    /// Card was played
    CardPlayed {
        player_id: PlayerId,
        card_id: CardId,
    },
    /// Pokemon was played to bench
    PokemonBenched {
        player_id: PlayerId,
        card_id: CardId,
    },
    /// Energy was attached
    EnergyAttached {
        player_id: PlayerId,
        energy_id: CardId,
        pokemon_id: CardId,
    },
    /// Attack was used
    AttackUsed {
        player_id: PlayerId,
        pokemon_id: CardId,
        attack_name: String,
    },
    /// Damage was dealt
    DamageDealt {
        player_id: PlayerId,
        pokemon_id: CardId,
        damage: u32,
    },
    /// Pokemon was knocked out
    PokemonKnockedOut {
        player_id: PlayerId,
        pokemon_id: CardId,
    },
    /// Prize card was taken
    PrizeTaken { player_id: PlayerId },
    /// Turn ended
    TurnEnded { player_id: PlayerId },
    /// Game ended
    GameEnded { winner: Option<PlayerId> },
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            format: "Standard".to_string(),
            prize_cards: 6,
            max_hand_size: None,
            turn_time_limit: None,
            auto_shuffle: true,
        }
    }
}

impl Game {
    /// Create a new game with default rules
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            state: GameState::Setup,
            phase: GamePhase::BeginningOfTurn,
            players: HashMap::new(),
            turn_order: Vec::new(),
            current_player_index: 0,
            card_database: HashMap::new(),
            turn_number: 1,
            rules: GameRules::default(),
            history: Vec::new(),
            players_waiting_for_mulligan: Vec::new(),
            mulligan_counts: Vec::new(),
        }
    }

    /// Create a new game with custom rules
    pub fn with_rules(rules: GameRules) -> Self {
        let mut game = Self::new();
        game.rules = rules;
        game
    }

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

    /// Start the game
    pub fn start(&mut self) -> Result<(), String> {
        if self.state != GameState::Setup {
            return Err("Game is not in setup state".to_string());
        }

        if self.players.len() < 2 {
            return Err("Need at least 2 players to start".to_string());
        }

        // Validate all players have decks
        for player in self.players.values() {
            if player.deck.is_empty() {
                return Err("All players must have decks".to_string());
            }
        }

        self.state = GameState::InProgress;
        self.add_event(GameEvent::GameStarted);

        // Deal opening hands
        for player in self.players.values_mut() {
            player.draw_cards(7);
        }

        // Start the first turn
        self.start_turn()?;

        Ok(())
    }

    /// Start a new turn
    pub fn start_turn(&mut self) -> Result<(), String> {
        if self.state != GameState::InProgress {
            return Err("Game is not in progress".to_string());
        }

        let current_player_id = self.get_current_player_id()?;

        if let Some(player) = self.players.get_mut(&current_player_id) {
            player.start_turn();
            player.draw_card(); // Draw card at beginning of turn
        }

        self.phase = GamePhase::BeginningOfTurn;
        self.add_event(GameEvent::TurnStarted {
            player_id: current_player_id,
            turn_number: self.turn_number,
        });

        self.add_event(GameEvent::CardDrawn {
            player_id: current_player_id,
            card_id: None, // In a real game, you'd track which card was drawn
        });

        Ok(())
    }

    /// End the current turn and move to the next player
    pub fn end_turn(&mut self) -> Result<(), String> {
        if self.state != GameState::InProgress {
            return Err("Game is not in progress".to_string());
        }

        let current_player_id = self.get_current_player_id()?;

        if let Some(player) = self.players.get_mut(&current_player_id) {
            player.end_turn();
        }

        self.add_event(GameEvent::TurnEnded {
            player_id: current_player_id,
        });

        // Check for win conditions
        if self.check_win_conditions()? {
            return Ok(());
        }

        // Move to next player
        self.current_player_index = (self.current_player_index + 1) % self.turn_order.len();

        // Increment turn number when we complete a full round
        if self.current_player_index == 0 {
            self.turn_number += 1;
        }

        self.start_turn()?;

        Ok(())
    }

    /// Get the current player's ID
    pub fn get_current_player_id(&self) -> Result<PlayerId, String> {
        self.turn_order
            .get(self.current_player_index)
            .copied()
            .ok_or_else(|| "No current player".to_string())
    }

    /// Get the current player
    pub fn get_current_player(&self) -> Result<&Player, String> {
        let player_id = self.get_current_player_id()?;
        self.players
            .get(&player_id)
            .ok_or_else(|| "Current player not found".to_string())
    }

    /// Get a mutable reference to the current player
    pub fn get_current_player_mut(&mut self) -> Result<&mut Player, String> {
        let player_id = self.get_current_player_id()?;
        self.players
            .get_mut(&player_id)
            .ok_or_else(|| "Current player not found".to_string())
    }

    /// Advance to the next phase
    pub fn next_phase(&mut self) -> Result<(), String> {
        self.phase = match self.phase {
            GamePhase::BeginningOfTurn => GamePhase::Main,
            GamePhase::Main => GamePhase::Attack,
            GamePhase::Attack => GamePhase::EndOfTurn,
            GamePhase::EndOfTurn => {
                self.end_turn()?;
                return Ok(());
            }
        };
        Ok(())
    }

    /// Check for win conditions
    pub fn check_win_conditions(&mut self) -> Result<bool, String> {
        let mut winner = None;

        for (&player_id, player) in &self.players {
            if player.has_won() {
                winner = Some(player_id);
                break;
            }

            // Check if opponent has lost
            let opponent_lost = self
                .players
                .values()
                .any(|p| p.id != player_id && p.has_lost());

            if opponent_lost {
                winner = Some(player_id);
                break;
            }
        }

        if let Some(winner_id) = winner {
            self.state = GameState::Finished {
                winner: Some(winner_id),
            };
            self.add_event(GameEvent::GameEnded {
                winner: Some(winner_id),
            });
            return Ok(true);
        }

        Ok(false)
    }

    /// Add a card to the game's database
    pub fn add_card_to_database(&mut self, card: Card) {
        self.card_database.insert(card.id, card);
    }

    /// Get a card from the database
    pub fn get_card(&self, card_id: CardId) -> Option<&Card> {
        self.card_database.get(&card_id)
    }

    /// Add an event to the game history
    pub fn add_event(&mut self, event: GameEvent) {
        self.history.push(event);
    }

    /// Get the game history
    pub fn get_history(&self) -> &[GameEvent] {
        &self.history
    }

    /// Check if it's a specific player's turn
    pub fn is_player_turn(&self, player_id: PlayerId) -> bool {
        self.get_current_player_id()
            .map(|id| id == player_id)
            .unwrap_or(false)
    }

    /// Get all players
    pub fn get_players(&self) -> &HashMap<PlayerId, Player> {
        &self.players
    }

    /// Get a specific player
    pub fn get_player(&self, player_id: PlayerId) -> Option<&Player> {
        self.players.get(&player_id)
    }

    /// Get a specific player (mutable)
    pub fn get_player_mut(&mut self, player_id: PlayerId) -> Option<&mut Player> {
        self.players.get_mut(&player_id)
    }

    /// Force end the game
    pub fn end_game(&mut self, winner: Option<PlayerId>) {
        self.state = GameState::Finished { winner };
        self.add_event(GameEvent::GameEnded { winner });
    }

    /// Cancel the game
    pub fn cancel_game(&mut self) {
        self.state = GameState::Cancelled;
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
        if !self.players_waiting_for_mulligan.contains(&player_id) {
            self.players_waiting_for_mulligan.push(player_id);
        }
        
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
    pub fn get_mulligan_compensation_limit(&self, player_id: PlayerId) -> Result<usize, String> {
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
            if let CardType::Pokemon { stage: EvolutionStage::Basic, .. } = card.card_type {
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

        // 设置游戏状态为进行中
        self.state = GameState::InProgress;
        self.add_event(GameEvent::GameStarted);

        // 开始第一个回合
        self.start_turn()?;

        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_game() {
        let game = Game::new();
        assert_eq!(game.state, GameState::Setup);
        assert_eq!(game.players.len(), 0);
        assert_eq!(game.turn_number, 1);
    }

    #[test]
    fn test_add_players() {
        let mut game = Game::new();
        let player1 = Player::new("Player 1".to_string());
        let player2 = Player::new("Player 2".to_string());

        assert!(game.add_player(player1).is_ok());
        assert!(game.add_player(player2).is_ok());
        assert_eq!(game.players.len(), 2);
    }

    #[test]
    fn test_too_many_players() {
        let mut game = Game::new();
        let player1 = Player::new("Player 1".to_string());
        let player2 = Player::new("Player 2".to_string());
        let player3 = Player::new("Player 3".to_string());

        assert!(game.add_player(player1).is_ok());
        assert!(game.add_player(player2).is_ok());
        assert!(game.add_player(player3).is_err());
    }

    #[test]
    fn test_game_phases() {
        let mut game = Game::new();
        assert_eq!(game.phase, GamePhase::BeginningOfTurn);

        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, GamePhase::Main);

        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, GamePhase::Attack);

        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, GamePhase::EndOfTurn);
    }

    #[test]
    fn test_custom_rules() {
        let rules = GameRules {
            format: "Expanded".to_string(),
            prize_cards: 4,
            max_hand_size: Some(10),
            turn_time_limit: Some(300),
            auto_shuffle: false,
        };

        let game = Game::with_rules(rules.clone());
        assert_eq!(game.rules, rules);
    }

    #[test]
    fn test_game_events() {
        let mut game = Game::new();

        game.add_event(GameEvent::GameStarted);
        assert_eq!(game.history.len(), 1);

        match &game.history[0] {
            GameEvent::GameStarted => (),
            _ => panic!("Expected GameStarted event"),
        }
    }
}
