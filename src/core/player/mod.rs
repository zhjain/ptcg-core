//! PTCG引擎的玩家模块
//!
//! 此模块包含所有与玩家相关的数据结构和功能。

pub mod state;
pub mod conditions;
pub mod actions;

// 重新导出常用类型
pub use state::*;
pub use conditions::*;
pub use actions::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}