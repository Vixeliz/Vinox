use derive_more::{Deref, DerefMut};
use fixed::types::I60F4;
use naia_hecs_shared::{ConstBitLength, Property, Replicate, Serde};

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedPoint(pub I60F4);

impl From<I60F4> for FixedPoint {
    fn from(value: I60F4) -> Self {
        Self(value)
    }
}

impl Serde for FixedPoint {
    fn ser(&self, writer: &mut dyn naia_hecs_shared::BitWrite) {
        self.to_bits().ser(writer)
    }

    fn de(reader: &mut naia_hecs_shared::BitReader) -> Result<Self, naia_hecs_shared::SerdeErr> {
        Ok(FixedPoint(I60F4::from_bits(i64::de(reader)?)))
    }

    fn bit_length(&self) -> u32 {
        <i64 as ConstBitLength>::const_bit_length()
    }
}

#[derive(Replicate)]
pub struct Position {
    pub x: Property<FixedPoint>,
    pub y: Property<FixedPoint>,
}

impl Position {
    pub fn new(x: impl Into<I60F4>, y: impl Into<I60F4>) -> Self {
        Self::new_complete(FixedPoint(x.into()), FixedPoint(y.into()))
    }
}
