use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Error, Ident, Result, Token, Type, braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// A single `name: Type` field from the model definition.
struct Field {
    name: Ident,
    ty: Type,
}

impl Parse for Field {
    /// Parses `name: Type`.
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(Field { name, ty })
    }
}

/// Full input of the `project!` macro.
///
/// Expected form:
/// ```ignore
/// ModelName {
///     id: i64,
///     title: String,
///     ...
/// }
/// ProjectionName: only(id, title)
/// // or
/// ProjectionName: except(password, deleted_at)
/// ```
#[allow(unused)]
pub struct ProjectInput {
    /// Original model name (currently only used for better error messages).
    model_name: Ident,
    fields: Vec<Field>,
    /// Name of the struct that will be generated.
    name: Ident,
    /// `true` = only mode, `false` = except mode.
    is_only: bool,
    /// Columns listed in `only(...)` or `except(...)`.
    listed: Vec<Ident>,
}

impl Parse for ProjectInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Model name (kept for diagnostics)
        let model_name: Ident = input.parse()?;

        // { field: Type, ... }
        let content;
        braced!(content in input);
        let fields = Punctuated::<Field, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect::<Vec<_>>();

        // ProjectionName: only(...) | except(...)
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let mode: Ident = input.parse()?;
        let is_only = match mode.to_string().as_str() {
            "only" => true,
            "except" => false,
            other => {
                return Err(Error::new(
                    mode.span(),
                    format!("expected `only` or `except`, found `{other}`"),
                ));
            }
        };

        let args;
        parenthesized!(args in input);
        let listed = Punctuated::<Ident, Token![,]>::parse_terminated(&args)?
            .into_iter()
            .collect::<Vec<_>>();

        // Reject any trailing tokens (e.g. `only(id, name) garbage`)
        if !input.is_empty() {
            return Err(input.error("unexpected tokens after projection rule"));
        }

        Ok(ProjectInput {
            model_name,
            fields,
            name,
            is_only,
            listed,
        })
    }
}

/// Core expansion logic. Separated so it can be tested more easily.
pub fn expand(input: ProjectInput) -> Result<TokenStream> {
    let ProjectInput {
        model_name: _,
        fields,
        name,
        is_only,
        listed,
    } = input;

    // 1. Reject unknown columns
    for col in &listed {
        if !fields.iter().any(|f| f.name == *col) {
            return Err(Error::new(
                col.span(),
                format!("unknown column `{col}` in projection `{name}`"),
            ));
        }
    }

    // 2. Reject duplicates inside only(...) / except(...)
    let mut seen = std::collections::HashSet::new();
    for col in &listed {
        if !seen.insert(col.to_string()) {
            return Err(Error::new(
                col.span(),
                format!("duplicate column `{col}` in projection `{name}`"),
            ));
        }
    }

    // 3. Select the fields that should appear in the projection
    let kept: Vec<&Field> = fields
        .iter()
        .filter(|f| {
            let in_list = listed.iter().any(|c| c == &f.name);
            // only → keep if present, except → keep if absent
            in_list == is_only
        })
        .collect();

    if kept.is_empty() {
        return Err(Error::new(
            name.span(),
            format!("projection `{name}` would contain no fields"),
        ));
    }

    let field_names = kept.iter().map(|f| &f.name);
    let field_types = kept.iter().map(|f| &f.ty);
    let columns = kept
        .iter()
        .map(|f| f.name.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let expanded = quote! {
        #[derive(Clone, ::serde::Serialize, ::serde::Deserialize, ::sqlx::FromRow)]
        #[allow(dead_code)]
        pub struct #name {
            #( pub #field_names: #field_types, )*
        }

        impl #name {
            /// Comma-separated column list, always in sync with the struct fields.
            /// Useful for building `SELECT` clauses.
            pub const COLUMNS: &'static str = #columns;
        }
    };

    Ok(expanded.into())
}
