#[expect(dead_code)]
mod complex_u_enum;
#[expect(dead_code)]
mod simple_fieldless_enum;
#[expect(dead_code)]
mod simple_no_discriminant_enum;
#[expect(dead_code)]
mod simple_signed_fieldless_enum;

use super::*;

use pretty_assertions::assert_eq;

/// Converts a str representation of the expected output into the actual form.
///
/// * `expected_str`: The string to expect, typically read from a file.
fn str_to_expected(expected_str: &str) -> String {
    #[allow(clippy::unwrap_used)]
    let expected = expected_str.parse::<TokenStream>().unwrap();
    let expected = expected.to_string();
    expected.replace(":<", ": <").replace("<&", "< &")
}

#[test]
#[should_panic = "Unsupported discriminant type `transparent` for `SimpleEnum`. Must be a primitive integer type. See <https://doc.rust-lang.org/reference/type-layout.html#primitive-representations>"]
fn bad_repr_type() {
    let tokens = quote! { enum SimpleEnum{ A, B } };
    let attr = quote! { transparent };
    let _ = discriminant_impl(attr, tokens);
}

#[test]
fn fieldless_variants() {
    let tokens = quote! { enum SimpleEnum{ A, B } };
    let attr = quote! { u8 };
    let _ = discriminant_impl(attr, tokens).to_string();
}

#[test]
fn assigned_fieldless_variants() {
    let tokens = quote! {
        pub enum SimpleFieldlessEnum {
            One = 1,
            Two,
            Five = 5,
            Six,
        }
    };
    let attr = quote! { u8 };
    let result = discriminant_impl(attr, tokens);

    let expected = str_to_expected(include_str!("./tests/simple_fieldless_enum.rs"));
    assert_eq!(result.to_string(), expected);
}

#[test]
fn assigned_variants() {
    let tokens = quote! {
        pub enum ComplexUEnum {
            One(i32) = 1,
            Two(i32),
            Five { x: u32 } = 5,
            Six { x: u32 },
        }
    };
    let attr = quote! { u8 };
    let result = discriminant_impl(attr, tokens);

    let expected = str_to_expected(include_str!("./tests/complex_u_enum.rs"));
    assert_eq!(result.to_string(), expected);
}

#[test]
fn assigned_signed_fieldless_variants() {
    let tokens = quote! {
        pub enum SimpleSignedFieldlessEnum {
            NOne = -1,
            One = 1,
            Two,
            Five = 5,
            Six,
            Seven,
        }
    };
    let attr = quote! { i8 };
    let result = discriminant_impl(attr, tokens);

    let expected = str_to_expected(include_str!("./tests/simple_signed_fieldless_enum.rs"));
    assert_eq!(result.to_string(), expected);
}

#[test]
fn simple_no_discriminant_enum() {
    let tokens = quote! {
        pub enum SimpleNoDiscriminantEnum {
            Zero,
            One,
            Two,
            Three = 3,
            Four,
            Five,
        }
    };
    let attr = quote! { u16 };
    let result = discriminant_impl(attr, tokens);

    let expected = str_to_expected(include_str!("./tests/simple_no_discriminant_enum.rs"));
    assert_eq!(result.to_string(), expected);
}
