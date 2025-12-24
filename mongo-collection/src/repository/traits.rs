use async_trait::async_trait;
use crate::Collection;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use super::utils::parse_object_id;
use super::models::{PaginatedQuery, PaginatedData, SortOrder};

/// 通用集合仓储 Trait
///
/// 提供基础的 CRUD 操作接口
/// 注意：所有错误使用 MongoDB 原生错误类型 `mongodb::error::Error`
#[async_trait]
pub trait CollectionRepository: Collection + Clone + Send + Sync + Unpin
where
    Self: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    // ========== 创建操作 ==========

    /// 创建单个文档
    async fn create(
        db: &mongodb::Database,
        document: &Self,
    ) -> Result<Self, mongodb::error::Error> {
        let collection = Self::collection(db);
        collection.insert_one(document).await?;
        Ok(document.clone())
    }

    /// 批量创建文档
    async fn create_many(
        db: &mongodb::Database,
        documents: Vec<Self>,
    ) -> Result<Vec<Self>, mongodb::error::Error> {
        let collection = Self::collection(db);
        collection.insert_many(&documents).await?;
        Ok(documents)
    }

    // ========== 查询操作 ==========

    /// 根据 ID 查找文档
    async fn find_by_id(
        db: &mongodb::Database,
        id: &str,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        let collection = Self::collection(db);
        let oid = parse_object_id(id)?;

        collection.find_one(doc! { "_id": oid }).await
    }

    /// 根据条件查找单个文档
    async fn find_one(
        db: &mongodb::Database,
        filter: Document,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        let collection = Self::collection(db);
        collection.find_one(filter).await
    }

    /// 根据条件查找多个文档
    async fn find_many(
        db: &mongodb::Database,
        filter: Document,
        options: Option<FindOptions>,
    ) -> Result<Vec<Self>, mongodb::error::Error> {
        let collection = Self::collection(db);
        let mut cursor = match options {
            None => collection.find(filter).await?,
            Some(options) => collection.find(filter).with_options(options).await?
        };

        let mut results = Vec::new();
        use futures::TryStreamExt;
        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }

        Ok(results)
    }

    /// 查找所有文档
    async fn find_all(
        db: &mongodb::Database,
    ) -> Result<Vec<Self>, mongodb::error::Error> {
        Self::find_many(db, doc! {}, None).await
    }

    /// 分页查询
    async fn find_paginated(
        db: &mongodb::Database,
        filter: Document,
        query: &PaginatedQuery,
    ) -> Result<PaginatedData<Self>, mongodb::error::Error> {
        let collection = Self::collection(db);

        // 获取总数
        let total_count = collection.count_documents(filter.clone()).await?;

        // 构建排序选项
        let sort_doc = if let Some(ref sort_by) = query.sort_by {
            let sort_value = match query.sort_order {
                SortOrder::Asc => 1,
                SortOrder::Desc => -1,
            };
            doc! { sort_by: sort_value }
        } else {
            doc! { "_id": -1 } // 默认按 ID 降序
        };

        // 构建查询选项
        let find_options = FindOptions::builder()
            .skip(query.skip())
            .limit(query.limit())
            .sort(sort_doc)
            .build();

        // 执行查询
        let items = Self::find_many(db, filter, Some(find_options)).await?;

        // 计算总页数
        let total_pages = (total_count as f64 / query.page_size as f64).ceil() as u64;

        Ok(PaginatedData {
            items,
            total_count,
            page: query.page,
            page_size: query.page_size,
            total_pages,
        })
    }

    /// 统计文档数量
    async fn count(
        db: &mongodb::Database,
        filter: Document,
    ) -> Result<u64, mongodb::error::Error> {
        let collection = Self::collection(db);
        collection.count_documents(filter).await
    }

    /// 检查文档是否存在
    async fn exists(
        db: &mongodb::Database,
        filter: Document,
    ) -> Result<bool, mongodb::error::Error> {
        Ok(Self::find_one(db, filter).await?.is_some())
    }

    // ========== 更新操作 ==========

    /// 根据 ID 更新文档
    async fn update_by_id(
        db: &mongodb::Database,
        id: &str,
        update: Document,
    ) -> Result<bool, mongodb::error::Error> {
        let collection = Self::collection(db);
        let oid = parse_object_id(id)?;

        let result = collection
            .update_one(doc! { "_id": oid }, update)
            .await?;

        Ok(result.modified_count > 0)
    }

    /// 根据条件更新单个文档
    async fn update_one(
        db: &mongodb::Database,
        filter: Document,
        update: Document,
    ) -> Result<bool, mongodb::error::Error> {
        let collection = Self::collection(db);
        let result = collection.update_one(filter, update).await?;
        Ok(result.modified_count > 0)
    }

    /// 根据条件更新多个文档
    async fn update_many(
        db: &mongodb::Database,
        filter: Document,
        update: Document,
    ) -> Result<u64, mongodb::error::Error> {
        let collection = Self::collection(db);
        let result = collection.update_many(filter, update).await?;
        Ok(result.modified_count)
    }

    /// 查找并更新文档（返回更新后的文档）
    async fn find_one_and_update(
        db: &mongodb::Database,
        filter: Document,
        update: Document,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        let collection = Self::collection(db);
        collection.find_one_and_update(filter, update).await
    }

    // ========== 删除操作 ==========

    /// 根据 ID 删除文档
    async fn delete_by_id(
        db: &mongodb::Database,
        id: &str,
    ) -> Result<bool, mongodb::error::Error> {
        let collection = Self::collection(db);
        let oid = parse_object_id(id)?;

        let result = collection.delete_one(doc! { "_id": oid }).await?;
        Ok(result.deleted_count > 0)
    }

    /// 根据条件删除单个文档
    async fn delete_one(
        db: &mongodb::Database,
        filter: Document,
    ) -> Result<bool, mongodb::error::Error> {
        let collection = Self::collection(db);
        let result = collection.delete_one(filter).await?;
        Ok(result.deleted_count > 0)
    }

    /// 根据条件删除多个文档
    async fn delete_many(
        db: &mongodb::Database,
        filter: Document,
    ) -> Result<u64, mongodb::error::Error> {
        let collection = Self::collection(db);
        let result = collection.delete_many(filter).await?;
        Ok(result.deleted_count)
    }

    /// 查找并删除文档（返回被删除的文档）
    async fn find_one_and_delete(
        db: &mongodb::Database,
        filter: Document,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        let collection = Self::collection(db);
        collection.find_one_and_delete(filter).await
    }
}
