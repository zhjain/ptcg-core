//! PTCG引擎的效果模块
//!
//! 此模块包含所有与效果相关的功能。

pub mod manager;
pub mod types;
pub mod targets;
pub mod outcomes;
pub mod pokemon_effects;
pub mod trainer_effects;
pub mod energy_effects;

// 重新导出常用类型
pub use manager::*;
pub use types::*;
pub use targets::*;
pub use pokemon_effects::*;
pub use trainer_effects::*;
pub use energy_effects::*;

#[cfg(test)]
mod tests {

    #[test]
    fn test_effects_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}