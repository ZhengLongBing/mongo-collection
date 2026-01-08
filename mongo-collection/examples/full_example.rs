/// 完整示例：展示如何使用 Collection 和 CollectionRepository derive 宏
///
/// 注意：这是一个代码示例，需要实际的 MongoDB 连接才能运行
///
/// 运行前需要：
/// 1. 启动 MongoDB 服务
/// 2. 设置环境变量：export MONGODB_URI="mongodb://localhost:27017"
use mongo_collection::{Collection, CollectionRepository, PaginatedQuery};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Collection, CollectionRepository, Serialize, Deserialize, Debug, Clone)]
#[collection(name = "users")]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    email: String,
    age: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 mongo-collection 完整示例\n");

    // 连接到 MongoDB
    let uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    println!("正在连接到 MongoDB: {}", uri);
    let client = mongodb::Client::with_uri_str(&uri).await?;
    let db = client.database("test_db");

    println!("✅ 连接成功！\n");

    // 1. 创建用户
    println!("📝 1. 创建用户");
    let new_user = User {
        id: None,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        age: 25,
    };

    let created_user = User::create(&db, &new_user).await?;
    println!("   创建成功: {:?}\n", created_user);

    // 2. 批量创建
    println!("📝 2. 批量创建用户");
    let users = vec![
        User {
            id: None,
            name: "李四".to_string(),
            email: "lisi@example.com".to_string(),
            age: 30,
        },
        User {
            id: None,
            name: "王五".to_string(),
            email: "wangwu@example.com".to_string(),
            age: 28,
        },
    ];
    User::create_many(&db, users).await?;
    println!("   批量创建成功\n");

    // 3. 查找所有用户
    println!("🔍 3. 查找所有用户");
    let all_users = User::find_all(&db).await?;
    println!("   找到 {} 个用户", all_users.len());
    for user in &all_users {
        println!("   - {} ({}岁) - {}", user.name, user.age, user.email);
    }
    println!();

    // 4. 根据条件查找
    println!("🔍 4. 查找年龄大于 26 的用户");
    let filter = doc! { "age": { "$gt": 26 } };
    let filtered_users = User::find_many(&db, filter, None).await?;
    println!("   找到 {} 个用户", filtered_users.len());
    for user in &filtered_users {
        println!("   - {} ({}岁)", user.name, user.age);
    }
    println!();

    // 5. 统计
    println!("📊 5. 统计用户数量");
    let count = User::count(&db, doc! {}).await?;
    println!("   总共有 {} 个用户\n", count);

    // 6. 分页查询
    println!("📄 6. 分页查询（第1页，每页2条）");
    let query = PaginatedQuery {
        page: 1,
        page_size: 2,
        sort_by: Some("age".to_string()),
        ..Default::default()
    };
    let paginated = User::find_paginated(&db, doc! {}, &query).await?;
    println!("   页码: {}/{}", paginated.page, paginated.total_pages);
    println!("   总数: {}", paginated.total_count);
    println!("   当前页数据:");
    for user in &paginated.items {
        println!("   - {} ({}岁)", user.name, user.age);
    }
    println!();

    // 7. 更新
    if let Some(first_user) = all_users.first() {
        if let Some(ref id) = first_user.id {
            println!("✏️  7. 更新用户");
            let update = doc! { "$set": { "age": 26 } };
            let updated = User::update_by_id(&db, id, update).await?;
            println!("   更新成功: {}\n", updated);
        }
    }

    // 8. 检查存在
    println!("🔍 8. 检查用户是否存在");
    let exists = User::exists(&db, doc! { "email": "zhangsan@example.com" }).await?;
    println!("   用户存在: {}\n", exists);

    // 9. 删除
    println!("🗑️  9. 删除年龄小于 27 的用户");
    let delete_filter = doc! { "age": { "$lt": 27 } };
    let deleted_count = User::delete_many(&db, delete_filter).await?;
    println!("   删除了 {} 个用户\n", deleted_count);

    // 10. 最终统计
    println!("📊 10. 最终统计");
    let final_count = User::count(&db, doc! {}).await?;
    println!("   剩余用户数: {}\n", final_count);

    println!("✅ 示例完成！");

    Ok(())
}
