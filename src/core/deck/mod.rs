//! PTCG引擎的牌组模块
//!
//! 此模块包含所有与牌组相关的功能。

pub mod manager;
pub mod validation;

// 重新导出常用类型
pub use manager::*;
pub use validation::*;

#[cfg(test)]
mod tests {

    #[test]
    fn test_deck_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}