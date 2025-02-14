//! # Discrimin-Ant Proc
//!
//! Discrimin-Ant is a tiny library to make working with `enum` discriminants just a little easier.
//!
//! ---
//!
//! Discrimin-Ant was originally built to make it easier to marshal enums for use for
//! software systems that do not or cannot use the Rust ecosystem.
//! It is intended to be used with [Serde](https://crates.io/crates/serde) to
//! marshal and send enums over a network or save them to a file.
//! This can be done by implementing a custom `serde::Serialize` and
//! `serde::Deserialize` that treats an enum as a struct or tuple instead of a
//! variant, since the variant serialize methods only provide the index of the enum
//! instead of its discriminant. This allows enums to be serialized in a way such
//! that rearranging the order of enums does not change the discriminant serialized.
//!
//! The primary features of this crate are the `Discriminantable` trait and the
//! `discriminant` attribute.
//! The `discriminant` attribute is meant to take the place of the `repr`
//! attribute on enums, and will generate a few blocks of code:
//! - An `impl` for `Discriminantable` for the enum, using the enum's actual discriminants
//! - A fieldless version of the enum (regardless of whether or not the enum is already fieldless)
//! - An `impl` for `Discriminantable` on the fieldless version of the enum
//! - Other utilities on the fieldless version to ease use
//!
//! Specifically, given the enum
//! ```no_run
//! use discrimin_ant_proc::discriminant;
//! #[discriminant(u8)]
//! pub enum ComplexUEnum {
//!     One(i32) = 1,
//!     Two(i32),
//!     Five { x: u32 } = 5,
//!     Six { x: u32 },
//! }
//! ```
//! the result will be
//!
//! ```no_run
//! #[repr(u8)]
//! pub enum ComplexUEnum {
//!     One(i32) = 1,
//!     Two(i32),
//!     Five { x: u32 } = 5,
//!     Six { x: u32 },
//! }
//! impl ComplexUEnum {
//!     #[doc = r" Returns the discriminant of [Self]."]
//!     pub const fn discriminant(&self) -> u8 {
//!         unsafe { *core::ptr::from_ref::<Self>(self).cast::<u8>() }
//!     }
//! }
//! impl discrimin_ant::Discriminantable for ComplexUEnum {
//!     type Discriminant = u8;
//!
//!     fn discriminant(&self) -> Self::Discriminant {
//!         self.discriminant()
//!     }
//! }
//! #[doc = "Fieldless representations of [ComplexUEnum]. Used to extract discriminants without fully constructing the enum."]
//! #[repr(u8)]
//! pub enum ComplexUEnum_ {
//!     #[doc = "A fieldless version of [ComplexUEnum::One], used to extract the variant's discriminant without needing to fully construct it."]
//!     One = 1,
//!     #[doc = "A fieldless version of [ComplexUEnum::Two], used to extract the variant's discriminant without needing to fully construct it."]
//!     Two = 1 + 1u8,
//!     #[doc = "A fieldless version of [ComplexUEnum::Five], used to extract the variant's discriminant without needing to fully construct it."]
//!     Five = 5,
//!     #[doc = "A fieldless version of [ComplexUEnum::Six], used to extract the variant's discriminant without needing to fully construct it."]
//!     Six = 5 + 1u8,
//! }
//! impl ComplexUEnum_ {
//!     #[doc = r" Returns the discriminant of [Self]."]
//!     pub const fn discriminant(&self) -> u8 {
//!         unsafe { *core::ptr::from_ref::<Self>(self).cast::<u8>() }
//!     }
//! }
//! impl discrimin_ant::Discriminantable for ComplexUEnum_ {
//!     type Discriminant = u8;
//!
//!     fn discriminant(&self) -> Self::Discriminant {
//!         self.discriminant()
//!     }
//! }
//! impl TryFrom<u8> for ComplexUEnum_ {
//!     type Error = ();
//!     fn try_from(value: u8) -> Result<Self, Self::Error> {
//!         if value == (1) {
//!             return Ok(Self::One);
//!         }
//!         if value == (1 + 1u8) {
//!             return Ok(Self::Two);
//!         }
//!         if value == (5) {
//!             return Ok(Self::Five);
//!         }
//!         if value == (5 + 1u8) {
//!             return Ok(Self::Six);
//!         }
//!         Err(())
//!     }
//! }
//! impl From<&ComplexUEnum> for ComplexUEnum_ {
//!     fn from(value: &ComplexUEnum) -> Self {
//!         match value {
//!             ComplexUEnum::One(..) => Self::One,
//!             ComplexUEnum::Two(..) => Self::Two,
//!             ComplexUEnum::Five { .. } => Self::Five,
//!             ComplexUEnum::Six { .. } => Self::Six,
//!         }
//!     }
//! }
//! ```
//!
//! Note that the `discriminant` attribute only supports [primitive representations](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations)
//! since those are the only types that have a [reliably accessible discriminant](https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting).
//! Nevertheless, manual implementations of `Discriminant` can be made for any enum (and technically and other object).
use proc_macro::TokenStream;

