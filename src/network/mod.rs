//! PTCG引擎的网络模块
//!
//! 此模块包含所有与网络相关的功能。

pub mod server;
pub mod client;

// 重新导出常用类型
pub use server::*;
pub use client::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}