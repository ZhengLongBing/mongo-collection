use inflector::Inflector;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Lit, parse_macro_input};

/// Automatically derives the Collection trait implementation
///
/// # Usage
///
/// ## Basic usage (using default collection name - auto pluralization)
/// ```ignore
/// #[derive(Collection, Serialize, Deserialize, Debug, Clone)]
/// struct User {
///     id: String,
///     name: String,
/// }
/// // Collection name will be "users" (automatically pluralized)
/// ```
///
/// ## Specifying a custom collection name
/// ```ignore
/// #[derive(Collection, Serialize, Deserialize, Debug, Clone)]
/// #[collection(name = "users")]
/// struct User {
///     id: String,
///     name: String,
/// }
/// // Collection name will be "users"
/// ```
#[proc_macro_derive(Collection, attributes(collection))]
pub fn derive_collection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get the struct name
    let name = &input.ident;

    // Look for #[collection(name = "...")] attribute
    let collection_name =
        get_collection_name(&input).unwrap_or_else(|| to_plural_snake_case(&name.to_string()));

    // Ensure it's a struct
    match input.data {
        Data::Struct(_) => {}
        _ => {
            return syn::Error::new_spanned(name, "Collection can only be derived for structs")
                .to_compile_error()
                .into();
        }
    }

    // Generate implementation
    let expanded = quote! {
        impl Collection for #name {
            fn name() -> &'static str {
                #collection_name
            }
        }
    };

    TokenStream::from(expanded)
}

/// Extracts the collection name from attributes
fn get_collection_name(input: &DeriveInput) -> Option<String> {
    for attr in &input.attrs {
        if attr.path().is_ident("collection") {
            if let Ok(meta_list) = attr.meta.require_list() {
                // Store parsing result
                let mut name = None;

                // Parse name = "value" format
                let _ = meta_list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        let lit: Lit = value.parse()?;
                        if let Lit::Str(lit_str) = lit {
                            name = Some(lit_str.value());
                        }
                    }
                    Ok(())
                });

                if name.is_some() {
                    return name;
                }
            }
        }
    }
    None
}

/// Converts CamelCase to plural snake_case
/// Examples: "User" -> "users", "UserProfile" -> "user_profiles"
fn to_plural_snake_case(s: &str) -> String {
    // Use inflector to convert CamelCase to snake_case and pluralize
    s.to_table_case()
}

/// Automatically derives the CollectionRepository trait implementation
///
/// This macro automatically implements the `CollectionRepository` trait for a struct
/// that already implements the `Collection` trait. The struct must also implement
/// `Clone`, `Send`, `Sync`, `Unpin`, `Serialize`, and `Deserialize`.
///
/// # Usage
///
/// ```ignore
/// use mongo_collection::{Collection, CollectionRepository};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Collection, CollectionRepository, Serialize, Deserialize, Debug, Clone)]
/// struct User {
///     #[serde(rename = "_id")]
///     id: String,
///     name: String,
///     email: String,
/// }
///
/// // Now you can use all CRUD operations:
/// // User::create(&db, &user).await?;
/// // User::find_by_id(&db, "123").await?;
/// // User::find_paginated(&db, doc!{}, &query).await?;
/// ```
///
/// # Requirements
///
/// The struct must derive or implement:
/// - `Collection` - Provides collection name and reference
/// - `Clone` - Required for return values
/// - `Serialize` - For MongoDB serialization
/// - `Deserialize` - For MongoDB deserialization
///
/// # Example with all derives
///
/// ```ignore
/// #[derive(Collection, CollectionRepository, Serialize, Deserialize, Debug, Clone)]
/// #[collection(name = "users")]
/// struct User {
///     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
///     id: Option<String>,
///     name: String,
///     email: String,
/// }
/// ```
#[proc_macro_derive(CollectionRepository)]
pub fn derive_collection_repository(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get the struct name
    let name = &input.ident;

    // Ensure it's a struct
    match input.data {
        Data::Struct(_) => {}
        _ => {
            return syn::Error::new_spanned(name, "CollectionRepository can only be derived for structs")
                .to_compile_error()
                .into();
        }
    }

    // Generate implementation
    // Note: We don't need to implement any methods because CollectionRepository
    // trait provides default implementations for all methods
    let expanded = quote! {
        impl CollectionRepository for #name {}
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_plural_snake_case() {
        assert_eq!(to_plural_snake_case("User"), "users");
        assert_eq!(to_plural_snake_case("UserProfile"), "user_profiles");
        assert_eq!(to_plural_snake_case("Course"), "courses");
        assert_eq!(to_plural_snake_case("Category"), "categories");
        assert_eq!(to_plural_snake_case("Book"), "books");
    }
}
