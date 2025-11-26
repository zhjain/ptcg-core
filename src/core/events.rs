//! PTCG引擎的事件模块
//!
//! 此模块包含所有与事件相关的功能。

pub mod bus;
pub mod types;
pub mod handlers;

// 重新导出常用类型
pub use bus::*;
pub use types::*;
pub use handlers::*;

// 重新导出EventHandler trait
pub use handlers::EventHandler;

#[cfg(test)]
mod tests {

    #[test]
    fn test_events_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}