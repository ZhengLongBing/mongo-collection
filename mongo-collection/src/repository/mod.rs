//! Repository 模块，为 MongoDB 集合提供 CRUD 操作。

mod traits;
mod models;
mod utils;

// 重新导出 trait
pub use traits::CollectionRepository;

// 重新导出数据模型
pub use models::{
    PaginatedQuery,
    PaginatedData,
    ListData,
    SortOrder,
};

// 内部使用的工具函数不导出
#[allow(unused_imports)]
pub(crate) use utils::parse_object_id;
