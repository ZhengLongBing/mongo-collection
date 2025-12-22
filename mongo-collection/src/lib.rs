//! # MongoDB Collection Trait
//!
//! This crate provides a `Collection` trait and derive macro for seamlessly integrating
//! Rust types with MongoDB collections.
//!
//! ## Quick Start
//!
//! ```ignore
//! use serde::{Serialize, Deserialize};
//! use mongo_collection::Collection;
//!
//! #[derive(Collection, Serialize, Deserialize, Debug, Clone)]
//! struct User {
//!     id: String,
//!     name: String,
//! }
//!
//! // Use the collection name
//! assert_eq!(User::name(), "users");
//!
//! // Get MongoDB collection
//! let users = User::collection(&db);
//! ```

use mongodb::Database;

pub use mongo_collection_macro::Collection;

/// Trait for types that map to MongoDB collections.
///
/// This trait provides methods to get the collection name and obtain a typed
/// MongoDB collection reference. It is typically derived using the `#[derive(Collection)]`
/// macro rather than implemented manually.
///
/// # Examples
///
/// ```ignore
/// use mongo_collection::Collection;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Collection, Serialize, Deserialize, Debug, Clone)]
/// struct User {
///     username: String,
/// }
///
/// // Get the collection name
/// let name = User::name(); // "users"
///
/// // Get a typed MongoDB collection
/// let users = User::collection(&db);
/// ```
pub trait Collection: Send + Sync + Sized {
    /// Returns the name of the MongoDB collection for this type.
    ///
    /// When using the derive macro without specifying a custom name,
    /// this returns the pluralized snake_case version of the struct name.
    fn name() -> &'static str;

    /// Returns a typed MongoDB collection reference for this type.
    ///
    /// # Arguments
    ///
    /// * `db` - A reference to the MongoDB database
    ///
    /// # Returns
    ///
    /// A `mongodb::Collection<Self>` that can be used for database operations.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let users = User::collection(&db);
    /// let user = users.find_one(doc! { "username": "alice" }).await?;
    /// ```
    fn collection(db: &Database) -> mongodb::Collection<Self> {
        db.collection(Self::name())
    }
}
