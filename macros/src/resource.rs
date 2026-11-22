//! Procedural macro implementation for `resource!` and `resource_response!`.

use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::{
    Expr, GenericArgument, Ident, PathArguments, Token, Type, TypePath, Visibility, braced,
    ext::IdentExt, parse::Parse,
};

// ===========================================================================
// resource! parsing & expansion
// ===========================================================================

/// Parsed form of one `resource! { ... }` invocation.
pub struct ResourceDef {
    vis: Visibility,
    name: Ident,
    source: Type,
    fields: Vec<FieldDef>,
}

/// One output field of a resource.
struct FieldDef {
    /// Output field name in JSON
    name: Ident,
    /// Optional type annotation for nested resources (e.g. `Vec<CategoryResource>`)
    ty: Option<Type>,
    /// Expression that produces the value. `None` means "same name as field".
    expr: Option<Expr>,
}

impl Parse for ResourceDef {
    /// Parses `[vis] Name from SourceType { field, ... }`.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Optional visibility, then the resource name.
        let vis: Visibility = input.parse()?;
        let name: Ident = input.parse()?;

        // `from` is a contextual word, so parse it as an identifier and validate it.
        let from_kw: Ident = input.parse()?;
        if from_kw != "from" {
            return Err(syn::Error::new(from_kw.span(), "expected keyword `from`"));
        }

        // The source type to project from (a tuple for composite sources).
        let source: Type = input.parse()?;

        // The field list lives inside braces.
        let content;
        braced!(content in input);

        let mut fields = Vec::new();
        let mut seen = HashSet::new();
        while !content.is_empty() {
            let field = content.parse::<FieldDef>()?;

            // A repeated name would silently produce a JSON object with duplicate keys.
            if !seen.insert(field.name.unraw().to_string()) {
                return Err(syn::Error::new(
                    field.name.span(),
                    format!("duplicate field `{}`", field.name.unraw()),
                ));
            }
            fields.push(field);

            // Fields are separated by commas; the trailing comma is optional.
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else if !content.is_empty() {
                return Err(content.error("expected `,` or end of fields"));
            }
        }

        Ok(ResourceDef {
            vis,
            name,
            source,
            fields,
        })
    }
}

impl Parse for FieldDef {
    /// Parses `name [: Type] [= expr]`.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Possible forms:
        //   id
        //   name = this.title
        //   created_at = to_jalali(&this.created_at)
        //   categories: Vec<CategoryResource> = this.categories
        //   category: Option<CategoryResource> = this.category

        let name: Ident = input.parse()?;

        // Optional type annotation, only used to detect nested resources.
        let ty = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Some(input.parse::<Type>()?)
        } else {
            None
        };

        // Optional value expression, defaults to `this.<name>` during expansion.
        let expr = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse::<Expr>()?)
        } else {
            None
        };

        Ok(FieldDef { name, ty, expr })
    }
}

// ---------------------------------------------------------------------------
// Nested type detection
// ---------------------------------------------------------------------------

/// How a field annotation maps to a nested resource.
enum NestedKind {
    /// Vec<OtherResource>
    Vec(Type),
    /// Option<OtherResource>
    Option(Type),
    /// Not a recognized nested form
    Plain,
}

/// Classifies a field type annotation as `Vec<X>`, `Option<X>` or plain.
fn detect_nested(ty: &Type) -> NestedKind {
    // Only simple path types such as `Vec<X>` can be nested resources.
    let Type::Path(TypePath { path, .. }) = ty else {
        return NestedKind::Plain;
    };

    // Look at the last segment so `std::vec::Vec<X>` works too.
    let Some(segment) = path.segments.last() else {
        return NestedKind::Plain;
    };

    // Nested forms always carry generic arguments.
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return NestedKind::Plain;
    };

    // The first generic argument is the other resource type.
    let Some(GenericArgument::Type(inner)) = args.args.first() else {
        return NestedKind::Plain;
    };

    match segment.ident.to_string().as_str() {
        "Vec" => NestedKind::Vec(inner.clone()),
        "Option" => NestedKind::Option(inner.clone()),
        _ => NestedKind::Plain,
    }
}

// ---------------------------------------------------------------------------
// Expansion of resource!
// ---------------------------------------------------------------------------

