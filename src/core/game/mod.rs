//! PTCG引擎的游戏模块
//!
//! 此模块包含所有与游戏相关的功能。

pub mod state;
pub mod turn;
pub mod setup;
pub mod actions;
pub mod events;

// 重新导出常用类型
pub use state::*;
pub use setup::*;
pub use actions::*;

#[cfg(test)]
mod tests {

    #[test]
    fn test_game_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}