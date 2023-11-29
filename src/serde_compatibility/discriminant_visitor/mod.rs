use serde::de::{Error, Visitor};
use core::fmt::Formatter;

///Visitor to deserialize usize
pub const DISCRIMINANT_VISITOR: USizediscriminantVisitor = USizediscriminantVisitor;

///Empty struct of a visitor that deserialize to a single usize
pub struct USizediscriminantVisitor;

impl Visitor<'_> for USizediscriminantVisitor {
    type Value = usize;

    fn expecting(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("Value was supossed to be in usize's range")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> where E: Error {
        v.try_into().map_err(|_|E::custom("Value not in usize's range"))
    }

}
