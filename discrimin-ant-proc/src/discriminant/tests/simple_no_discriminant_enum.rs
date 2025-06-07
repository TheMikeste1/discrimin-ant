#[repr(u16)]
pub enum SimpleNoDiscriminantEnum {
    Zero,
    One,
    Two,
    Three = 3,
    Four,
    Five,
}
impl SimpleNoDiscriminantEnum {
    #[doc = r" Returns the discriminant of [Self]."]
    pub const fn discriminant(&self) -> u16 {
        unsafe { *core::ptr::from_ref::<Self>(self).cast::<u16>() }
    }
}
impl discrimin_ant::Discriminantable for SimpleNoDiscriminantEnum {
    type Discriminant = u16;

    fn discriminant(&self) -> Self::Discriminant {
        self.discriminant()
    }
}
#[doc = "Fieldless representations of [SimpleNoDiscriminantEnum]. Used to extract discriminants without fully constructing the enum."]
#[repr(u16)]
pub enum SimpleNoDiscriminantEnum_ {
    #[doc = "A fieldless version of [SimpleNoDiscriminantEnum::Zero], used to extract the variant's discriminant without needing to fully construct it."]
    Zero = 0u16,
    #[doc = "A fieldless version of [SimpleNoDiscriminantEnum::One], used to extract the variant's discriminant without needing to fully construct it."]
    One = 1u16,
    #[doc = "A fieldless version of [SimpleNoDiscriminantEnum::Two], used to extract the variant's discriminant without needing to fully construct it."]
    Two = 2u16,
    #[doc = "A fieldless version of [SimpleNoDiscriminantEnum::Three], used to extract the variant's discriminant without needing to fully construct it."]
    Three = 3,
    #[doc = "A fieldless version of [SimpleNoDiscriminantEnum::Four], used to extract the variant's discriminant without needing to fully construct it."]
    Four = 3 + 1u16,
    #[doc = "A fieldless version of [SimpleNoDiscriminantEnum::Five], used to extract the variant's discriminant without needing to fully construct it."]
    Five = 3 + 2u16,
}
impl SimpleNoDiscriminantEnum_ {
    #[doc = r" Returns the discriminant of [Self]."]
    pub const fn discriminant(&self) -> u16 {
        unsafe { *core::ptr::from_ref::<Self>(self).cast::<u16>() }
    }
}
impl discrimin_ant::Discriminantable for SimpleNoDiscriminantEnum_ {
    type Discriminant = u16;

    fn discriminant(&self) -> Self::Discriminant {
        self.discriminant()
    }
}
impl TryFrom<u16> for SimpleNoDiscriminantEnum_ {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == (0u16) {
            return Ok(Self::Zero);
        }
        if value == (1u16) {
            return Ok(Self::One);
        }
        if value == (2u16) {
            return Ok(Self::Two);
        }
        if value == (3) {
            return Ok(Self::Three);
        }
        if value == (3 + 1u16) {
            return Ok(Self::Four);
        }
        if value == (3 + 2u16) {
            return Ok(Self::Five);
        }
        Err(())
    }
}
impl From<&SimpleNoDiscriminantEnum> for SimpleNoDiscriminantEnum_ {
    fn from(value: &SimpleNoDiscriminantEnum) -> Self {
        match value {
            SimpleNoDiscriminantEnum::Zero => Self::Zero,
            SimpleNoDiscriminantEnum::One => Self::One,
            SimpleNoDiscriminantEnum::Two => Self::Two,
            SimpleNoDiscriminantEnum::Three => Self::Three,
            SimpleNoDiscriminantEnum::Four => Self::Four,
            SimpleNoDiscriminantEnum::Five => Self::Five,
        }
    }
}
