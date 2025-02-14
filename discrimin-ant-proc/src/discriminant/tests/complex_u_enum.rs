#[repr(u8)]
pub enum ComplexUEnum {
    One(i32) = 1,
    Two(i32),
    Five { x: u32 } = 5,
    Six { x: u32 },
}
impl ComplexUEnum {
    #[doc = r" Returns the discriminant of [Self]."]
    pub const fn discriminant(&self) -> u8 {
        unsafe { *core::ptr::from_ref::<Self>(self).cast::<u8>() }
    }
}
impl discrimin_ant::Discriminantable for ComplexUEnum {
    type Discriminant = u8;

    fn discriminant(&self) -> Self::Discriminant {
        self.discriminant()
    }
}
#[doc = "Fieldless representations of [ComplexUEnum]. Used to extract discriminants without fully constructing the enum."]
#[repr(u8)]
pub enum ComplexUEnum_ {
    #[doc = "A fieldless version of [ComplexUEnum::One], used to extract the variant's discriminant without needing to fully construct it."]
    One = 1,
    #[doc = "A fieldless version of [ComplexUEnum::Two], used to extract the variant's discriminant without needing to fully construct it."]
    Two = 1 + 1u8,
    #[doc = "A fieldless version of [ComplexUEnum::Five], used to extract the variant's discriminant without needing to fully construct it."]
    Five = 5,
    #[doc = "A fieldless version of [ComplexUEnum::Six], used to extract the variant's discriminant without needing to fully construct it."]
    Six = 5 + 1u8,
}
impl ComplexUEnum_ {
    #[doc = r" Returns the discriminant of [Self]."]
    pub const fn discriminant(&self) -> u8 {
        unsafe { *core::ptr::from_ref::<Self>(self).cast::<u8>() }
    }
}
impl discrimin_ant::Discriminantable for ComplexUEnum_ {
    type Discriminant = u8;

    fn discriminant(&self) -> Self::Discriminant {
        self.discriminant()
    }
}
impl TryFrom<u8> for ComplexUEnum_ {
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
impl From<&ComplexUEnum> for ComplexUEnum_ {
    fn from(value: &ComplexUEnum) -> Self {
        match value {
            ComplexUEnum::One(..) => Self::One,
            ComplexUEnum::Two(..) => Self::Two,
            ComplexUEnum::Five { .. } => Self::Five,
            ComplexUEnum::Six { .. } => Self::Six,
        }
    }
}
