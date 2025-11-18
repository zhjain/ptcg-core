//! PTCG引擎的核心模块
//!
//! 此模块包含所有核心功能。

pub mod card;
pub mod deck;
pub mod game;
pub mod player;

// 重新导出常用类型
pub use card::*;
pub use deck::*;
pub use game::*;
pub use player::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}