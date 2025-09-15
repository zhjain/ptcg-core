//! Basic example demonstrating the PTCG core engine usage
//! 
//! This example shows how to:
//! - Create cards
//! - Build a deck
//! - Set up a game
//! - Use the rule engine

use ptcg_core::*;
use ptcg_core::core::card::EvolutionStage;
use ptcg_core::rules::GameAction;
use ptcg_core::events::{GameEvent, ConsoleEventHandler};
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
    });
    
    let pikachu_id = pikachu.id;
    card_database.insert(pikachu_id, pikachu);
    
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
    deck.add_card(pikachu_id, 4);       // 4x Pikachu
    deck.add_card(energy_id, 56);       // 56x Lightning Energy (to make 60 cards)
    
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
    for (card_id, card) in card_database {
        game.add_card_to_database(card);
    }
    
    println!("✅ Game setup complete!");
    println!("   - Game ID: {}", game.id);
    println!("   - Players: {}", game.get_players().len());
    println!("   - Cards in database: {}", game.card_database.len());
    println!();

    // Demonstrate rule engine
    println!("⚖️  Testing rule engine...");
    let rule_engine = StandardRules::create_engine();
    println!("   - Active rules: {:?}", rule_engine.get_rule_names());
    
    // Test a valid action (current player's turn)
    let action = GameAction::DrawCard { player_id: player1_id };
    let violations = rule_engine.validate_action(&game, &action);
    
    if violations.is_empty() {
        println!("✅ Action 'DrawCard' is valid for current player");
    } else {
        println!("❌ Action violations: {:?}", violations);
    }
    
    // Test an invalid action (wrong player's turn)
    let action = GameAction::DrawCard { player_id: player2_id };
    let violations = rule_engine.validate_action(&game, &action);
    
    if violations.is_empty() {
        println!("✅ Action 'DrawCard' is valid for player 2");
    } else {
        println!("❌ Action violations for player 2: {:?}", violations);
    }
    println!();

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