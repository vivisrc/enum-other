//! This crate provides an attribute macro that turns enums with discriminants
//! into an enum with an "other" variant that can act as a fallback value when
//! implementing extensible data protocols.
//!
//! # Usage
//!
//! When applied to an enum like so:
//!
//! ```
//! #[enum_other::other(u16)]
//! enum DnsRecordType {
//!     A = 1,
//!     Ns = 2,
//!     Cname = 5,
//!     Soa = 6,
//!     Ptr = 12,
//!     Mx = 15,
//!     Txt = 16,
//!     Aaaa = 28,
//!     Srv = 33,
//! }
//! ```
//!
//! Will translate to the following snippet:
//!
//! ```
//! enum DnsRecordType {
//!     A,
//!     Ns,
//!     Cname,
//!     Soa,
//!     Ptr,
//!     Mx,
//!     Txt,
//!     Aaaa,
//!     Srv,
//!     Other(u16),
//! }
//!
//! impl From<DnsRecordType> for u16 {
//!     fn from(value: DnsRecordType) -> Self {
//!         match value {
//!             DnsRecordType::A => 1,
//!             DnsRecordType::Ns => 2,
//!             DnsRecordType::Cname => 5,
//!             DnsRecordType::Soa => 6,
//!             DnsRecordType::Ptr => 12,
//!             DnsRecordType::Mx => 15,
//!             DnsRecordType::Txt => 16,
//!             DnsRecordType::Aaaa => 28,
//!             DnsRecordType::Srv => 33,
//!             DnsRecordType::Other(value) => value,
//!         }
//!     }
//! }
//!
//! impl From<u16> for DnsRecordType {
//!     fn from(value: u16) -> Self {
//!         match value {
//!             1 => Self::A,
//!             2 => Self::Ns,
//!             5 => Self::Cname,
//!             6 => Self::Soa,
//!             12 => Self::Ptr,
//!             15 => Self::Mx,
//!             16 => Self::Txt,
//!             28 => Self::Aaaa,
//!             33 => Self::Srv,
//!             _ => Self::Other(value),
//!         }
//!     }
//! }
//! ```
//!
//! As it generates match statements, the discriminants must both be valid
//! expressions and patterns.
//!
//! There exist special rules for tuple types, which have their contents
//! flattened in the "other" value.
//!
//! When the discriminants are string literals, the macro will automatically
//! add calls to to_string and as_str where neccesary to allow for string types
//! to be used.

use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Expr, ExprLit, ExprUnary, Ident, ItemEnum, Lit, LitInt, Token, Type, TypeTuple, UnOp,
};

struct Args {
    data_type: Type,
    other_ident: Ident,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let data_type: Type = input.parse()?;
        let comma: Option<Token![,]> = input.parse()?;
        let other_ident: Option<Ident> = comma.and_then(|_| input.parse().ok());

        Ok(Self {
            data_type,
            other_ident: other_ident.unwrap_or_else(|| parse_quote! { Other }),
        })
    }
}

fn parse_int_expr(expr: &Expr) -> Result<Option<isize>> {
    let mut expr = expr;
    let mut negative = false;

    if let Expr::Unary(ExprUnary {
        op: UnOp::Neg(_),
        expr: expression,
        ..
    }) = expr
    {
        negative = true;
        expr = expression;
    }

    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(int), ..
        }) => Ok(Some(
            int.base10_parse::<isize>()? * if negative { -1 } else { 1 },
        )),
        _ => Ok(None),
    }
}

