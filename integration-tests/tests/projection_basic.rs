//! Integration tests for the `project!` macro.
//!
//! We simulate the generated `*_fields!` helper and then create
//! projections with `only` / `except`. This gives high confidence
//! that the expanded code is correct.

use macros::project;

// ---------------------------------------------------------------------------
// Fake model + generated-style helper (simulates what the DB generator emits)
// ---------------------------------------------------------------------------

type PKeyType = i64;

/// Simulates the macro that the generator places next to each model.
macro_rules! todo_fields {
    ($($rest:tt)*) => {
        project! {
            Todo {
                id: PKeyType,
                title: String,
                done: bool,
                created_at: Option<String>,
                updated_at: Option<String>,
                deleted_at: Option<String>,
            }
            $($rest)*
        }
    };
}

// ---------------------------------------------------------------------------
// Projections under test
// ---------------------------------------------------------------------------

todo_fields!(TodoRow: only(id, title, done));
todo_fields!(TodoCombo: except(done, created_at, updated_at, deleted_at));
todo_fields!(TodoFull: only(id, title, done, created_at, updated_at, deleted_at));

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn only_projection_has_expected_fields_and_columns() {
    // Compile-time check: these fields must exist
    let row = TodoRow {
        id: 1,
        title: "Buy milk".into(),
        done: false,
    };

    assert_eq!(row.id, 1);
    assert_eq!(row.title, "Buy milk");
    assert!(!row.done);

    // COLUMNS must stay in sync with the struct
    assert_eq!(TodoRow::COLUMNS, "id, title, done");
}

#[test]
fn except_projection_drops_requested_fields() {
    let combo = TodoCombo {
        id: 42,
        title: "Important".into(),
    };

    assert_eq!(combo.id, 42);
    assert_eq!(combo.title, "Important");

    // Only id + title should remain
    assert_eq!(TodoCombo::COLUMNS, "id, title");
}

#[test]
fn full_projection_keeps_everything() {
    let full = TodoFull {
        id: 7,
        title: "Complete".into(),
        done: true,
        created_at: Some("2024-01-01".into()),
        updated_at: None,
        deleted_at: None,
    };

    assert_eq!(full.id, 7);
    assert!(full.done);
    assert_eq!(full.created_at.as_deref(), Some("2024-01-01"));

    assert_eq!(
        TodoFull::COLUMNS,
        "id, title, done, created_at, updated_at, deleted_at"
    );
}

#[test]
fn projections_are_serializable() {
    let row = TodoRow {
        id: 1,
        title: "Test".into(),
        done: true,
    };

    // Just make sure Serialize works (we don't care about the exact JSON shape here)
    let json = serde_json::to_value(&row).expect("should serialize");
    assert_eq!(json["id"], 1);
    assert_eq!(json["title"], "Test");
    assert_eq!(json["done"], true);
}

#[test]
fn projections_are_deserializable() {
    let json = serde_json::json!({
        "id": 99,
        "title": "From JSON",
        "done": false
    });

    let row: TodoRow = serde_json::from_value(json).expect("should deserialize");
    assert_eq!(row.id, 99);
    assert_eq!(row.title, "From JSON");
    assert!(!row.done);
}
