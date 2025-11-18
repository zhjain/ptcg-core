//! PTCG引擎的规则模块
//!
//! 此模块包含所有与规则相关的功能。

pub mod engine;
pub mod standard;
pub mod validation;
pub mod effects;

// 重新导出常用类型
pub use engine::*;
pub use standard::*;
pub use validation::*;
pub use effects::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}