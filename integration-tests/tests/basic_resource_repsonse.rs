//! Integration tests for the `resource!` and `resource_response!` macros.
//!
//! These tests intentionally mirror real-world usage:
//! - Models come from sqlx / database and do NOT contain nested relations.
//! - Related records (categories, comments, ...) arrive from separate queries.
//! - We combine them at the handler layer using a composite source.
//! - Field transformations and computed fields are supported via expressions.

use macros::{resource_response, resource};
use serde::Serialize;
use serde_json::json;

// ---------------------------------------------------------------------------
// Real-world style models (what sqlx would give you)
// ---------------------------------------------------------------------------

// `category_id` and `user_id` model database columns that are deliberately
// NOT exposed by any resource, so they are never read.
#[allow(dead_code)]
struct Product {
    id: u64,
    name: String,
    description: Option<String>,
    slug: String,
    price: u64,
    quantity: i32,
    category_id: Option<u64>,
    user_id: u64,
    image: Option<String>,
    thumb: Option<String>,
    created_at: String, // simplified; in real code this would be time::OffsetDateTime
}

struct Category {
    id: u64,
    name: String,
    slug: String,
}

struct User {
    id: u64,
    name: String,
}

// ---------------------------------------------------------------------------
// Test data builders
// ---------------------------------------------------------------------------

/// Builds a product row with fixed defaults; tests override only what they assert on.
fn product(id: u64, name: &str, slug: &str) -> Product {
    Product {
        id,
        name: name.into(),
        description: None,
        slug: slug.into(),
        price: 100,
        quantity: 2,
        category_id: None,
        user_id: 1,
        image: None,
        thumb: None,
        created_at: "2024-01-01".into(),
    }
}

/// Builds a category row.
fn category(id: u64, name: &str, slug: &str) -> Category {
    Category {
        id,
        name: name.into(),
        slug: slug.into(),
    }
}

// ---------------------------------------------------------------------------
// Helper functions used inside resource expressions
// ---------------------------------------------------------------------------

/// Fake jalali converter. Real implementation would do proper conversion.
/// We only care that the expression is evaluated and the result is serializable.
fn to_jalali(date: &str) -> String {
    format!("jalali:{date}")
}

