# MongoDB Collection Macros

Procedural macros for MongoDB collection trait derivation with built-in CRUD operations.

## Overview

This crate provides two powerful derive macros:

1. **`Collection`** - Automatically implements the Collection trait for MongoDB collection name mapping
2. **`CollectionRepository`** - Provides full CRUD operations with pagination support

## Collection Derive Macro

The `Collection` derive macro automatically implements the `Collection` trait, which identifies MongoDB collection names for your Rust types.

### Features

- **Automatic trait implementation** - Derive the `Collection` trait with a single attribute
- **Custom collection names** - Override default naming with `#[collection(name = "...")]`
- **Smart naming conventions** - Default plural snake_case naming (User → users, Category → categories)
- **Compile-time type safety** - Catch errors at compile time, not runtime
- **Thread-safe** - Built-in `Send + Sync` support for concurrent applications
- **Zero runtime overhead** - All name resolution happens at compile time

### Usage

#### 1. Using Default Collection Name (Auto-Pluralized Snake Case)

```rust
use serde::{Serialize, Deserialize};
use mongo_collection::Collection;

#[derive(Collection, Serialize, Deserialize, Debug, Clone)]
struct User {
    id: String,
    name: String,
}

// Collection name will automatically be "users" (pluralized)
assert_eq!(User::name(), "users");
```

#### 2. Specifying a Custom Collection Name

```rust
use serde::{Serialize, Deserialize};
use mongo_collection::Collection;

#[derive(Collection, Serialize, Deserialize, Debug, Clone)]
#[collection(name = "user_accounts")]
struct User {
    id: String,
    name: String,
}

// Overrides default naming with custom collection name
assert_eq!(User::name(), "user_accounts");
```

#### 3. Complex Struct Example

```rust
use serde::{Serialize, Deserialize};
use mongo_collection::Collection;

#[derive(Collection, Serialize, Deserialize, Debug, Clone)]
struct UserProfile {
    user_id: String,
    bio: String,
    avatar_url: Option<String>,
}

// Without specifying name, defaults to "user_profiles" (pluralized snake_case)
assert_eq!(UserProfile::name(), "user_profiles");
```

### Naming Conversion Rules

When `#[collection(name = "...")]` is not specified, the macro automatically converts struct names to plural snake_case:

| Struct Name | Default Collection Name | Description |
|-------------|------------------------|-------------|
| `User` | `users` | Singular → Plural |
| `UserProfile` | `user_profiles` | CamelCase → snake_case + Plural |
| `Course` | `courses` | Singular → Plural |
| `Category` | `categories` | y → ies |
| `Book` | `books` | Singular → Plural |
| `CourseEnrollment` | `course_enrollments` | CamelCase → snake_case + Plural |

### Requirements

The `Collection` trait has the following constraints:

- `Send + Sync + Sized` - Required by the trait itself for thread-safe operations

**Recommended traits for MongoDB usage:**

When working with MongoDB collections, your types should typically implement:

- `Serialize` (from serde) - Required for writing to MongoDB
- `Deserialize` (from serde) - Required for reading from MongoDB
- `Debug` - Useful for debugging
- `Clone` - Often needed for data manipulation

**Note:** The `Collection` derive macro itself doesn't enforce serde traits, but MongoDB operations require them for serialization/deserialization.

### Complete Example

```rust
use serde::{Serialize, Deserialize};
use mongo_collection::Collection;

// Example 1: User entity (using default plural naming)
#[derive(Collection, Serialize, Deserialize, Debug, Clone)]
struct User {
    #[serde(rename = "_id")]
    id: String,
    username: String,
    email: String,
}

// Example 2: Course entity (using default plural naming)
#[derive(Collection, Serialize, Deserialize, Debug, Clone)]
struct Course {
    #[serde(rename = "_id")]
    id: String,
    title: String,
    description: String,
}

// Example 3: Category entity (demonstrating smart pluralization)
#[derive(Collection, Serialize, Deserialize, Debug, Clone)]
struct Category {
    #[serde(rename = "_id")]
    id: String,
    name: String,
}

fn main() {
    println!("User collection: {}", User::name());         // Output: users
    println!("Course collection: {}", Course::name());     // Output: courses
    println!("Category collection: {}", Category::name()); // Output: categories
}
```

### Error Handling

If you attempt to use the `Collection` macro on a non-struct type, you'll get a compile error:

```rust
// ❌ Error: Collection can only be derived for structs
#[derive(Collection)]
enum Status {
    Active,
    Inactive,
}
```

