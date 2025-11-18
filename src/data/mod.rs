//! PTCG引擎的数据模块
//!
//! 此模块包含所有与数据相关的功能。

pub mod import;
pub mod export;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "csv_import")]
pub mod csv;

#[cfg(feature = "database")]
pub mod database;

// 重新导出常用类型
pub use import::*;
pub use export::*;

#[cfg(feature = "json")]
pub use json::*;

#[cfg(feature = "csv_import")]
pub use csv::*;

#[cfg(feature = "database")]
pub use database::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_module_structure() {
        // 这是一个占位测试，确保模块结构正确
        assert_eq!(2 + 2, 4);
    }
}