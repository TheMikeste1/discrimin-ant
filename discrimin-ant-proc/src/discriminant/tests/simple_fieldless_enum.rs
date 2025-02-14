#[repr(u8)]
pub enum SimpleFieldlessEnum {
    One = 1,
    Two,
    Five = 5,
    Six,
}
impl SimpleFieldlessEnum {
    #[doc = r" Returns the discriminant of [Self]."]
    pub const fn discriminant(&self) -> u8 {
        unsafe { *core::ptr::from_ref::<Self>(self).cast::<u8>() }
    }
}
impl discrimin_ant::Discriminantable for SimpleFieldlessEnum {
    type Discriminant = u8;

    fn discriminant(&self) -> Self::Discriminant {
        self.discriminant()
    }
}
#[doc = "Fieldless representations of [SimpleFieldlessEnum]. Used to extract discriminants without fully constructing the enum."]
#[repr(u8)]
pub enum SimpleFieldlessEnum_ {
    #[doc = "A fieldless version of [SimpleFieldlessEnum::One], used to extract the variant's discriminant without needing to fully construct it."]
    One = 1,
    #[doc = "A fieldless version of [SimpleFieldlessEnum::Two], used to extract the variant's discriminant without needing to fully construct it."]
    Two = 1 + 1u8,
    #[doc = "A fieldless version of [SimpleFieldlessEnum::Five], used to extract the variant's discriminant without needing to fully construct it."]
    Five = 5,
    #[doc = "A fieldless version of [SimpleFieldlessEnum::Six], used to extract the variant's discriminant without needing to fully construct it."]
    Six = 5 + 1u8,
}
impl SimpleFieldlessEnum_ {
    #[doc = r" Returns the discriminant of [Self]."]
    pub const fn discriminant(&self) -> u8 {
        unsafe { *core::ptr::from_ref::<Self>(self).cast::<u8>() }
    }
}
impl discrimin_ant::Discriminantable for SimpleFieldlessEnum_ {
    type Discriminant = u8;

    fn discriminant(&self) -> Self::Discriminant {
        self.discriminant()
    }
}
impl TryFrom<u8> for SimpleFieldlessEnum_ {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == (1) {
            return Ok(Self::One);
        }
        if value == (1 + 1u8) {
            return Ok(Self::Two);
        }
        if value == (5) {
            return Ok(Self::Five);
        }
        if value == (5 + 1u8) {
            return Ok(Self::Six);
        }
        Err(())
    }
}
impl From<&SimpleFieldlessEnum> for SimpleFieldlessEnum_ {
    fn from(value: &SimpleFieldlessEnum) -> Self {
        match value {
            SimpleFieldlessEnum::One => Self::One,
            SimpleFieldlessEnum::Two => Self::Two,
            SimpleFieldlessEnum::Five => Self::Five,
            SimpleFieldlessEnum::Six => Self::Six,
        }
    }
}
