//! Contains the implementation for the discriminant proc macro.
#[cfg(test)]
mod tests;

use num_traits::PrimInt;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Expr, Ident, Variant, parse_quote};

/// Sets up the discriminant for an enum. This includes setting the `repr`, creating a fieldless
/// version of the enum (even if it's already fieldless for consistency), and implementing
/// Discriminantable.
///
/// * `repr`: The repr for the enum.
/// * `item`: The enum.
///
/// # Panics
/// This macro will fail if
/// - An invalid discriminant is used
/// - `item` cannot be parsed or is invalid in some way (e.g., too many variants for the repr type)
/// - It is applied any anything besides an enum
#[expect(clippy::needless_pass_by_value, reason = "We want to consume it anyway")]
pub fn discriminant_impl(repr: TokenStream, item: TokenStream) -> TokenStream {
    #[expect(clippy::expect_used)]
    let item: DeriveInput = syn::parse2(item).expect("Unable to parse tokens");
    let variants: Vec<Variant> = match &item.data {
        Data::Enum(e) => e.variants.clone().into_iter().collect(),
        _ => panic!("Only enums are supported"),
    };

    let discriminants = match repr.to_string().as_str() {
        "u8" => extract_enum_discriminants::<u8>(&variants),
        "u16" => extract_enum_discriminants::<u16>(&variants),
        "u32" => extract_enum_discriminants::<u32>(&variants),
        "u64" => extract_enum_discriminants::<u64>(&variants),
        "u128" => extract_enum_discriminants::<u128>(&variants),
        "i8" => extract_enum_discriminants::<i8>(&variants),
        "i16" => extract_enum_discriminants::<i16>(&variants),
        "i32" => extract_enum_discriminants::<i32>(&variants),
        "i64" => extract_enum_discriminants::<i64>(&variants),
        "i128" => extract_enum_discriminants::<i128>(&variants),
        "usize" => {
            eprintln!("The {repr} discriminant type varies in size between architectures. It is recommended to use fixed-length data types.");
            extract_enum_discriminants::<usize>(&variants)
        }
        "isize" => {
            eprintln!("The {repr} discriminant type varies in size between architectures. It is recommended to use fixed-length data types.");
            extract_enum_discriminants::<isize>(&variants)
        }
        _ => panic!(
            "Unsupported discriminant type `{repr}` for `{}`. Must be a primitive integer type. See <https://doc.rust-lang.org/reference/type-layout.html#primitive-representations>",
            item.ident
        ),
    };

    let item_ident = &item.ident;
    let visibility = &item.vis;
    let fieldless_ident = Ident::new(&format!("{item_ident}_"), Span::call_site());
    let fieldless_variants = generate_fieldless_variants(&discriminants, item_ident);
    let fieldless_try_from_variants = generate_fieldless_try_from_variants(&discriminants);
    let fieldless_from_match_arms = generate_original_to_fieldless_match_arms(&discriminants, item_ident);

    let fieldless_doc = format!("Fieldless representations of [{item_ident}]. Used to extract discriminants without fully constructing the enum.");
    quote! {
        #[repr(#repr)]
        #item

        impl #item_ident {
            /// Returns the discriminant of [Self].
            pub const fn discriminant(&self) -> #repr {
                unsafe { *core::ptr::from_ref::<Self>(self).cast::<#repr>() }
            }
        }

        impl discrimin_ant::Discriminantable for #item_ident {
            type Discriminant = #repr;

            fn discriminant(&self) -> Self::Discriminant {
                self.discriminant()
            }
        }

        #[doc = #fieldless_doc]
        #[repr(#repr)]
        #visibility enum #fieldless_ident {
            #fieldless_variants
        }

        impl #fieldless_ident {
            /// Returns the discriminant of [Self].
            pub const fn discriminant(&self) -> #repr {
                unsafe { *core::ptr::from_ref::<Self>(self).cast::<#repr>() }
            }
        }

        impl discrimin_ant::Discriminantable for #fieldless_ident {
            type Discriminant = #repr;

            fn discriminant(&self) -> Self::Discriminant {
                self.discriminant()
            }
        }

        impl TryFrom<#repr> for #fieldless_ident {
            type Error = ();

            fn try_from(value: #repr) -> Result<Self, Self::Error> {
                #fieldless_try_from_variants
                Err(())
            }
        }

        impl From<&#item_ident> for #fieldless_ident {
            fn from(value: &#item_ident) -> Self {
                match value {
                   #fieldless_from_match_arms
                }
            }
        }
    }
}

