#[repr(i8)]
pub enum SimpleSignedFieldlessEnum {
    NOne = -1,
    One = 1,
    Two,
    Five = 5,
    Six,
    Seven,
}
impl SimpleSignedFieldlessEnum {
    #[doc = r" Returns the discriminant of [Self]."]
    pub const fn discriminant(&self) -> i8 {
        unsafe { *core::ptr::from_ref::<Self>(self).cast::<i8>() }
    }
}
impl discrimin_ant::Discriminantable for SimpleSignedFieldlessEnum {
    type Discriminant = i8;

    fn discriminant(&self) -> Self::Discriminant {
        self.discriminant()
    }
}
#[doc = "Fieldless representations of [SimpleSignedFieldlessEnum]. Used to extract discriminants without fully constructing the enum."]
#[repr(i8)]
pub enum SimpleSignedFieldlessEnum_ {
    #[doc = "A fieldless version of [SimpleSignedFieldlessEnum::NOne], used to extract the variant's discriminant without needing to fully construct it."]
    NOne = -1,
    #[doc = "A fieldless version of [SimpleSignedFieldlessEnum::One], used to extract the variant's discriminant without needing to fully construct it."]
    One = 1,
    #[doc = "A fieldless version of [SimpleSignedFieldlessEnum::Two], used to extract the variant's discriminant without needing to fully construct it."]
    Two = 1 + 1i8,
    #[doc = "A fieldless version of [SimpleSignedFieldlessEnum::Five], used to extract the variant's discriminant without needing to fully construct it."]
    Five = 5,
    #[doc = "A fieldless version of [SimpleSignedFieldlessEnum::Six], used to extract the variant's discriminant without needing to fully construct it."]
    Six = 5 + 1i8,
    #[doc = "A fieldless version of [SimpleSignedFieldlessEnum::Seven], used to extract the variant's discriminant without needing to fully construct it."]
    Seven = 5 + 2i8,
}
impl SimpleSignedFieldlessEnum_ {
    #[doc = r" Returns the discriminant of [Self]."]
    pub const fn discriminant(&self) -> i8 {
        unsafe { *core::ptr::from_ref::<Self>(self).cast::<i8>() }
    }
}
impl discrimin_ant::Discriminantable for SimpleSignedFieldlessEnum_ {
    type Discriminant = i8;

    fn discriminant(&self) -> Self::Discriminant {
        self.discriminant()
    }
}
impl TryFrom<i8> for SimpleSignedFieldlessEnum_ {
    type Error = ();
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == (-1) {
            return Ok(Self::NOne);
        }
        if value == (1) {
            return Ok(Self::One);
        }
        if value == (1 + 1i8) {
            return Ok(Self::Two);
        }
        if value == (5) {
            return Ok(Self::Five);
        }
        if value == (5 + 1i8) {
            return Ok(Self::Six);
        }
        if value == (5 + 2i8) {
            return Ok(Self::Seven);
        }
        Err(())
    }
}
impl From<&SimpleSignedFieldlessEnum> for SimpleSignedFieldlessEnum_ {
    fn from(value: &SimpleSignedFieldlessEnum) -> Self {
        match value {
            SimpleSignedFieldlessEnum::NOne => Self::NOne,
            SimpleSignedFieldlessEnum::One => Self::One,
            SimpleSignedFieldlessEnum::Two => Self::Two,
            SimpleSignedFieldlessEnum::Five => Self::Five,
            SimpleSignedFieldlessEnum::Six => Self::Six,
            SimpleSignedFieldlessEnum::Seven => Self::Seven,
        }
    }
}
