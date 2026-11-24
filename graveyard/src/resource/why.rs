//! # Why `resource!` (and not `#[derive(Resource)]`)
//!
//! Decision record for the resource system. Newest revision first.
//! Older revisions are kept on purpose: this repo is a trial-and-error log,
//! and a rejected design is only useful if the reason it was rejected survives.
//!
//! ## Scope
//!
//! A Resource answers exactly one question: "what shape does this data have on
//! the way out?". Its whole job is to hand the final serialized output (JSON)
//! to axum. It does not own data, validate, authorize, paginate or load
//! relations. Every decision below follows from this scope.
//!
//! The only contract with the outside world is serde's `Serialize`. JSON for
//! axum is the main consumer, but nothing in the design is JSON-specific:
//! anything that accepts a `Serialize` value can consume a resource.
//!
//! ## Why this is not a DTO
//!
//! A DTO is a second, typed copy of the data that is built, stored and then
//! passed on. A resource holds no fields: it is a unit struct that only
//! describes how to read the source and how to write the output. Nothing is
//! built or stored between the source value and the serializer. Because the
//! scope ends at serialization, there is nothing for an intermediate type to
//! do. Skipping allocations is a side effect of that, not the goal, and it has
//! not been measured.
//!
//! ## Advantages over the attribute derive (see `graveyard/src/resource`)
//!
//! 1. **One linear declaration.** The whole mapping is a single block. A reader
//!    no longer reconstructs it from struct-level and field-level attributes.
//! 2. **No overloaded keyword.** `#[source]` meant a type on the struct and a
//!    field name on fields. Now `from` names the source type and each field
//!    expression names where its value comes from.
//! 3. **Transform and compute are plain Rust.** `#[transform]` and `#[compute]`
//!    collapse into ordinary expressions (`format_date(&this.created_at)`,
//!    `this.price * this.quantity`). There is no attribute vocabulary to learn
//!    and no rule about which attribute may combine with which.
//! 4. **Borrowing by construction.** Expressions run against `&Source`
//!    (`this`) while serializing. Nothing is moved out, so several output
//!    fields can read the same source field and no clone is forced. This fixes
//!    both "future bottlenecks" recorded in the graveyard note.
//! 5. **Composite sources.** A source can be a tuple such as
//!    `(Product, Vec<Category>)`, which matches data that arrives from separate
//!    queries. No joined struct has to be assembled first.
//! 6. **Far less machinery.** One macro file instead of a three-phase pipeline
//!    (AST, type analysis, expansion) with its own error accumulator. Less code
//!    to maintain, which matches the "is this actually necessary?" rule.
//! 7. **Compiler-owned diagnostics.** Type and borrow errors come straight from
//!    rustc, pointing at the user's own expression.
//!
//! ## Costs we accept
//!
//! - **No typed output struct.** Output fields cannot be introspected, read back
//!   in Rust code, or used to derive OpenAPI schemas. Tests go through
//!   `serde_json::Value`.
//! - **The nested annotation is a hint, not a type.** `Vec<CategoryResource>`
//!   only tells the macro that the value is another resource. The macro still
//!   decides this syntactically, exactly like the old derive did, so aliases
//!   and other containers are not recognized.
//! - **A forgotten annotation fails silently.** Without it the raw source value
//!   is serialized as-is. See the rule below.
//! - **Owned constructors only.** `from` and `collection` take ownership. There
//!   is no borrowed constructor yet.
//! - **No conditional fields.** `None` always becomes `null`; a field cannot be
//!   omitted.
//! - **Custom diagnostics are limited to syntax errors.** The old derive
//!   reported every attribute mistake at once with precise spans.
//!
//! ## Rules that keep the design safe
//!
//! - Database models must NOT derive `Serialize`. Then a forgotten nested
//!   annotation is a compile error instead of a data leak.
//! - Every value expression reads from `this`; shorthand `id,` means
//!   `id = this.id` and only works on plain structs.
//!
//! ## When to use a plain `#[derive(Serialize)]` instead
//!
//! If the output is a 1:1 copy of the model, a derive on a dedicated struct is
//! simpler than this macro. The macro earns its place only when the output
//! differs from the source: renames, transforms, computed fields or composite
//! sources.
//!
//! ## Open questions
//!
//! - Borrowed construction (`view(&source)`), for handlers that only hold a
//!   reference.
//! - Conditional fields, once a real endpoint needs one.
//!
//! ## Revision history
//!
//! 1. **Attribute derive** (`#[derive(Resource)]`, `#[source]`, `#[transform]`,
//!    `#[compute]`). Sound implementation, abandoned for DX: mapping scattered
//!    across attributes, plus by-value transforms and no `&Source` support.
//!    Preserved in `graveyard/src/resource`.
//! 2. **Declarative `resource!` macro** (current). Keeps the purity and
//!    structural Vec/Option handling of revision 1, replaces the attribute DSL
//!    with a single mapping block.
//! 3. **Hardening after review** (2026-11). Value expressions are parenthesized
//!    so operators work, raw identifiers such as `r#type` produce plain JSON
//!    keys, duplicate fields and missing commas are compile errors, unused
//!    dependencies and the unusable `Clone` derive were removed.