/// Build a simple absolute URL from a slug.
fn product_url(slug: &str) -> String {
    format!("http://127.0.0.1/{slug}")
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

resource! {
    CategoryResource from Category {
        id,
        name,
        slug,
    }
}

resource! {
    UserResource from User {
        id,
        name,
    }
}

// Realistic case: Product + Categories come from separate queries.
// Also demonstrates:
//   1. Field transformation  (created_at → to_jalali)
//   2. Computed new field    (url built from slug)
resource! {
    ProductResource from (Product, Vec<Category>) {
        id            = this.0.id,
        name          = this.0.name,
        description   = this.0.description,
        slug          = this.0.slug,
        price         = this.0.price,
        quantity      = this.0.quantity,
        image         = this.0.image,
        thumb         = this.0.thumb,

        // 1. Transform a primitive field into a different type/representation
        created_at    = to_jalali(&this.0.created_at),

        // 2. Compute a brand-new field that does not exist on the source
        url           = product_url(&this.0.slug),

        // Nested collection from a separate query
        categories: Vec<CategoryResource> = this.1,
    }
}

// Nested optional resource (e.g. a nullable relation loaded separately).
resource! {
    ProductWithCategoryResource from (Product, Option<Category>) {
        id = this.0.id,
        category: Option<CategoryResource> = this.1,
    }
}

// Shorthand fields, a raw identifier key and an operator expression.
resource! {
    LineResource from Product {
        id,

        // `type` is a Rust keyword, so a raw identifier is used; the JSON key must be `type`.
        r#type = this.slug,

        // Binary expressions must not be broken by the macro's `&` wrapping.
        total = this.price * this.quantity as u64,
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Serializes any value to `serde_json::Value`, panicking on failure.
fn to_value<T: Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect("serialization should succeed")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn single_product_with_categories_from_separate_query() {
    let product = Product {
        description: Some("A powerful laptop".into()),
        price: 1500,
        quantity: 5,
        image: Some("laptop.jpg".into()),
        thumb: Some("laptop_thumb.jpg".into()),
        created_at: "2024-05-10T12:00:00Z".into(),
        ..product(1, "Laptop", "laptop")
    };

    let categories = vec![
        category(10, "Electronics", "electronics"),
        category(11, "Computers", "computers"),
    ];

    let value = to_value(ProductResource::from((product, categories)));

    assert_eq!(
        value,
        json!({
            "id": 1,
            "name": "Laptop",
            "description": "A powerful laptop",
            "slug": "laptop",
            "price": 1500,
            "quantity": 5,
            "image": "laptop.jpg",
            "thumb": "laptop_thumb.jpg",
            "created_at": "jalali:2024-05-10T12:00:00Z",
            "url": "http://127.0.0.1/laptop",
            "categories": [
                { "id": 10, "name": "Electronics", "slug": "electronics" },
                { "id": 11, "name": "Computers", "slug": "computers" }
            ]
        })
    );
}

#[test]
fn field_transformation_to_jalali() {
    let product = Product {
        created_at: "2023-01-15".into(),
        ..product(99, "Test", "test")
    };

    let value = to_value(ProductResource::from((product, vec![])));

    // The important assertion: the transformation function was called
    assert_eq!(value["created_at"], "jalali:2023-01-15");
}

#[test]
fn computed_url_field() {
    let product = product(7, "Keyboard", "mechanical-keyboard");

    let value = to_value(ProductResource::from((product, vec![])));

    // Brand-new field that does not exist on the source model
    assert_eq!(value["url"], "http://127.0.0.1/mechanical-keyboard");
}

#[test]
fn product_with_empty_categories() {
    let product = Product {
        created_at: "2024-02-20".into(),
        ..product(2, "Phone", "phone")
    };

    let value = to_value(ProductResource::from((product, vec![])));

    assert_eq!(value["categories"], json!([]));
    assert_eq!(value["created_at"], "jalali:2024-02-20");
    assert_eq!(value["url"], "http://127.0.0.1/phone");
}

#[test]
fn collection_of_products_with_grouped_categories() {
    let rows = vec![
        (
            Product {
                created_at: "2024-03-01".into(),
                ..product(1, "Laptop", "laptop")
            },
            vec![category(10, "Electronics", "electronics")],
        ),
        (
            Product {
                created_at: "2024-03-02".into(),
                ..product(2, "Phone", "phone")
            },
            vec![],
        ),
    ];

    let value = to_value(ProductResource::collection(rows));

    assert_eq!(value[0]["url"], "http://127.0.0.1/laptop");
    assert_eq!(value[0]["created_at"], "jalali:2024-03-01");
    assert_eq!(value[1]["categories"], json!([]));
}

#[test]
fn multiple_resources_in_one_response() {
    let product = product(1, "Laptop", "laptop");
    let categories = vec![category(10, "Electronics", "electronics")];
    let user = User {
        id: 7,
        name: "Ali".into(),
    };

    let product_json = to_value(ProductResource::from((product, categories)));
    let user_json = to_value(UserResource::from(user));

    assert_eq!(product_json["name"], "Laptop");
    assert_eq!(product_json["url"], "http://127.0.0.1/laptop");
    assert_eq!(user_json, json!({ "id": 7, "name": "Ali" }));
}

#[test]
fn resource_response_macro() {
    let product = Product {
        created_at: "2024-05-01".into(),
        ..product(1, "Laptop", "laptop")
    };
    let categories = vec![category(10, "Electronics", "electronics")];
    let user = User {
        id: 7,
        name: "Ali".into(),
    };

    // Mixes a single resource and a collection, with a trailing comma.
    let response = resource_response! {
        data: ProductResource::from((product, categories)),
        user: UserResource::from(user),
        all_categories: CategoryResource::collection(vec![category(11, "Computers", "computers")]),
    };

    let value = to_value(response);

    assert_eq!(value["data"]["name"], "Laptop");
    assert_eq!(value["data"]["url"], "http://127.0.0.1/laptop");
    assert_eq!(value["data"]["created_at"], "jalali:2024-05-01");
    assert_eq!(value["user"]["name"], "Ali");
    assert_eq!(value["data"]["categories"][0]["name"], "Electronics");
    assert_eq!(value["all_categories"][0]["name"], "Computers");
}

#[test]
fn nested_optional_resource() {
    // Present relation is projected through the other resource.
    let with_category = ProductWithCategoryResource::from((
        product(1, "Laptop", "laptop"),
        Some(category(10, "Electronics", "electronics")),
    ));
    let value = to_value(with_category);
    assert_eq!(
        value["category"],
        json!({ "id": 10, "name": "Electronics", "slug": "electronics" })
    );

    // Missing relation becomes a JSON null.
    let without_category =
        ProductWithCategoryResource::from((product(2, "Phone", "phone"), None));
    let value = to_value(without_category);
    assert_eq!(value["category"], serde_json::Value::Null);
}

#[test]
fn raw_identifier_key_and_operator_expression() {
    let product = Product {
        price: 120,
        quantity: 3,
        ..product(1, "Keyboard", "keyboard")
    };

    let value = to_value(LineResource::from(product));

    // `r#type` must be emitted as `type`, and `price * quantity` must be computed.
    assert_eq!(value, json!({ "id": 1, "type": "keyboard", "total": 360 }));
}