/// Turn an enum with discriminants into an enum with an "other" value as a
/// fallback.
///
/// Automatically generates implementations for `From<Enum>` for `Type` and
/// `From<Type>` for `Enum`.
///
/// As it generates match statements to implement `From`, the provided
/// discriminants must be both valid expressions and patterns.
///
/// # Examples
///
/// ```
/// #[enum_other::other(u16)]
/// #[derive(Debug, PartialEq, Eq)]
/// pub enum HttpStatusCode {
///     Ok = 200,
///     Created = 201,
///     NoContent = 204,
///     MovedPermanently = 301,
///     Found = 302,
///     NotModified = 304,
///     BadRequest = 400,
///     Unauthorized = 401,
///     Forbidden = 403,
///     NotFound = 404,
///     Conflict = 409,
///     Gone = 410,
///     TooManyRequests = 429,
///     InternalServerError = 500,
///     BadGateway = 502,
///     ServiceUnavailable = 503,
///     GatewayTimeout = 504,
/// }
///
/// assert_eq!(u16::from(HttpStatusCode::MovedPermanently), 301);
/// assert_eq!(HttpStatusCode::from(201), HttpStatusCode::Created);
///
/// assert_eq!(u16::from(HttpStatusCode::Other(101)), 101);
/// assert_eq!(HttpStatusCode::from(418), HttpStatusCode::Other(418));
/// ```
///
/// Can optionally take a different identifier to replace `Other`:
///
/// ```
/// #[enum_other::other(u16, Unknown)]
/// #[derive(Debug, PartialEq, Eq)]
/// pub enum Radix {
///     Binary = 2,
///     Seximal = 6,
///     Octal = 8,
///     Decimal = 10,
///     Dozenal = 12,
///     Hexadecimal = 16,
/// }
///
/// assert_eq!(Radix::from(3), Radix::Unknown(3));
/// assert_eq!(u16::from(Radix::Hexadecimal), 16);
/// ```
///
/// Automatic discriminant values are also available for types that fit in an
/// `isize`:
///
/// ```
/// #[enum_other::other(u8)]
/// #[derive(Debug, PartialEq, Eq)]
/// pub enum Dimension {
///     Point = 0,
///     Line, // = 1
///     Surface, // = 2
///     Solid, // = 3
/// }
///
/// assert_eq!(Dimension::from(2), Dimension::Surface);
/// assert_eq!(u8::from(Dimension::Point), 0);
/// ```
#[proc_macro_attribute]
pub fn other(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemEnum);
    let Args {
        data_type,
        other_ident,
    } = parse_macro_input!(args as Args);

    let mut discriminants = Vec::with_capacity(item.variants.len());
    let mut curr_discriminant = 0isize;
    for mut variant in &mut item.variants {
        discriminants.push(match &variant.discriminant {
            Some((_, expr)) => {
                match parse_int_expr(expr) {
                    Ok(Some(int)) => curr_discriminant = int,
                    Ok(None) => (),
                    Err(e) => return TokenStream::from(e.to_compile_error()),
                }
                expr.clone()
            }
            None => ExprLit {
                attrs: Vec::new(),
                lit: LitInt::new(&curr_discriminant.to_string(), Span::call_site().into()).into(),
            }
            .into(),
        });
        variant.discriminant = None;
        curr_discriminant += 1;
    }

    let mut other_fields = Punctuated::new();
    match &data_type {
        Type::Tuple(TypeTuple { elems, .. }) => other_fields = elems.clone(),
        _ => other_fields.push_value(data_type.clone()),
    };
    item.variants
        .push(parse_quote! { #other_ident(#other_fields) });

    let other_fields_pattern = (0..other_fields.len())
        .map(|i| format_ident!("_{}", i))
        .collect::<Vec<Ident>>();

    let primary_variants = item
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .filter(|ident| *ident != other_ident)
        .collect::<Vec<Ident>>();

    let enum_ident = item.ident.clone();

    let data_type_match = discriminants
        .first()
        .map(|discriminant| match discriminant {
            Expr::Lit(ExprLit {
                lit: Lit::Str(_), ..
            }) => quote! { ::std::string::String::as_str(&value) },
            _ => quote! { value },
        });

    let convert_discriminant = match discriminants.first() {
        Some(Expr::Lit(ExprLit {
            lit: Lit::Str(_), ..
        })) => quote! { ::std::string::ToString::to_string },
        _ => quote! {},
    };

    let stream = TokenStream::from(quote! {
        #item

        impl ::core::convert::From<#enum_ident> for #data_type {
            fn from(value: #enum_ident) -> Self {
                match value {
                    #(
                        #enum_ident::#primary_variants => #convert_discriminant(#discriminants),
                    )*
                    #enum_ident :: #other_ident(
                        #(
                            #other_fields_pattern
                        ),*
                    ) => (
                        #(
                            #other_fields_pattern
                        ),*
                    ),
                }
            }
        }

        impl ::core::convert::From<#data_type> for #enum_ident {
            fn from(value: #data_type) -> Self {
                match #data_type_match {
                    #(
                        #discriminants => Self::#primary_variants,
                    )*
                    (
                        #(
                            #other_fields_pattern
                        ),*
                    ) => Self::#other_ident(
                        #(
                            #convert_discriminant(
                                #other_fields_pattern
                            )
                        ),*
                    ),
                }
            }
        }
    });

    stream
}
