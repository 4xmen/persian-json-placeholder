//! Derive macro powering the `pawn` crate.
//!
//! This crate only exports the [`Pawn`] derive; the runtime trait and the
//! fluent builders live in the `factory` crate, which re-exports this macro.

use macros_core::build_config;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Expr, ExprLit, Fields, Ident, Lit, LitInt, Path, Token};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

/// Expands the derive input into a `Pawn` implementation.
pub fn pawn_expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => {
                return Err(syn::Error::new(
                    input.ident.span(),
                    "Factory can only be derived for structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "Factory can only be derived for structs",
            ));
        }
    };

    let mut uses_fake = false;
    let mut inits = Vec::with_capacity(fields.len());
    for field in fields {
        let ident = field.ident.as_ref().expect("named field has an ident");
        let init = field_init(field, &mut uses_fake)?;
        inits.push(quote! { #ident: #init });
    }

    // Only bring `Fake` into scope when a `#[fake(...)]` field actually needs it,
    // so structs that never use it don't require the `fake` crate at all.
    let fake_import = uses_fake.then(|| quote! { use ::fake::Fake; });

    Ok(quote! {
        impl #impl_generics macros_core::factory::Pawn for #name #ty_generics #where_clause {
            fn definition() -> Self {
                #fake_import
                Self {
                    #(#inits),*
                }
            }
        }
    })
}

/// Builds the initializer expression for a single field from its attribute.
fn field_init(field: &syn::Field, uses_fake: &mut bool) -> syn::Result<TokenStream2> {
    let mut init: Option<TokenStream2> = None;

    for attr in &field.attrs {
        let path = attr.path();
        let is_factory_attr =
            path.is_ident("fake") || path.is_ident("generator") || path.is_ident("value");
        if !is_factory_attr {
            continue;
        }

        if init.is_some() {
            return Err(syn::Error::new(
                attr.span(),
                "a field may only use one of #[fake], #[generator], or #[value]",
            ));
        }

        init = Some(if path.is_ident("fake") {
            *uses_fake = true;
            fake_init(attr)?
        } else if path.is_ident("generator") {
            generator_init(attr)?
        } else {
            value_init(attr)?
        });
    }

    // No recognized attribute: fall back to `Default`.
    Ok(init.unwrap_or_else(|| quote! { ::core::default::Default::default() }))
}

