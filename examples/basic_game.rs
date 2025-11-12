//! Basic example demonstrating the PTCG core engine usage
//!
//! This example shows how to:
//! - Create cards
//! - Build a deck
//! - Set up a game
//! - Use the rule engine

use ptcg_core::core::card::{
    AttackTargetType, CardId, EvolutionStage, StatusCondition, StatusEffect,
};
use ptcg_core::events::{ConsoleEventHandler, GameEvent};
use ptcg_core::rules::GameAction;
use ptcg_core::*;
use std::collections::HashMap;

fn main() {
    println!("🎮 PTCG Core Engine Example");
    println!("==========================");

    // Show library info
    let info = ptcg_core::info();
    println!("📦 Library version: {}", info.version);
    println!("🔧 Enabled features: {:?}", info.features);
    println!();

    // Create some example cards
    println!("🃏 Creating cards...");
    let mut card_database = HashMap::new();

    // Create Pikachu
    let mut pikachu = Card::new(
        "皮卡丘".to_string(),
        CardType::Pokemon {
            species: "皮卡丘".to_string(),
            hp: 60,
            retreat_cost: 1,
            weakness: Some(EnergyType::Fighting),
            resistance: None,
            stage: EvolutionStage::Basic,
            evolves_from: None,
        },
        "基础包".to_string(),
        "025".to_string(),
        CardRarity::Common,
    );

    // Add an attack to Pikachu
    pikachu.add_attack(Attack {
        name: "电击".to_string(),
        cost: vec![EnergyType::Lightning, EnergyType::Colorless],
        damage: 30,
        effect: Some("投掷硬币。如果正面，对方的宝可梦陷入麻痹状态。".to_string()),
        damage_mode: None,
        status_effects: vec![StatusEffect {
            condition: StatusCondition::Paralysis,
            probability: 50,
            target: "defending".to_string(),
        }],
        conditions: Vec::new(),
        target_type: AttackTargetType::Active,
    });

    let pikachu_id = pikachu.id;
    card_database.insert(pikachu_id, pikachu);

    // Create Charmander
    let mut charmander = Card::new(
        "小火龙".to_string(),
        CardType::Pokemon {
            species: "小火龙".to_string(),
            hp: 60,
            retreat_cost: 1,
            weakness: Some(EnergyType::Water),
            resistance: None,
            stage: EvolutionStage::Basic,
            evolves_from: None,
        },
        "基础包".to_string(),
        "004".to_string(),
        CardRarity::Common,
    );

    // Add an attack to Charmander
    charmander.add_attack(Attack {
        name: "火花".to_string(),
        cost: vec![EnergyType::Fire],
        damage: 20,
        effect: Some("投掷硬币。如果正面，对方的宝可梦陷入灼伤状态。".to_string()),
        damage_mode: None,
        status_effects: vec![StatusEffect {
            condition: StatusCondition::Burn,
            probability: 50,
            target: "defending".to_string(),
        }],
        conditions: Vec::new(),
        target_type: AttackTargetType::Active,
    });

    let charmander_id = charmander.id;
    card_database.insert(charmander_id, charmander);

    // Create Bulbasaur
    let mut bulbasaur = Card::new(
        "妙蛙种子".to_string(),
        CardType::Pokemon {
            species: "妙蛙种子".to_string(),
            hp: 60,
            retreat_cost: 1,
            weakness: Some(EnergyType::Fire),
            resistance: None,
            stage: EvolutionStage::Basic,
            evolves_from: None,
        },
        "基础包".to_string(),
        "001".to_string(),
        CardRarity::Common,
    );

    // Add an attack to Bulbasaur
    bulbasaur.add_attack(Attack {
        name: "藤鞭".to_string(),
        cost: vec![EnergyType::Grass],
        damage: 20,
        effect: None,
        damage_mode: None,
        status_effects: Vec::new(),
        conditions: Vec::new(),
        target_type: AttackTargetType::Active,
    });

    let bulbasaur_id = bulbasaur.id;
    card_database.insert(bulbasaur_id, bulbasaur);

    // Create Squirtle
    let mut squirtle = Card::new(
        "杰尼龟".to_string(),
        CardType::Pokemon {
            species: "杰尼龟".to_string(),
            hp: 60,
            retreat_cost: 1,
            weakness: Some(EnergyType::Grass),
            resistance: None,
            stage: EvolutionStage::Basic,
            evolves_from: None,
        },
        "基础包".to_string(),
        "007".to_string(),
        CardRarity::Common,
    );

    // Add an attack to Squirtle
    squirtle.add_attack(Attack {
        name: "水枪".to_string(),
        cost: vec![EnergyType::Water],
        damage: 20,
        effect: None,
        damage_mode: None,
        status_effects: Vec::new(),
        conditions: Vec::new(),
        target_type: AttackTargetType::Active,
    });

    let squirtle_id = squirtle.id;
    card_database.insert(squirtle_id, squirtle);

    // Create Lightning Energy
    let lightning_energy = Card::new(
        "雷电能量".to_string(),
        CardType::Energy {
            energy_type: EnergyType::Lightning,
            is_basic: true,
        },
        "基础包".to_string(),
        "100".to_string(),
        CardRarity::Common,
    );

    let energy_id = lightning_energy.id;
    card_database.insert(energy_id, lightning_energy);

    println!("✅ Created {} cards", card_database.len());
    println!();

    // Create a deck
    println!("📚 Building deck...");
    let mut deck = Deck::new("示例牌组".to_string(), "Standard".to_string());

    // Add cards to deck
    deck.add_card(pikachu_id, 10); // 10x Pikachu
    deck.add_card(charmander_id, 10); // 10x Charmander
    deck.add_card(bulbasaur_id, 10); // 10x Bulbasaur
    deck.add_card(squirtle_id, 10); // 10x Squirtle
    deck.add_card(energy_id, 20); // 20x Lightning Energy (to make 60 cards)

    println!("📊 Deck statistics:");
    let stats = deck.get_statistics(&card_database);
    println!("   - Total cards: {}", stats.total_cards);
    println!("   - Unique cards: {}", stats.unique_cards);
    println!("   - Pokemon: {}", stats.pokemon_count);
    println!("   - Energy: {}", stats.energy_count);

    // Validate deck
    match deck.validate(&card_database) {
        Ok(()) => println!("✅ Deck is valid!"),
        Err(errors) => {
            println!("❌ Deck validation errors:");
            for error in errors {
                println!("   - {:?}", error);
            }
        }
    }
    println!();

    // Create players
    println!("👥 Creating players...");
    let player1 = Player::new("玩家1".to_string());
    let player2 = Player::new("玩家2".to_string());

    let player1_id = player1.id;
    let player2_id = player2.id;

    println!("   - {}: {}", player1.name, player1_id);
    println!("   - {}: {}", player2.name, player2_id);
    println!();

    // Create game
    println!("🎯 Setting up game...");
    let mut game = Game::new();

    // Add players to game
    if let Err(e) = game.add_player(player1) {
        println!("❌ Failed to add player 1: {}", e);
        return;
    }
    if let Err(e) = game.add_player(player2) {
        println!("❌ Failed to add player 2: {}", e);
        return;
    }

    // Set decks for both players (same deck for simplicity)
    if let Err(e) = game.set_player_deck(player1_id, deck.clone()) {
        println!("❌ Failed to set deck for player 1: {}", e);
        return;
    }
    if let Err(e) = game.set_player_deck(player2_id, deck) {
        println!("❌ Failed to set deck for player 2: {}", e);
        return;
    }

    // Add cards to game database
    for (_card_id, card) in card_database {
        game.add_card_to_database(card);
    }

    println!("✅ Game setup complete!");
    println!("   - Game ID: {}", game.id);
    println!("   - Players: {}", game.get_players().len());
    println!("   - Cards in database: {}", game.card_database.len());

    // 显示初始玩家顺序
    println!("   - Initial player order:");
    for (index, player_id) in game.turn_order.iter().enumerate() {
        if let Some(player) = game.get_player(*player_id) {
            println!("     {}. {}", index + 1, player.name);
        }
    }

    // Demonstrate rule engine
    println!("⚖️  Testing rule engine...");
    let rule_engine = StandardRules::create_engine();
    println!("   - Active rules: {:?}", rule_engine.get_rule_names());

    // Test a valid action (current player's turn)
    let action = GameAction::DrawCard {
        player_id: player1_id,
    };
    let violations = rule_engine.validate_action(&game, &action);

    if violations.is_empty() {
        println!("✅ Action 'DrawCard' is valid for current player");
    } else {
        println!("❌ Action violations: {:?}", violations);
    }

    // Test an invalid action (wrong player's turn)
    let action = GameAction::DrawCard {
        player_id: player2_id,
    };
    let violations = rule_engine.validate_action(&game, &action);

    if violations.is_empty() {
        println!("✅ Action 'DrawCard' is valid for player 2");
    } else {
        println!("❌ Action violations for player 2: {:?}", violations);
    }
    println!();

    // 新增：启动游戏并显示先后手
    println!("🎮 Starting game setup...");
    let mut game = game; // 转换为可变引用

    // 阶段1: 开始设置过程
    match game.start_setup() {
        Ok(()) => println!("✅ Game setup started!"),
        Err(e) => {
            println!("❌ Failed to start game setup: {}", e);
            return;
        }
    }

    // 阶段2: 确定先后手顺序
    match game.determine_turn_order() {
        Ok(()) => {
            println!("✅ Turn order determined!");

            // 显示当前玩家顺序
            println!("   - Player order:");
            for (index, player_id) in game.turn_order.iter().enumerate() {
                if let Some(player) = game.get_player(*player_id) {
                    println!("     {}. {}", index + 1, player.name);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to determine turn order: {}", e);
            return;
        }
    }

    // 阶段3: 发放初始手牌
    match game.deal_opening_hands() {
        Ok(()) => {
            println!("✅ Opening hands dealt!");

            // 显示玩家手牌数量
            for player_id in &game.turn_order {
                if let Some(player) = game.get_player(*player_id) {
                    println!("   - {}: {} cards in hand", player.name, player.hand.len());
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to deal opening hands: {}", e);
            return;
        }
    }

    // 阶段4: 检查基础宝可梦
    match game.check_for_basic_pokemon() {
        Ok(players_without_basic) => {
            if players_without_basic.is_empty() {
                println!("✅ All players have basic Pokemon!");
            } else {
                println!("⚠️  Some players don't have basic Pokemon:");
                for &player_id in &players_without_basic {
                    if let Some(player) = game.get_player(player_id) {
                        println!("   - {}", player.name);
                    }
                }

                // 按照官方规则书实现穆勒规则
                println!("🔄 Handling mulligan according to official rules...");

                // 阶段5a: 玩家宣告没有基础宝可梦
                match game.declare_no_basic_pokemon() {
                    Ok((players_without_basic, all_without_basic)) => {
                        if all_without_basic {
                            // 双方都没有基础宝可梦
                            println!("   Both players declared no basic Pokemon");
                            println!("   Checking each other's hands...");

                            // 对于双方都没有基础宝可梦的情况，我们简化处理：
                            // 直接为所有玩家执行重抽
                            for &player_id in &players_without_basic {
                                match game.perform_mulligan(player_id) {
                                    Ok(()) => {
                                        if let Some(player) = game.get_player(player_id) {
                                            println!(
                                                "   - {} performed mulligan (now has {} cards)",
                                                player.name,
                                                player.hand.len()
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        println!(
                                            "   ❌ Failed to perform mulligan for player: {}",
                                            e
                                        );
                                    }
                                }
                            }
                        } else {
                            // 只有一方没有基础宝可梦
                            println!("   Only one player declared no basic Pokemon");
                            println!("   That player waits while the other continues setup...");

                            // 阶段5b: 记录需要等待重抽的玩家
                            for &player_id in &players_without_basic {
                                match game.mark_player_for_mulligan(player_id) {
                                    Ok(()) => {
                                        if let Some(player) = game.get_player(player_id) {
                                            println!(
                                                "   - {} marked for mulligan after opponent completes setup",
                                                player.name
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        println!("   ❌ Failed to mark player for mulligan: {}", e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("   ❌ Failed to declare no basic Pokemon: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to check for basic Pokemon: {}", e);
            return;
        }
    }

    // 阶段5: 玩家选择活跃宝可梦
    println!("🎯 Selecting active Pokemon...");
    // 创建turn_order的副本以避免借用冲突
    let player_order = game.turn_order.clone();
    for player_id in player_order {
        // 使用单独的作用域来避免借用冲突
        let player_name = {
            if let Some(player) = game.get_player(player_id) {
                Some(player.name.clone())
            } else {
                None
            }
        };

        if let Some(name) = player_name {
            // 再次获取玩家引用以检查手牌
            let basic_pokemon = {
                if let Some(player) = game.get_player(player_id) {
                    player.find_basic_pokemon_in_hand(&game.card_database)
                } else {
                    Vec::new()
                }
            };

            if !basic_pokemon.is_empty() {
                let first_pokemon = basic_pokemon[0];
                // 使用单独的作用域来避免借用冲突
                let select_result = { game.select_active_pokemon(player_id, first_pokemon) };
                match select_result {
                    Ok(()) => {
                        if let Some(pokemon_card) = game.get_card(first_pokemon) {
                            println!(
                                "   - {} selected {} as active Pokemon",
                                name, pokemon_card.name
                            );
                        }
                    }
                    Err(e) => {
                        println!("   ❌ {} failed to select active Pokemon: {}", name, e);
                    }
                }
            }
        }
    }

    // 阶段6: 玩家设置备战区
    println!("📋 Setting up bench...");
    // 创建turn_order的副本以避免借用冲突
    let player_order = game.turn_order.clone();
    for player_id in player_order {
        // 使用单独的作用域来避免借用冲突
        let player_name = {
            if let Some(player) = game.get_player(player_id) {
                Some(player.name.clone())
            } else {
                None
            }
        };

        if let Some(name) = player_name {
            // 再次获取玩家引用以检查手牌
            let basic_pokemon = {
                if let Some(player) = game.get_player(player_id) {
                    player.find_basic_pokemon_in_hand(&game.card_database)
                } else {
                    Vec::new()
                }
            };

            // 选择最多2只其他基础宝可梦放到备战区（保留1只为活跃宝可梦）
            let bench_pokemon: Vec<CardId> =
                basic_pokemon.iter().skip(1).take(2).cloned().collect();

            if !bench_pokemon.is_empty() {
                // 使用单独的作用域来避免借用冲突
                let setup_result = { game.setup_bench(player_id, bench_pokemon.clone()) };
                match setup_result {
                    Ok(()) => {
                        println!(
                            "   - {} placed {} Pokemon on bench",
                            name,
                            bench_pokemon.len()
                        );
                    }
                    Err(e) => {
                        println!("   ❌ {} failed to setup bench: {}", name, e);
                    }
                }
            }
        }
    }

    // 阶段7: 放置奖赏卡
    match game.place_prize_cards() {
        Ok(()) => {
            println!("🏆 Prize cards placed!");
            // 创建turn_order的副本以避免借用冲突
            let player_order = game.turn_order.clone();
            for player_id in player_order {
                if let Some(player) = game.get_player(player_id) {
                    println!("   - {}: {} prize cards", player.name, player.prize_cards);
                }
            }

            // 执行等待中的重抽操作（如果有的话）
            match game.perform_pending_mulligans() {
                Ok(()) => {
                    if !game.players_waiting_for_mulligan.is_empty() {
                        println!(
                            "🔄 Performed pending mulligans for players who declared no basic Pokemon"
                        );
                    }
                }
                Err(e) => {
                    println!("❌ Failed to perform pending mulligans: {}", e);
                    return;
                }
            }

            // 检查重抽后的结果，如果仍然没有基础宝可梦，需要继续重抽
            loop {
                let players_without_basic = match game.check_for_basic_pokemon() {
                    Ok(players) => players,
                    Err(e) => {
                        println!("❌ Failed to check for basic Pokemon: {}", e);
                        return;
                    }
                };

                if players_without_basic.is_empty() {
                    break; // 所有玩家都有基础宝可梦了
                }

                println!(
                    "⚠️  Some players still don't have basic Pokemon after mulligan, performing additional mulligans..."
                );

                // 为仍然没有基础宝可梦的玩家执行额外的重抽
                for &player_id in &players_without_basic {
                    match game.perform_mulligan(player_id) {
                        Ok(()) => {
                            if let Some(player) = game.get_player(player_id) {
                                println!(
                                    "   - {} performed additional mulligan (now has {} cards)",
                                    player.name,
                                    player.hand.len()
                                );
                            }
                        }
                        Err(e) => {
                            println!(
                                "   ❌ Failed to perform additional mulligan for player: {}",
                                e
                            );
                            return;
                        }
                    }
                }
            }

            // 阶段7b: 奖赏卡补偿
            // 如果对手执行了步骤5.d.（重抽），则可以进行卡牌张数的宣告
            let compensation_limit = match game.get_mulligan_compensation_limit(game.turn_order[0])
            {
                Ok(limit) => limit,
                Err(e) => {
                    println!("❌ Failed to get mulligan compensation limit: {}", e);
                    return;
                }
            };

            if compensation_limit > 0 {
                println!(
                    "🎁 Mulligan compensation available: up to {} cards",
                    compensation_limit
                );

                // 简化处理：玩家抽取补偿卡牌
                // TODO ....
            } else {
                println!("🎁 No mulligan compensation available");
            }
        }
        Err(e) => {
            println!("❌ Failed to place prize cards: {}", e);
            return;
        }
    }

    // 阶段8: 完成设置，开始游戏
    match game.complete_setup() {
        Ok(()) => {
            println!("🎉 Game setup completed! Game started!");

            // 显示当前回合信息
            if let Ok(current_player) = game.get_current_player() {
                println!(
                    "   - Current turn: {} (Turn {})",
                    current_player.name, game.turn_number
                );
                println!("   - Current phase: {:?}", game.phase);
            }
        }
        Err(e) => {
            println!("❌ Failed to complete setup: {}", e);
            return;
        }
    }

    // Demonstrate event system
    println!("📢 Testing event system...");
    let mut event_bus = EventBus::new();

    // Register a console event handler
    let console_handler = ConsoleEventHandler::new(false);
    event_bus.register_handler(console_handler);

    // Emit some events
    let event = GameEvent::GameStarted {
        timestamp: ptcg_core::events::current_timestamp(),
        players: vec![player1_id, player2_id],
    };
    event_bus.emit(&event);

    let event = GameEvent::TurnStarted {
        timestamp: ptcg_core::events::current_timestamp(),
        player_id: player1_id,
        turn_number: 1,
    };
    event_bus.emit(&event);

    println!("   - Events in history: {}", event_bus.get_history().len());
    println!();

    // Show some game information
    println!("ℹ️  Game Information:");
    println!("   - Current state: {:?}", game.state);
    println!("   - Current phase: {:?}", game.phase);
    println!("   - Turn number: {}", game.turn_number);

    if let Ok(current_player) = game.get_current_player() {
        println!("   - Current player: {}", current_player.name);
        println!("   - Hand size: {}", current_player.hand.len());
        println!("   - Deck size: {}", current_player.deck.len());
        println!("   - Prize cards: {}", current_player.prize_cards);
    }

    println!();
    println!("🎉 Example completed successfully!");
}
