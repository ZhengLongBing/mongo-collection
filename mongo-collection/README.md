# MongoDB Collection Macros

Procedural macros for MongoDB collection trait derivation.

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

### License

MIT