### MongoDB Integration

The `Collection` trait provides a convenient `collection()` method for direct MongoDB integration:

```rust
use mongodb::{bson::doc, Database};
use serde::{Serialize, Deserialize};
use mongo_collection::Collection;

#[derive(Collection, Serialize, Deserialize, Debug, Clone)]
struct User {
    #[serde(rename = "_id")]
    id: String,
    username: String,
    email: String,
}

async fn example(db: &Database) {
    // Get the MongoDB collection directly from the trait
    // The collection is automatically named "users" (pluralized)
    let users = User::collection(db);

    // Insert a document
    let new_user = User {
        id: "123".to_string(),
        username: "john".to_string(),
        email: "john@example.com".to_string(),
    };
    users.insert_one(&new_user).await.unwrap();

    // Query documents
    let user = users
        .find_one(doc! { "username": "john" })
        .await
        .unwrap();

    // Update documents
    users
        .update_one(
            doc! { "_id": "123" },
            doc! { "$set": { "email": "newemail@example.com" } },
        )
        .await
        .unwrap();

    // The collection name is accessible
    println!("Collection name: {}", User::name()); // "users"
}
```

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mongo-collection = "0.1"
serde = { version = "1.0", features = ["derive"] }
mongodb = "2.0"
```

### Best Practices

#### Thread Safety

Types implementing `Collection` are required to be `Send + Sync`, making them safe to use across threads. This is particularly important for web servers and concurrent applications:

```rust
use std::sync::Arc;
use tokio::task;
use mongo_collection::Collection;
use serde::{Serialize, Deserialize};

#[derive(Collection, Serialize, Deserialize, Debug, Clone)]
struct User {
    id: String,
    name: String,
}

