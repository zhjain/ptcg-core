//! 效果演示

use ptcg_core::{
    Card, CardType, CardRarity, EnergyType, TrainerType,
    Effect, core::effects::EffectManager, PokemonAbilityEffect, PokemonAttackEffect, 
    TrainerEffect, SpecialEnergyEffect, AbilityType,
    EffectTrigger, TargetRequirement
};

fn main() {
    println!("PTCG 效果系统演示");
    println!("========================");
    
    // 创建效果管理器
    let mut effect_manager = EffectManager::new();
    
    // 创建宝可梦能力效果
    let lightning_rod = PokemonAbilityEffect::new(
        "避雷针".to_string(),
        "每当这只宝可梦受到电属性攻击时，攻击的宝可梦变为麻痹状态。".to_string(),
        AbilityType::Passive,
        vec![EffectTrigger::OnTakeDamage],
        vec![TargetRequirement::Pokemon],
    );
    
    let lightning_rod_id = effect_manager.register_effect(lightning_rod.clone());
    println!("创建了宝可梦能力：{} (ID: {})", lightning_rod.name(), lightning_rod_id);
    
    // 创建宝可梦攻击效果
    let thunder_effect = PokemonAttackEffect::new(
        "十万伏特".to_string(),
        "此攻击造成50点伤害。抛硬币，如果正面，防御的宝可梦变为麻痹状态。".to_string(),
        50,
        vec![TargetRequirement::Pokemon, TargetRequirement::InPlay],
    );
    
    let thunder_id = effect_manager.register_effect(thunder_effect.clone());
    println!("创建了宝可梦攻击效果：{} (ID: {})", thunder_effect.name(), thunder_id);
    
    // 创建训练家卡效果
    let professor_oak = TrainerEffect::new(
        "大木博士".to_string(),
        "丢弃你的手牌，然后抽7张卡。".to_string(),
        TrainerType::Supporter,
        true, // 一次性效果
        vec![],
    );
    
    let oak_id = effect_manager.register_effect(professor_oak.clone());
    println!("创建了训练家效果：{} (ID: {})", professor_oak.name(), oak_id);
    
    // 创建特殊能量效果
    let double_colorless = SpecialEnergyEffect::new(
        "双重无色能量".to_string(),
        "提供无色无色能量。如果附加此卡的宝可梦使用攻击，则该攻击对活跃宝可梦造成20点额外伤害。".to_string(),
        EnergyType::Colorless,
        None,
        None,
        vec![TargetRequirement::Pokemon],
    );
    
    let double_colorless_id = effect_manager.register_effect(double_colorless.clone());
    println!("创建了特殊能量效果：{} (ID: {})", double_colorless.name(), double_colorless_id);
    
    // 演示将效果附加到卡牌上
    let pokemon_card = Card::new(
        "皮卡丘".to_string(),
        CardType::Pokemon {
            species: "皮卡丘".to_string(),
            hp: 60,
            retreat_cost: 1,
            weakness: Some(EnergyType::Fighting),
            resistance: None,
            stage: ptcg_core::core::card::EvolutionStage::Basic,
            evolves_from: None,
        },
        "基础系列".to_string(),
        "001".to_string(),
        CardRarity::Common,
    );
    
    println!("\n将效果附加到{}...", pokemon_card.name);
    
    // 将能力附加到宝可梦卡上
    match effect_manager.attach_effect(pokemon_card.id, lightning_rod_id) {
        Ok(_) => println!("成功将避雷针能力附加到{}", pokemon_card.name),
        Err(e) => println!("附加能力失败：{:?}", e),
    }
    
    // 检查卡牌是否有效果
    if effect_manager.has_effects(pokemon_card.id) {
        println!("{}有附加效果", pokemon_card.name);
        let effects = effect_manager.get_card_effects(pokemon_card.id);
        println!("效果数量：{}", effects.len());
    }
    
    // 根据触发器获取效果
    let triggered_effects = effect_manager.get_effects_by_trigger(EffectTrigger::OnTakeDamage);
    println!("\n由OnTakeDamage触发的效果：");
    for (effect, card_id) in triggered_effects {
        println!("- {} 在卡牌ID: {}", effect.name(), card_id);
    }
    
    println!("\n演示完成！");
}