/// Maps a `#[fake(...)]` attribute to a `fake`-crate expression that yields a `String`.
fn fake_init(attr: &syn::Attribute) -> syn::Result<TokenStream2> {
    let spec: FakeSpec = attr.parse_args()?;
    let locale_ident = syn::Ident::new(build_config::FAKER_LOCALE, Span::call_site());

    let expr = match spec.kind.to_string().as_str() {
        "name" => {
            spec.expect_no_args()?;
            quote! { ::fake::faker::name::#locale_ident::Name().fake::<String>() }
        }
        "username" => {
            spec.expect_no_args()?;
            quote! { ::fake::faker::internet::#locale_ident::Username().fake::<String>() }
        }
        "email" => {
            spec.expect_no_args()?;
            quote! { ::fake::faker::internet::#locale_ident::SafeEmail().fake::<String>() }
        }
        "password" => {
            let len = spec.required_int("length")?;
            quote! { ::fake::faker::internet::#locale_ident::Password((#len)..(#len + 1)).fake::<String>() }
        }
        "lorem" => {
            let words = spec.required_int("words")?;
            quote! { ::fake::faker::lorem::#locale_ident::Sentence((#words)..(#words + 1)).fake::<String>() }
        }

        // Generate a random date.
        "date" => {
            spec.expect_no_args()?;
            quote! {
                ::fake::faker::time::#locale_ident::Date().fake::<String>()
            }
        }

        // Generate a random time.
        "time" => {
            spec.expect_no_args()?;
            quote! {
                ::fake::faker::time::#locale_ident::Time().fake::<String>()
            }
        }

        // Generate a random datetime.
        "datetime" => {
            spec.expect_no_args()?;
            quote! {
                ::fake::faker::time::#locale_ident::DateTime().fake::<String>()
            }
        }
        "beforeDatetime" => {
            let args = spec.named_args(&["before"])?;
            let dt = datetime_tokens(datetime_arg(args[0])?);
            db_datetime(quote! { ::fake::faker::time::#locale_ident::DateTimeBefore(#dt) })
        }

        "afterDatetime" => {
            let args = spec.named_args(&["after"])?;
            let dt = datetime_tokens(datetime_arg(args[0])?);
            db_datetime(quote! { ::fake::faker::time::#locale_ident::DateTimeAfter(#dt) })
        }

        "betweenDatetime" => {
            let args = spec.named_args(&["start", "end"])?;
            let start = datetime_arg(args[0])?;
            let end = datetime_arg(args[1])?;
            if start >= end {
                return Err(syn::Error::new(
                    args[0].key.span(),
                    "`start` must be earlier than `end`",
                ));
            }
            let (s, e) = (datetime_tokens(start), datetime_tokens(end));
            db_datetime(quote! { ::fake::faker::time::#locale_ident::DateTimeBetween(#s, #e) })
        }
        other => {
            return Err(syn::Error::new(
                spec.kind.span(),
                format!(
                    "unknown fake generator `{other}`; \
                 expected name, username, email, password, lorem, \
                 date, time, datetime, beforeDatetime, afterDatetime, \
                 or betweenDatetime"
                ),
            ));
        }
    };
    Ok(expr)
}

/// Maps `#[generator(path)]` to a call of the named zero-argument function.
fn generator_init(attr: &syn::Attribute) -> syn::Result<TokenStream2> {
    let path: Path = attr.parse_args()?;
    Ok(quote! { #path() })
}

/// Maps `#[value(expr)]` to an initializer.
///
/// A string literal is wrapped in `.into()` so it can populate `String` (or any
/// `From<&str>`) fields directly; every other expression is emitted verbatim.
fn value_init(attr: &syn::Attribute) -> syn::Result<TokenStream2> {
    let expr: Expr = attr.parse_args()?;
    if matches!(
        &expr,
        Expr::Lit(ExprLit {
            lit: Lit::Str(_),
            ..
        })
    ) {
        Ok(quote! { (#expr).into() })
    } else {
        Ok(quote! { #expr })
    }
}

/// Parsed contents of a `#[fake(...)]` attribute, e.g. the `password(length = 8)` part.
struct FakeSpec {
    kind: Ident,
    args: Punctuated<FakeArg, Token![,]>,
}

/// A single `key = literal` argument inside a fake spec.
struct FakeArg {
    key: Ident,
    value: Lit,
}

impl Parse for FakeSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kind: Ident = input.parse()?;
        let args = if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            Punctuated::<FakeArg, Token![,]>::parse_terminated(&content)?
        } else {
            Punctuated::new()
        };
        Ok(Self { kind, args })
    }
}

impl Parse for FakeArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: Lit = input.parse()?;
        Ok(Self { key, value })
    }
}

impl FakeSpec {
    /// Returns args in the order of `keys`, regardless of the order the user wrote them.
    /// Errors on missing, duplicate, or unknown keys.
    fn named_args(&self, keys: &[&str]) -> syn::Result<Vec<&FakeArg>> {
        for arg in &self.args {
            if !keys.iter().any(|k| arg.key == k) {
                return Err(syn::Error::new(
                    arg.key.span(),
                    format!(
                        "`{}` does not accept `{}`; expected: {}",
                        self.kind,
                        arg.key,
                        keys.join(", ")
                    ),
                ));
            }
        }

        keys.iter()
            .map(|key| {
                let mut found = self.args.iter().filter(|a| a.key == key);
                let first = found.next().ok_or_else(|| {
                    syn::Error::new(
                        self.kind.span(),
                        format!("`{}` requires `{key} = \"<RFC 3339 datetime>\"`", self.kind),
                    )
                })?;
                if let Some(dup) = found.next() {
                    return Err(syn::Error::new(
                        dup.key.span(),
                        format!("duplicate argument `{key}`"),
                    ));
                }
                Ok(first)
            })
            .collect()
    }
    /// Errors if any argument was supplied to a generator that takes none.
    fn expect_no_args(&self) -> syn::Result<()> {
        if let Some(arg) = self.args.first() {
            return Err(syn::Error::new(
                arg.key.span(),
                format!("`{}` does not take arguments", self.kind),
            ));
        }
        Ok(())
    }

    /// Extracts a required integer argument by position.
    fn required_int(&self, key: &str) -> syn::Result<LitInt> {
        let mut iter = self.args.iter();
        let arg = iter.next().ok_or_else(|| {
            syn::Error::new(
                self.kind.span(),
                format!("`{}` requires `{key} = N`", self.kind),
            )
        })?;

        if let Some(extra) = iter.next() {
            return Err(syn::Error::new(
                extra.key.span(),
                format!("`{}` accepts only `{key} = N`", self.kind),
            ));
        }

        if arg.key != key {
            return Err(syn::Error::new(
                arg.key.span(),
                format!("expected `{key}`, found `{}`", arg.key),
            ));
        }

        match &arg.value {
            Lit::Int(int) => Ok(int.clone()),
            other => Err(syn::Error::new(
                other.span(),
                format!("`{key}` must be an integer"),
            )),
        }
    }
}

/// Validates an argument as an RFC 3339 string literal at compile time.
fn datetime_arg(arg: &FakeArg) -> syn::Result<OffsetDateTime> {
    match &arg.value {
        Lit::Str(s) => OffsetDateTime::parse(&s.value(), &Rfc3339).map_err(|e| {
            syn::Error::new(
                s.span(),
                format!(
                    "`{}` is not a valid RFC 3339 datetime ({e}); \
                     example: \"2025-01-01T00:00:00Z\"",
                    s.value()
                ),
            )
        }),
        other => Err(syn::Error::new(
            other.span(),
            format!("`{}` must be a string literal", arg.key),
        )),
    }
}

/// Emits a runtime `OffsetDateTime` built from an already-validated value
/// (no parsing at runtime).
fn datetime_tokens(dt: OffsetDateTime) -> TokenStream2 {
    let nanos = dt.unix_timestamp_nanos(); // i128 -> emitted as `123i128`
    quote! {
        ::time::OffsetDateTime::from_unix_timestamp_nanos(#nanos).unwrap()
    }
}

/// Wraps a `fake` datetime generator so the result is a DB-friendly
/// `YYYY-MM-DD HH:MM:SS` string (UTC, no `T`, no offset), accepted by
/// MySQL DATETIME/TIMESTAMP and also by PostgreSQL/SQLite.
fn db_datetime(faker: TokenStream2) -> TokenStream2 {
    quote! {
        {
            let __dt: ::time::OffsetDateTime =
                ::fake::Fake::fake::<::time::OffsetDateTime>(&#faker);
            let __dt = __dt.to_offset(::time::UtcOffset::UTC);
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                __dt.year(),
                u8::from(__dt.month()),
                __dt.day(),
                __dt.hour(),
                __dt.minute(),
                __dt.second(),
            )
        }
    }
}