/// Extracts and calculates the discriminants for the enum.
///
/// * `variants`: The variants of the enum.
#[expect(clippy::expect_used)]
fn extract_enum_discriminants<T: PrimInt + ToTokens>(variants: &[Variant]) -> Vec<(Variant, Expr)> {
    let mut disciminant_offset = T::one();
    let mut current_discriminant_expr: Expr = parse_quote!(#disciminant_offset);
    variants
        .iter()
        .map(|variant| {
            let discriminant_expr = if let Some(d) = &variant.discriminant {
                disciminant_offset = T::one();
                current_discriminant_expr = d.1.clone();
                current_discriminant_expr.clone()
            } else {
                let expr = parse_quote!(#current_discriminant_expr + #disciminant_offset);
                disciminant_offset = disciminant_offset.checked_add(&T::one()).expect("Too many variants!");
                expr
            };
            (variant.clone(), discriminant_expr)
        })
        .collect()
}

/// Generates code to transform the repr into the the fieldless enum. For use with `TryFrom`.
///
/// * `discriminants`: The discriminants of the original enum.
fn generate_fieldless_try_from_variants(discriminants: &[(Variant, Expr)]) -> TokenStream {
    let mut try_from_variants = quote! {};
    for (variant, expr) in discriminants {
        let ident = variant.ident.clone();
        try_from_variants = quote! {
            #try_from_variants
            if value == (#expr) {
                return Ok(Self::#ident);
            }
        };
    }
    try_from_variants
}

/// Generates the fieldless variants of the original enum. Shall have the same variants and
/// discriminants, just with all variants as units.
///
/// * `discriminants`: The discriminants of the original enum.
/// * `item_ident`: The name of the original enum.
fn generate_fieldless_variants(discriminants: &[(Variant, Expr)], item_ident: &syn::Ident) -> TokenStream {
    let mut fieldless_variants = quote! {};
    for (variant, expr) in discriminants {
        let ident = variant.ident.clone();
        let doc = format!("A fieldless version of [{item_ident}::{ident}], used to extract the variant's discriminant without needing to fully construct it.");
        fieldless_variants = quote! {
            #fieldless_variants
            #[doc = #doc]
            #ident = #expr,
        };
    }
    fieldless_variants
}

/// Generates the match arms to map the original enum to the fieldless enum.
///
/// * `discriminants`: The discriminants for the original enum.
/// * `enum_name`: The name of the original enum.
fn generate_original_to_fieldless_match_arms(discriminants: &[(Variant, Expr)], enum_name: &Ident) -> TokenStream {
    if discriminants.is_empty() {
        return quote! {
            _ => unreachable!("Only reachable with a zero-variant enum")
        };
    }

    let mut match_arms = quote! {};
    for (variant, _) in discriminants {
        let ident = variant.ident.clone();
        match_arms = match &variant.fields {
            syn::Fields::Named(_) => quote! {
                #match_arms
                #enum_name::#ident{ .. } => Self::#ident,
            },
            syn::Fields::Unnamed(_) => quote! {
                #match_arms
                #enum_name::#ident(..) => Self::#ident,
            },
            syn::Fields::Unit => quote! {
                #match_arms
                #enum_name::#ident => Self::#ident,
            },
        };
    }
    match_arms
}