async fn concurrent_example(db: Arc<mongodb::Database>) {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let db = Arc::clone(&db);
            task::spawn(async move {
                let users = User::collection(&db);
                // Safe to use across threads
                users.find_one(doc! { "id": i.to_string() }).await
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }
}
```

#### Generic Functions

Use the `Collection` trait for generic database operations:

```rust
use mongodb::{bson::Document, Database};
use mongo_collection::Collection;

async fn count_documents<T: Collection>(db: &Database) -> u64 {
    T::collection(db).count_documents(None).await.unwrap()
}

// Usage
let user_count = count_documents::<User>(db).await;
let post_count = count_documents::<Post>(db).await;
```

#### Repository Pattern

Combine with the repository pattern for clean architecture:

```rust
pub struct Repository<T: Collection> {
    collection: mongodb::Collection<T>,
}

impl<T: Collection + Serialize + DeserializeOwned> Repository<T> {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: T::collection(db),
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Option<T> {
        self.collection
            .find_one(doc! { "_id": id })
            .await
            .ok()
            .flatten()
    }

    pub async fn insert(&self, item: &T) -> Result<(), mongodb::error::Error> {
        self.collection.insert_one(item).await?;
        Ok(())
    }
}

// Usage
let user_repo = Repository::<User>::new(db);
let user = user_repo.find_by_id("123").await;
```

## CollectionRepository Derive Macro

The `CollectionRepository` derive macro automatically implements the `CollectionRepository` trait, providing a complete set of CRUD operations for your MongoDB collections.

### Features

- **Complete CRUD operations** - Create, Read, Update, Delete with a single derive
- **Pagination support** - Built-in paginated queries with sorting
- **Type-safe operations** - All operations use MongoDB's native error types
- **Async/await native** - Built with `async-trait` for modern async Rust
- **Zero boilerplate** - Just add the derive macro and start using

### Available Operations

#### Create Operations
- `create()` - Create a single document
- `create_many()` - Batch create multiple documents

#### Read Operations
- `find_by_id()` - Find document by ObjectId string
- `find_one()` - Find single document by filter
- `find_many()` - Find multiple documents with optional sorting/pagination
- `find_all()` - Find all documents in collection
- `find_paginated()` - Paginated query with sorting and metadata
- `count()` - Count documents matching filter
- `exists()` - Check if document exists

#### Update Operations
- `update_by_id()` - Update document by ObjectId string
- `update_one()` - Update single document by filter
- `update_many()` - Update multiple documents by filter
- `find_one_and_update()` - Find and update, returning the document

#### Delete Operations
- `delete_by_id()` - Delete document by ObjectId string
- `delete_one()` - Delete single document by filter
- `delete_many()` - Delete multiple documents by filter
- `find_one_and_delete()` - Find and delete, returning the document

### Quick Start

```rust
use mongo_collection::{Collection, CollectionRepository};
use serde::{Deserialize, Serialize};
use mongodb::bson::doc;

#[derive(Collection, CollectionRepository, Serialize, Deserialize, Debug, Clone)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database("mydb");

    // Create
    let user = User {
        id: None,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    let created = User::create(&db, &user).await?;

    // Read
    let found = User::find_one(&db, doc! { "email": "alice@example.com" }).await?;

    // Update
    if let Some(user_id) = created.id {
        User::update_by_id(&db, &user_id, doc! { "$set": { "name": "Alice Smith" } }).await?;
    }

    // Delete
    User::delete_one(&db, doc! { "email": "alice@example.com" }).await?;

    Ok(())
}
```

### Pagination Example

```rust
use mongo_collection::{Collection, CollectionRepository, PaginatedQuery};
use mongodb::bson::doc;

#[derive(Collection, CollectionRepository, Serialize, Deserialize, Debug, Clone)]
struct Article {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    title: String,
    views: u64,
}

async fn list_articles(db: &mongodb::Database) -> Result<(), Box<dyn std::error::Error>> {
    // Create pagination query
    let query = PaginatedQuery {
        page: 1,
        page_size: 10,
        sort_by: Some("views".to_string()),
        sort_order: SortOrder::Desc,
        ..Default::default()
    };

    // Execute paginated query
    let result = Article::find_paginated(&db, doc! {}, &query).await?;

    println!("Page {}/{}", result.page, result.total_pages);
    println!("Total: {} articles", result.total_count);

    for article in result.items {
        println!("- {} ({} views)", article.title, article.views);
    }

    Ok(())
}
```

### Requirements

To use `CollectionRepository`, your struct must:

1. Derive or implement `Collection` - Provides collection name
2. Derive or implement `Clone` - Required for return values
3. Derive or implement `Serialize` + `Deserialize` - For MongoDB operations
4. Implement `Send + Sync + Unpin` - Usually automatic

### Complete Example

```rust
use mongo_collection::{Collection, CollectionRepository, PaginatedQuery, SortOrder};
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
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database("myapp");

    // 1. Create users
    let users = vec![
        User { id: None, name: "Alice".into(), email: "alice@example.com".into(), age: 25 },
        User { id: None, name: "Bob".into(), email: "bob@example.com".into(), age: 30 },
        User { id: None, name: "Charlie".into(), email: "charlie@example.com".into(), age: 35 },
    ];
    User::create_many(&db, users).await?;

    // 2. Find all users
    let all_users = User::find_all(&db).await?;
    println!("Total users: {}", all_users.len());

    // 3. Find users over 28
    let adults = User::find_many(&db, doc! { "age": { "$gt": 28 } }, None).await?;
    println!("Users over 28: {}", adults.len());

    // 4. Paginated query with sorting
    let query = PaginatedQuery {
        page: 1,
        page_size: 2,
        sort_by: Some("age".to_string()),
        sort_order: SortOrder::Desc,
        ..Default::default()
    };
    let page = User::find_paginated(&db, doc! {}, &query).await?;
    println!("Page {}/{}: {} users", page.page, page.total_pages, page.items.len());

    // 5. Count total users
    let count = User::count(&db, doc! {}).await?;
    println!("Total count: {}", count);

    // 6. Check if user exists
    let exists = User::exists(&db, doc! { "email": "alice@example.com" }).await?;
    println!("Alice exists: {}", exists);

    // 7. Update user
    User::update_one(&db,
        doc! { "email": "alice@example.com" },
        doc! { "$set": { "age": 26 } }
    ).await?;

    // 8. Delete users under 30
    let deleted = User::delete_many(&db, doc! { "age": { "$lt": 30 } }).await?;
    println!("Deleted {} users", deleted);

    Ok(())
}
```

### OpenAPI Support

The pagination types support OpenAPI schema generation when the `openapi` feature is enabled:

```toml
[dependencies]
mongo-collection = { version = "0.1", features = ["openapi"] }
```

With this feature enabled, `PaginatedQuery`, `PaginatedData`, `ListData`, and `SortOrder` will implement `utoipa::ToSchema`:

```rust
use utoipa::OpenApi;
use mongo_collection::{PaginatedQuery, PaginatedData};

#[derive(OpenApi)]
#[openapi(
    components(schemas(PaginatedQuery, PaginatedData<User>))
)]
struct ApiDoc;
```

### License

MIT
