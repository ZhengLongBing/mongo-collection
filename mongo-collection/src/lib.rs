pub use mongo_collection_macro::Collection;

mod collection;
pub use collection::Collection;

mod list;
mod paginated;
pub mod repository;
mod utils;

pub use mongo_collection_macro::CollectionRepository;

use serde::{Deserialize, Serialize};

#[cfg(feature = "openapi")]
use utoipa::{IntoParams, ToSchema};

/// 排序方向
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// 降序
    #[default]
    Desc,
    /// 升序
    Asc,
}

pub use crate::list::{ListData, ListQuery};
pub use crate::paginated::{PaginatedData, PaginatedQuery};
pub use crate::repository::CollectionRepository;