mod discriminant;

/// Assign a `repr` and produce a `discriminant` method. Also produce a fieldless version of the
/// enum to allow for easier access to the discriminants without needing to construct one.
///
/// * `repr`: The representation for the enum. Should be one of the standard `u*` or `i*` data types.
///   See <https://doc.rust-lang.org/nomicon/other-reprs.html#repru-repri>
///   and <https://doc.rust-lang.org/reference/type-layout.html#primitive-representations>
///
/// # Example
/// Fieldless
/// ```rust
/// use discrimin_ant_proc::discriminant;
/// #[discriminant(i8)]
/// enum FieldlessEnum {
///     NOne = -1,
///     One = 1,
///     Two = 2,
///     Three,
///     Five = 5,
///     Seven = 7,
///     Nine = 9,
/// }
///
/// assert_eq!(FieldlessEnum::NOne.discriminant(), -1);
/// assert_eq!(FieldlessEnum::One.discriminant(), 1);
/// assert_eq!(FieldlessEnum::Two.discriminant(), 2);
/// assert_eq!(FieldlessEnum::Three.discriminant(), 3);
/// assert_eq!(FieldlessEnum::Five.discriminant(), 5);
/// assert_eq!(FieldlessEnum::Seven.discriminant(), 7);
/// assert_eq!(FieldlessEnum::Nine.discriminant(), 9);
/// // Still generates the Fieldless version for consistency
/// assert_eq!(FieldlessEnum_::NOne.discriminant(), -1);
/// assert_eq!(FieldlessEnum_::One.discriminant(), 1);
/// assert_eq!(FieldlessEnum_::Two.discriminant(), 2);
/// assert_eq!(FieldlessEnum_::Three.discriminant(), 3);
/// assert_eq!(FieldlessEnum_::Five.discriminant(), 5);
/// assert_eq!(FieldlessEnum_::Seven.discriminant(), 7);
/// assert_eq!(FieldlessEnum_::Nine.discriminant(), 9);
/// ```
///
/// With fields
/// ```rust
/// use discrimin_ant_proc::discriminant;
/// #[discriminant(i8)]
/// enum FieldEnum {
///     NOne(u8) = -1,
///     One { item: u8 } = 1,
///     Two = 2,
///     Three(u8, i16),
///     Five = 5,
///     Seven { item_a: i128, item_b: u32 } = 7,
///     Nine = 9,
/// }
///
/// assert_eq!(FieldEnum::NOne(0).discriminant(), -1);
/// assert_eq!(FieldEnum::One { item: 0 }.discriminant(), 1);
/// assert_eq!(FieldEnum::Two.discriminant(), 2);
/// assert_eq!(FieldEnum::Three(0, 0).discriminant(), 3);
/// assert_eq!(FieldEnum::Five.discriminant(), 5);
/// assert_eq!(FieldEnum::Seven { item_a: 0, item_b: 0 }.discriminant(), 7);
/// assert_eq!(FieldEnum::Nine.discriminant(), 9);
/// // Fieldless version has the same discriminants
/// assert_eq!(FieldEnum_::NOne.discriminant(), -1);
/// assert_eq!(FieldEnum_::One.discriminant(), 1);
/// assert_eq!(FieldEnum_::Two.discriminant(), 2);
/// assert_eq!(FieldEnum_::Three.discriminant(), 3);
/// assert_eq!(FieldEnum_::Five.discriminant(), 5);
/// assert_eq!(FieldEnum_::Seven.discriminant(), 7);
/// assert_eq!(FieldEnum_::Nine.discriminant(), 9);
/// ```
#[proc_macro_attribute]
pub fn discriminant(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let attr = proc_macro2::TokenStream::from(attr);

    discriminant::discriminant_impl(attr, item).into()
}
