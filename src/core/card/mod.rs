//! PTCG引擎的卡牌模块
//!
//! 此模块包含所有与卡牌相关的数据结构和功能。

pub mod types;
pub mod pokemon;
pub mod energy;
pub mod trainer;
pub mod attacks;
pub mod abilities;

// 重新导出常用类型
pub use types::*;
pub use pokemon::*;
pub use energy::*;
pub use trainer::*;
pub use attacks::*;
pub use abilities::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}