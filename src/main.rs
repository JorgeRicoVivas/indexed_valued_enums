#![feature(generic_const_exprs)]
#![feature(trace_macros)]

use core::convert::TryInto;
use std::ops::Deref;

use indexed_enum::Indexed;
use valued_enum::Valued;

use crate::Number::*;

fn main() {
    println!("{}", MyThird.index());
    println!("{:?}", MyThird.value());
    println!("{:?}", Number::from_index(2));

    /*
    let serialized = serde_json::to_string(&MyThird).unwrap();
    println!("Serialized: {}", serialized);
    let deserialized = serde_json::from_str::<Number>(&*serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
    */

    println!("{:?}", Number::from_index(1));
}

#[derive(Clone, Debug, PartialEq)]
struct ValStruct {
    name: &'static str,
    num: u16,
}


indexed_and_valued_enum! {
    enum Number,
    derives: [Hash,Ord, PartialOrd, Eq, PartialEq, Debug],
    //features: [Serialize, Deserialize, DerefToValue],
    features: [DerefToValue],
    value type: ValStruct,
    MyZero, ValStruct { name: "Zero", num: 0 },
    MyFirst, ValStruct { name: "First", num: 1 },
    MySecond, ValStruct { name: "Second", num: 2 },
    MyThird, ValStruct { name: "Third", num: 3 }
}


#[cfg(feature = "serde_enums")]
pub mod serde_compatibility;
pub mod valued_enum;
pub mod indexed_enum;
pub mod macros;