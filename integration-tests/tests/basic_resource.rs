//! Basic integration tests for the `resource!` macro.

use macros::{resource};
use serde::Serialize;
use serde_json::json;

// ---------------------------------------------------------------------------
// Simple models
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct Category {
    id: u64,
    name: String,
}

#[derive(Clone)]
struct Product {
    id: u64,
    title: String,
    price: i64,
    /// Many-to-many style relation
    categories: Vec<Category>,
    /// Optional single relation
    primary_category: Option<Category>,
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

resource! {
    CategoryResource from Category {
        id,
        name,
    }
}

resource! {
    ProductResource from Product {
        id,
        title,
        price,
        categories: Vec<CategoryResource> = this.categories,
        primary_category: Option<CategoryResource> = this.primary_category,
    }
}

// Composite source (when data comes from separate queries)
resource! {
    ProductWithCategories from (Product, Vec<Category>) {
        id         = this.0.id,
        title      = this.0.title,
        price      = this.0.price,
        categories: Vec<CategoryResource> = this.1,
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn to_value<T: Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect("serialization should succeed")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn single_resource_with_nested_vec_and_option() {
    let product = Product {
        id: 1,
        title: "Laptop".into(),
        price: 1500,
        categories: vec![
            Category {
                id: 10,
                name: "Electronics".into(),
            },
            Category {
                id: 11,
                name: "Computers".into(),
            },
        ],
        primary_category: Some(Category {
            id: 10,
            name: "Electronics".into(),
        }),
    };

    let value = to_value(ProductResource::from(product));

    assert_eq!(
        value,
        json!({
            "id": 1,
            "title": "Laptop",
            "price": 1500,
            "categories": [
                { "id": 10, "name": "Electronics" },
                { "id": 11, "name": "Computers" }
            ],
            "primary_category": {
                "id": 10,
                "name": "Electronics"
            }
        })
    );
}

#[test]
fn option_none() {
    let product = Product {
        id: 2,
        title: "Phone".into(),
        price: 800,
        categories: vec![],
        primary_category: None,
    };

    let value = to_value(ProductResource::from(product));

    assert_eq!(
        value,
        json!({
            "id": 2,
            "title": "Phone",
            "price": 800,
            "categories": [],
            "primary_category": null
        })
    );
}

#[test]
fn collection() {
    let products = vec![
        Product {
            id: 1,
            title: "Laptop".into(),
            price: 1500,
            categories: vec![Category {
                id: 10,
                name: "Electronics".into(),
            }],
            primary_category: Some(Category {
                id: 10,
                name: "Electronics".into(),
            }),
        },
        Product {
            id: 2,
            title: "Phone".into(),
            price: 800,
            categories: vec![],
            primary_category: None,
        },
    ];

    let value = to_value(ProductResource::collection(products));

    assert_eq!(
        value,
        json!([
            {
                "id": 1,
                "title": "Laptop",
                "price": 1500,
                "categories": [
                    { "id": 10, "name": "Electronics" }
                ],
                "primary_category": {
                    "id": 10,
                    "name": "Electronics"
                }
            },
            {
                "id": 2,
                "title": "Phone",
                "price": 800,
                "categories": [],
                "primary_category": null
            }
        ])
    );
}

#[test]
fn composite_source() {
    let product = Product {
        id: 1,
        title: "Laptop".into(),
        price: 1500,
        categories: vec![], // ignored in this mapping
        primary_category: None,
    };

    let categories = vec![
        Category {
            id: 10,
            name: "Electronics".into(),
        },
        Category {
            id: 11,
            name: "Computers".into(),
        },
    ];

    let value = to_value(ProductWithCategories::from((product, categories)));

    assert_eq!(
        value,
        json!({
            "id": 1,
            "title": "Laptop",
            "price": 1500,
            "categories": [
                { "id": 10, "name": "Electronics" },
                { "id": 11, "name": "Computers" }
            ]
        })
    );
}

#[test]
fn rename_and_expression() {
    fn double(n: i64) -> i64 {
        n * 2
    }

    resource! {
        PriceResource from Product {
            id,
            name = this.title,
            double_price = double(this.price),
        }
    }

    let product = Product {
        id: 42,
        title: "Mouse".into(),
        price: 25,
        categories: vec![],
        primary_category: None,
    };

    let value = to_value(PriceResource::from(product));

    assert_eq!(
        value,
        json!({
            "id": 42,
            "name": "Mouse",
            "double_price": 50
        })
    );
}
