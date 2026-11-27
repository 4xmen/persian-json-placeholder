//! Rusatavel Procedural macros exposed by this crate.
//!
//! This file contains only the public `proc_macro_derive` entry points and macro
//! registrations. The implementation details, parsing logic, code generation, and
//! supporting utilities are organized into dedicated modules and files to keep the
//! crate structure maintainable and easy to navigate.
//!
//! Available derive macros:
//!
//! * `checkmate` - validator
//! * `pawn` - factory
//!
//! For implementation details of a specific macro, refer to its corresponding
//! module.

use crate::checkmate::*;
use crate::pawn::pawn_expand;
use crate::projection::ProjectInput;
use crate::resource::{ResourceDef, ResponseDef};
use proc_macro::TokenStream;
use quote::quote;
use quote::spanned::Spanned;
use std::collections::HashSet;
use syn::Error;
use syn::{DeriveInput, parse_macro_input};

mod checkmate;
mod pawn;
mod projection;
mod resource;

/// Derive macro for the CheckMate validation system.
///
/// CheckMate provides a Laravel-inspired validation experience for Rust.
/// Validation rules are declared with the #[validating(...)] attribute on
/// struct fields, and an asynchronous validate() method is generated
/// automatically.
///
/// Refer to the module documentation for detailed rule definitions and examples.
#[proc_macro_derive(CheckMate, attributes(validating))]
pub fn mate_validate(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree (DeriveInput represents the struct)
    let mut ast = parse_macro_input!(input as DeriveInput);

    // We'll collect all the parsed rules for each field in a String for display in tests
    // let mut rules_display = String::new();

    // Check if the derive is on a struct (we assume it is, but in full version, add error handling)
    let mut validations = Vec::new();

    if let syn::Data::Struct(data_struct) = &mut ast.data {
        // collect all fields name required in safe validation like confirm
        let field_names: HashSet<String> = data_struct
            .fields
            .iter()
            .filter_map(|field| field.ident.as_ref().map(|ident| ident.to_string()))
            .collect();

        // Iterate over each field in the struct
        for field in data_struct.fields.iter_mut() {
            // Find the #[validating] attribute on this field
            let validating_attr = find_validating_attr(field);

            if let Some(attr) = validating_attr {
                // Parse the rules from the attribute (either single string or list) into Vec<Rule>
                match parse_rules(&attr, &field_names) {
                    Ok(rules) => {
                        // check nullable can't be a non Option<_> type
                        if rules.iter().any(|r| matches!(r, Rule::Nullable))
                            && !is_option_type(&field.ty)
                        {
                            return Error::new(
                                field.ty.__span(),
                                format!(
                                    "Field '{}' is marked as nullable but is not Option<T>",
                                    field.ident.as_ref().unwrap()
                                ),
                            )
                            .to_compile_error()
                            .into();
                        }

                        let field_ident = field.ident.as_ref().unwrap();
                        let field_name = field_ident.to_string();
                        let field_ty = &field.ty;

                        // runtime validation code generation
                        for rule in &rules {
                            validations.push(rule.expand(field_ident, field_ty, &field_name));
                        }

                        // let rules_str: Vec<String> = rules.iter().map(|r| r.as_str()).collect();
                        // rules_display.push_str(&format!(
                        //     "{}: {}\n",
                        //     field_name,
                        //     rules_str.join("|")
                        // ));
                    }
                    Err(err) => {
                        return err.to_compile_error().into();
                    }
                }
            }
        }
    }

    // Generate a simple impl for the struct with an associated function to display the parsed rules
    // This is static (no &self needed) since rules are compile-time known
    // In tests, we can call Struct::display_parsed_rules()
    let struct_name = &ast.ident;
    // let lit = LitStr::new(&rules_display, Span::call_site());
    let r#gen = quote! {

        // impl  #struct_name {
        //     async fn validate(&self) -> Result<(), macros_core::ValidationErrors> {
        impl macros_core::CheckMateValidator for #struct_name {
            fn validate(&self) -> impl std::future::Future<Output = Result<(), macros_core::ValidationErrors>> + Send {
                #struct_name::validate(self)
            }
        }

        impl  #struct_name {
            pub async fn validate(&self) -> Result<(), macros_core::ValidationErrors> {
                let mut errors = macros_core::ValidationErrors::new();

                #(#validations)*

                 // println!("{:?}", self);

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    r#gen.into()
}

/// Derives `macros::Pawn`, generating a `definition()` that builds one fully
/// populated instance from each field's attribute.
///
/// Supported field attributes:
/// - `#[fake(name)]`, `#[fake(username)]`, `#[fake(email)]`
/// - `#[fake(password(length = N))]`
/// - `#[fake(lorem(words = N))]`
/// - `#[generator(path::to::function)]` — calls a zero-argument function
/// - `#[value(expr)]` — uses a literal/expression verbatim
///
/// Fields without any of these attributes fall back to `Default::default()`.
#[proc_macro_derive(Pawn, attributes(fake, generator, value))]
pub fn derive_factory(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    pawn_expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Defines a serialization projection (Resource).
///
/// Every value expression reads from `this`, the borrowed source value.
///
/// ```ignore
/// resource! {
///     ProductResource from Product {
///         id,                                                    // same as `id = this.id`
///         title,
///         price = this.amount,                                   // rename
///         created_at = to_jalali(&this.created_at),              // transform
///         categories: Vec<CategoryResource> = this.categories,   // nested resources
///     }
/// }
/// ```
///
/// The `Vec<X>` / `Option<X>` annotation is not a real field type. It only tells
/// the macro that `X` is another resource. Omit the annotation for plain data
/// such as `Vec<String>`.
#[proc_macro]
pub fn resource(input: TokenStream) -> TokenStream {
    // Parse the definition, then generate the struct and trait impl.
    let def = parse_macro_input!(input as ResourceDef);
    def.expand().into()
}

/// Builds a response object that serializes directly to JSON without an
/// intermediate `serde_json::Value`.
///
/// # Example
///
/// ```ignore
/// Json(resource_response! {
///     data: ProductResource::from((product, categories)),
///     user: UserResource::from(user),
///     comments: CommentResource::collection(comments),
/// })
/// ```
#[proc_macro]
pub fn resource_response(input: TokenStream) -> TokenStream {
    // Parse the `name: expr` list, then generate a local serializable struct.
    let response = parse_macro_input!(input as ResponseDef);
    response.expand().into()
}

/// Generates a projection struct + a `COLUMNS` constant.
///
/// The generated struct always derives:
/// `Clone`, `serde::Serialize`, `serde::Deserialize`, `sqlx::FromRow`.
///
/// # Example
/// ```ignore
/// project! {
///     Todo {
///         id: i64,
///         title: String,
///         done: bool,
///     }
///     TodoRow: only(id, title)
/// }
/// ```
#[proc_macro]
pub fn project(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ProjectInput);
    match projection::expand(input) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}
