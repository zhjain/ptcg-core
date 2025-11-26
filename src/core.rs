//! PTCG引擎的核心模块
//!
//! 此模块包含所有核心功能。

pub mod card;
pub mod deck;
pub mod game;
pub mod player;
/// 卡牌效果系统模块
pub mod effects;
/// 事件系统模块
pub mod events;
/// 游戏规则引擎模块
pub mod rules;

// 重新导出常用类型
pub use card::*;
pub use deck::*;
pub use game::*;
pub use player::*;

#[cfg(test)]
mod tests {

    #[test]
    fn test_core_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}