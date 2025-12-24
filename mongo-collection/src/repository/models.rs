use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::collections::HashMap;

#[cfg(feature = "openapi")]
use utoipa::{IntoParams, ToSchema};

/// 分页查询参数
#[derive(Debug, Clone, SmartDefault, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema, IntoParams))]
#[serde(default)]
pub struct PaginatedQuery {
    // 基础分页
    /// 页码（从 1 开始）
    #[default = 1]
    pub page: u64,
    /// 每页数量
    #[default = 10]
    pub page_size: u64,
    // 排序相关
    /// 排序字段（如 "created_at"、"name"）
    pub sort_by: Option<String>,
    /// 排序方向
    #[default(_code = "SortOrder::Desc")]
    pub sort_order: SortOrder,
    // 搜索/筛选
    /// 全局搜索关键词
    pub search: Option<String>,
    /// 字段级筛选
    pub filters: Option<HashMap<String, String>>,
}

/// 排序方向
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum SortOrder {
    /// 降序
    #[default]
    Desc,
    /// 升序
    Asc,
}

impl PaginatedQuery {
    /// 计算跳过的记录数
    pub fn skip(&self) -> u64 {
        (self.page - 1) * self.page_size
    }

    /// 计算查询限制数
    pub fn limit(&self) -> i64 {
        self.page_size as i64
    }
}

/// 分页数据响应
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[cfg_attr(feature = "openapi", schema(bound = "T: ToSchema"))]
pub struct PaginatedData<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 总记录数
    pub total_count: u64,
    /// 当前页码
    pub page: u64,
    /// 每页数量
    pub page_size: u64,
    /// 总页数
    pub total_pages: u64,
}

impl<T> PaginatedData<T> {
    /// 转换数据类型
    ///
    /// 使用泛型闭包对每个元素进行转换
    pub fn map<P, F>(self, f: F) -> PaginatedData<P>
    where
        F: FnMut(T) -> P,
    {
        PaginatedData {
            items: self.items.into_iter().map(f).collect(),
            total_count: self.total_count,
            page: self.page,
            page_size: self.page_size,
            total_pages: self.total_pages,
        }
    }

    /// 转换数据类型（可能失败）
    ///
    /// 使用泛型闭包对每个元素进行转换，转换可能失败
    pub fn try_map<P, E, F>(self, f: F) -> Result<PaginatedData<P>, E>
    where
        F: FnMut(T) -> Result<P, E>,
    {
        Ok(PaginatedData {
            items: self
                .items
                .into_iter()
                .map(f)
                .collect::<Result<Vec<_>, _>>()?,
            total_count: self.total_count,
            page: self.page,
            page_size: self.page_size,
            total_pages: self.total_pages,
        })
    }
}

/// 列表数据响应
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[cfg_attr(feature = "openapi", schema(bound = "T: ToSchema"))]
pub struct ListData<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 总记录数
    pub total_count: u64,
}

impl<T> ListData<T> {
    /// 创建新的列表数据
    pub fn new(items: Vec<T>) -> Self {
        Self {
            total_count: items.len() as u64,
            items,
        }
    }

    /// 转换数据类型
    ///
    /// 使用泛型闭包对每个元素进行转换
    pub fn map<P, F>(self, f: F) -> ListData<P>
    where
        F: FnMut(T) -> P,
    {
        ListData {
            items: self.items.into_iter().map(f).collect(),
            total_count: self.total_count,
        }
    }

    /// 转换数据类型（可能失败）
    ///
    /// 使用泛型闭包对每个元素进行转换，转换可能失败
    pub fn try_map<P, E, F>(self, f: F) -> Result<ListData<P>, E>
    where
        F: FnMut(T) -> Result<P, E>,
    {
        Ok(ListData {
            items: self
                .items
                .into_iter()
                .map(f)
                .collect::<Result<Vec<_>, _>>()?,
            total_count: self.total_count,
        })
    }
}
