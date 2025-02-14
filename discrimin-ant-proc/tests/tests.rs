#![expect(missing_docs)]
use discrimin_ant_proc::discriminant;

#[discriminant(i8)]
pub enum FieldEnum {
    NOne(u8) = -1,
    One { item: u8 } = 1,
    Two = 2,
    Three(u8, i16),
    Five = 5,
    Seven { item_a: i128, item_b: u32 } = 7,
    Nine = 9,
}

#[discriminant(i8)]
pub enum FieldlessEnum {
    NOne = -1,
    One = 1,
    Two = 2,
    Three,
    Five = 5,
    Seven = 7,
    Nine = 9,
}

#[test]
fn test_field_discriminants() {
    assert_eq!(FieldEnum::NOne(0).discriminant(), -1);
    assert_eq!(FieldEnum::One { item: 0 }.discriminant(), 1);
    assert_eq!(FieldEnum::Two.discriminant(), 2);
    assert_eq!(FieldEnum::Three(0, 0).discriminant(), 3);
    assert_eq!(FieldEnum::Five.discriminant(), 5);
    assert_eq!(FieldEnum::Seven { item_a: 0, item_b: 0 }.discriminant(), 7);
    assert_eq!(FieldEnum::Nine.discriminant(), 9);

    assert_eq!(FieldEnum_::NOne.discriminant(), -1);
    assert_eq!(FieldEnum_::One.discriminant(), 1);
    assert_eq!(FieldEnum_::Two.discriminant(), 2);
    assert_eq!(FieldEnum_::Three.discriminant(), 3);
    assert_eq!(FieldEnum_::Five.discriminant(), 5);
    assert_eq!(FieldEnum_::Seven.discriminant(), 7);
    assert_eq!(FieldEnum_::Nine.discriminant(), 9);
}

#[test]
fn test_fieldless_discriminants() {
    assert_eq!(FieldlessEnum::NOne.discriminant(), -1);
    assert_eq!(FieldlessEnum::One.discriminant(), 1);
    assert_eq!(FieldlessEnum::Two.discriminant(), 2);
    assert_eq!(FieldlessEnum::Three.discriminant(), 3);
    assert_eq!(FieldlessEnum::Five.discriminant(), 5);
    assert_eq!(FieldlessEnum::Seven.discriminant(), 7);
    assert_eq!(FieldlessEnum::Nine.discriminant(), 9);

    assert_eq!(FieldlessEnum_::NOne.discriminant(), -1);
    assert_eq!(FieldlessEnum_::One.discriminant(), 1);
    assert_eq!(FieldlessEnum_::Two.discriminant(), 2);
    assert_eq!(FieldlessEnum_::Three.discriminant(), 3);
    assert_eq!(FieldlessEnum_::Five.discriminant(), 5);
    assert_eq!(FieldlessEnum_::Seven.discriminant(), 7);
    assert_eq!(FieldlessEnum_::Nine.discriminant(), 9);
}