impl ResourceDef {
    /// Generates the resource struct, its constructors and the `ResourceSpec` impl.
    pub fn expand(self) -> proc_macro2::TokenStream {
        let ResourceDef {
            vis,
            name,
            source,
            fields,
        } = self;

        let field_count = fields.len();
        let name_str = name.to_string();

        // Generate one `serialize_field` call per field.
        let serialize_fields = fields.iter().map(|f| {
            let field_name = &f.name;
            // `unraw` turns `r#type` into the JSON key `type`.
            let field_name_str = field_name.unraw().to_string();

            // Base expression that produces the raw value.
            let raw_expr = match &f.expr {
                Some(expr) => quote! { #expr },
                None => quote! { this.#field_name },
            };

            // Wrap it if it is a nested resource relation.
            // The expression is parenthesized so that `&` and `.as_ref()` apply to the
            // whole expression, e.g. `this.price * this.quantity` would otherwise
            // parse as `(&this.price) * this.quantity`.
            let kind = f.ty.as_ref().map_or(NestedKind::Plain, detect_nested);
            let value_expr = match kind {
                NestedKind::Vec(res_ty) => quote! {
                    &macros_core::resource::NestedCollection::<#res_ty> {
                        items: &(#raw_expr),
                    }
                },
                NestedKind::Option(res_ty) => quote! {
                    &macros_core::resource::NestedOption::<#res_ty> {
                        item: (#raw_expr).as_ref(),
                    }
                },
                NestedKind::Plain => quote! { &(#raw_expr) },
            };

            quote! {
                state.serialize_field(#field_name_str, #value_expr)?;
            }
        });

        quote! {
            /// Serialization projection generated by `resource!`.
            ///
            /// This type exists only as a namespace for the `from` and `collection`
            /// constructors. It does not hold data itself.
            #vis struct #name;

            impl #name {
                /// Create a resource from an owned source value.
                ///
                /// The source is moved into the resource and will be serialized
                /// according to the mapping defined in the `resource!` macro.
                #[inline]
                pub fn from(source: #source) -> macros_core::resource::Resource<#name> {
                    macros_core::resource::Resource::new(source)
                }

                /// Create a collection of resources from a vector of source values.
                ///
                /// The items are moved into the collection. Serialization is performed
                /// without allocating intermediate Resource wrappers.
                #[inline]
                pub fn collection(items: Vec<#source>) -> macros_core::resource::ResourceCollection<#name> {
                    macros_core::resource::ResourceCollection::new(items)
                }
            }

            #[doc(hidden)]
            #[allow(unused_parens)]
            impl macros_core::resource::ResourceSpec for #name {
                type Source = #source;

                fn serialize_source<S>(
                    this: &Self::Source,
                    serializer: S,
                ) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    use ::serde::ser::SerializeStruct;

                    let mut state = serializer.serialize_struct(#name_str, #field_count)?;
                    #(#serialize_fields)*
                    state.end()
                }
            }
        }
    }
}

// ===========================================================================
// resource_response! parsing & expansion
// ===========================================================================

/// Parsed form of one `resource_response! { ... }` invocation.
pub struct ResponseDef {
    fields: Vec<ResponseField>,
}

/// One `name: expr` entry of a response.
struct ResponseField {
    name: Ident,
    value: Expr,
}

impl Parse for ResponseDef {
    /// Parses a comma separated list of `name: expr` entries.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();

        while !input.is_empty() {
            // Each entry is `name: expr`.
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;

            fields.push(ResponseField { name, value });

            // Entries are separated by commas; the trailing comma is optional.
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else if !input.is_empty() {
                return Err(input.error("expected comma or end of input"));
            }
        }

        // An empty response is almost certainly a mistake.
        if fields.is_empty() {
            return Err(input.error("resource_response! requires at least one field"));
        }

        Ok(ResponseDef { fields })
    }
}

impl ResponseDef {
    /// Generates a local struct that serializes every entry as a JSON object field.
    pub fn expand(self) -> proc_macro2::TokenStream {
        let field_names: Vec<_> = self.fields.iter().map(|f| &f.name).collect();
        let field_values: Vec<_> = self.fields.iter().map(|f| &f.value).collect();
        let field_count = self.fields.len();

        // JSON keys; `unraw` turns `r#type` into `type`.
        let field_keys: Vec<String> = field_names.iter().map(|n| n.unraw().to_string()).collect();

        // Generic parameters get synthetic CamelCase names (`__T0`, `__T1`, ...)
        // instead of the field names, which avoids `non_camel_case_types` warnings.
        let generics: Vec<Ident> = (0..field_count)
            .map(|i| format_ident!("__T{}", i))
            .collect();

        // The struct is defined inside a block, so repeated or nested uses do not clash.
        let struct_name = format_ident!("__ResourceResponse");

        quote! {
            {
                struct #struct_name<#(#generics),*> {
                    #(#field_names: #generics,)*
                }

                impl<#(#generics: ::serde::Serialize),*> ::serde::Serialize for #struct_name<#(#generics),*> {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::Serializer,
                    {
                        use ::serde::ser::SerializeStruct;
                        let mut state = serializer.serialize_struct("ResourceResponse", #field_count)?;
                        #(state.serialize_field(#field_keys, &self.#field_names)?;)*
                        state.end()
                    }
                }

                #struct_name {
                    #(#field_names: #field_values,)*
                }
            }
        }
    }
}
