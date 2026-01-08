use mongo_collection::{Collection, CollectionRepository};
use serde::{Deserialize, Serialize};

/// 示例：使用 CollectionRepository 派生宏
///
/// 这个示例展示如何使用 derive 宏自动实现 CollectionRepository trait
#[derive(Collection, CollectionRepository, Serialize, Deserialize, Debug, Clone)]
#[collection(name = "users")]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    email: String,
}

#[derive(Collection, CollectionRepository, Serialize, Deserialize, Debug, Clone)]
struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    title: String,
    content: String,
}

fn main() {
    println!("CollectionRepository 派生宏示例");
    println!();

    // 验证 Collection trait
    println!("用户集合名称: {}", User::name());
    println!("文章集合名称: {}", Post::name());
    println!();

    println!("✅ CollectionRepository trait 已自动实现！");
    println!();
    println!("现在可以使用以下方法：");
    println!("  - User::create(&db, &user).await              // 创建单个文档");
    println!("  - User::find_by_id(&db, \"123\").await          // 根据 ID 查找");
    println!(
        "  - User::find_one(&db, doc! {{ \"email\": \"test@example.com\" }}).await  // 查找单个"
    );
    println!("  - User::find_many(&db, doc! {{}}, None).await  // 查找多个");
    println!("  - User::find_all(&db).await                   // 查找所有");
    println!("  - User::find_paginated(&db, doc! {{}}, &query).await  // 分页查询");
    println!("  - User::count(&db, doc! {{}}).await            // 统计数量");
    println!(
        "  - User::exists(&db, doc! {{ \"email\": \"test@example.com\" }}).await  // 检查存在"
    );
    println!(
        "  - User::update_by_id(&db, \"123\", doc! {{ \"$set\": {{ \"name\": \"新名字\" }} }}).await  // 根据 ID 更新"
    );
    println!("  - User::update_one(&db, filter, update).await  // 更新单个");
    println!("  - User::update_many(&db, filter, update).await // 批量更新");
    println!("  - User::delete_by_id(&db, \"123\").await        // 根据 ID 删除");
    println!("  - User::delete_one(&db, filter).await         // 删除单个");
    println!("  - User::delete_many(&db, filter).await        // 批量删除");
}
