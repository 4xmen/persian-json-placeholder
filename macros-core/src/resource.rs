//! Runtime support types for the `resource!` macro.
//!
//! This crate provides the building blocks that the generated code relies on.
//! Users should normally depend on the `resource` crate instead of this one.

use serde::ser::{Serialize, SerializeSeq, Serializer};

/// Internal trait implemented by every generated Resource.
///
/// It knows how to serialize a reference to its `Source` type.
/// The macro generates an implementation of this trait for each Resource.
#[doc(hidden)]
pub trait ResourceSpec {
    /// The underlying data type this resource projects from.
    type Source;

    /// Serialize a single source value.
    fn serialize_source<S>(source: &Self::Source, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

/// Owned wrapper around a single source value.
///
/// This is what `ResourceName::from(value)` returns.
/// It implements `Serialize` by delegating to the ResourceSpec.
pub struct Resource<R: ResourceSpec> {
    source: R::Source,
}

impl<R: ResourceSpec> Resource<R> {
    /// Create a new resource from an owned source.
    #[inline]
    pub fn new(source: R::Source) -> Self {
        Self { source }
    }
}

impl<R: ResourceSpec> Serialize for Resource<R> {
    /// Serializes the owned source through the generated projection.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Delegate to the macro-generated mapping for this resource.
        R::serialize_source(&self.source, serializer)
    }
}

/// Owned collection of source values.
///
/// This is what `ResourceName::collection(items)` returns.
/// It serializes as a JSON array without creating intermediate Resource instances.
pub struct ResourceCollection<R: ResourceSpec> {
    items: Vec<R::Source>,
}

impl<R: ResourceSpec> ResourceCollection<R> {
    /// Create a new collection from owned items.
    #[inline]
    pub fn new(items: Vec<R::Source>) -> Self {
        Self { items }
    }
}

impl<R: ResourceSpec> Serialize for ResourceCollection<R> {
    /// Serializes all items as a sequence.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Reuse the borrowed helper so the sequence logic lives in one place.
        NestedCollection::<R> { items: &self.items }.serialize(serializer)
    }
}

/// Internal helper that serializes a borrowed source using a ResourceSpec.
/// Used by nested collections.
#[doc(hidden)]
pub struct ResourceRef<'a, R: ResourceSpec> {
    pub source: &'a R::Source,
}

impl<'a, R: ResourceSpec> Serialize for ResourceRef<'a, R> {
    /// Serializes the borrowed source through the generated projection.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Same projection as the owned wrapper, just without taking ownership.
        R::serialize_source(self.source, serializer)
    }
}

/// Helper used by the macro for nested `Vec<OtherResource>`.
///
/// Serializes a slice of sources as an array of the other resource,
/// without allocating any intermediate Vec of Resource wrappers.
#[doc(hidden)]
pub struct NestedCollection<'a, R: ResourceSpec> {
    pub items: &'a [R::Source],
}

impl<'a, R: ResourceSpec> Serialize for NestedCollection<'a, R> {
    /// Serializes the slice as a sequence of projected items.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // The length is known up front, which lets formats pre-size the output.
        let mut seq = serializer.serialize_seq(Some(self.items.len()))?;
        for item in self.items {
            // Project each item by reference, no Resource wrapper is allocated.
            seq.serialize_element(&ResourceRef::<R> { source: item })?;
        }
        seq.end()
    }
}

/// Helper used by the macro for nested `Option<OtherResource>`.
#[doc(hidden)]
pub struct NestedOption<'a, R: ResourceSpec> {
    pub item: Option<&'a R::Source>,
}

impl<'a, R: ResourceSpec> Serialize for NestedOption<'a, R> {
    /// Serializes the projected value, or `null` when there is none.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.item {
            // Present: project it like any other resource.
            Some(source) => R::serialize_source(source, serializer),
            // Absent: emit a JSON null.
            None => serializer.serialize_none(),
        }
    }
}
