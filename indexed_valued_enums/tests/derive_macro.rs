use indexed_valued_enums_derive::{Valued, enum_valued_as};

#[derive(Clone, Debug, PartialEq, Valued)]
#[enum_valued_as(u8)]
#[enum_valued_features(Delegators, ValueToVariantDelegators, DerefToValue)]
enum NumberValue {
    #[value(0)]
    Zero,
    #[value(1)]
    First,
    #[value(2)]
    Second,
    #[value(3)]
    Third,
}

#[test]
fn test_valued() {
    assert_eq!(NumberValue::Zero.discriminant(), 0);
    assert_eq!(NumberValue::First.value(), 1);
    assert_eq!(*NumberValue::First, 1);
    assert_eq!(NumberValue::Second.clone(), NumberValue::Second);
    assert_eq!(NumberValue::Third, NumberValue::value_to_variant(&3));
    assert!(NumberValue::value_to_variant_opt(&4).is_none());
}


const THREE: u8 = 3;

#[derive(Clone, Debug, PartialEq, Valued)]
#[enum_valued_as(u8)]
#[unvalued_default(0)]
#[enum_valued_features(Delegators, ValueToVariantDelegators, DerefToValue)]
enum NumberValueDefaulted {
    Zero,
    #[value(1)]
    First,
    #[value(2)]
    Second,
    #[value(THREE)]
    Third,
}

#[test]
fn test_defaulted() {
    assert_eq!(NumberValueDefaulted::Zero.discriminant(), 0);
    assert_eq!(NumberValueDefaulted::First.value(), 1);
    assert_eq!(*NumberValueDefaulted::First, 1);
    assert_eq!(NumberValueDefaulted::Second.clone(), NumberValueDefaulted::Second);
    assert_eq!(NumberValueDefaulted::Third, NumberValueDefaulted::value_to_variant(&3));
    assert!(NumberValueDefaulted::value_to_variant_opt(&4).is_none());
}

#[derive(PartialEq)]
struct MyType {
    num: usize,
    name: &'static str,
}

#[derive(Clone, Valued, Debug)]
#[enum_valued_as(MyType)]
#[enum_valued_features(Delegators, ValueToVariantDelegators, DerefToValue)]
#[unvalued_default(MyType{ num: 10, name: "Ten",})]
enum NumberCustom {
    #[value(MyType { num: 0, name: "Zero" })]
    Zero,
    #[value(MyType { num: 1, name: "First" })]
    First,
    #[value(MyType { num: 2, name: "Second" })]
    Second,
    #[value(MyType { num: 3, name: "Third" })]
    Third,
    Ten,
}

#[test]
fn test_custom_type() {
    assert_eq!(NumberCustom::Zero.discriminant(), 0);
    assert_eq!(NumberCustom::First.value().num, 1);
    assert_eq!(NumberCustom::Ten.num, 10);
    assert_eq!(NumberCustom::Ten.name, "Ten");
    assert_eq!(NumberCustom::Second.clone().num, NumberCustom::Second.num);
    assert_eq!(NumberCustom::Third.num, NumberCustom::value_to_variant(&MyType { num: 3, name: "Third" }).num);
    assert!(NumberCustom::value_to_variant_opt(&MyType { num: 4, name: "Fourth" }).is_none());
}

#[test]
fn test_custom_constr() {
    assert!(NumberValueConstr::value_to_variant_opt(&4).is_none());
}

#[derive(Clone, Debug, PartialEq, Valued)]
#[enum_valued_features(Delegators, ValueToVariantDelegators, DerefToValue)]
#[enum_valued_as(u8)]
enum NumberValueConstr {
    #[value(0)]
    #[variant_initialize_uses(2, 3)]
    Zero(u16, u16),
    #[value(1)]
    First(u8, u16),
    #[value(2)]
    Second { a: u8, b: u16 },
    #[value(3)]
    #[variant_initialize_uses(c: 5, d: 7)]
    Third { c: u8, d: u16 },
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[derive(Valued)]
#[indexed_valued_enums_derive::enum_valued_as(& 'static str)]
#[enum_valued_features(Clone, DerefToValue, Delegators, ValueToVariantDelegators)]
#[unvalued_default("My default string")]
pub enum NumberComplex {
    /// Zero doesn't have a value, so it's value will resolve to "My default string"
    Zero,
    #[value("First position")]
    First,
    /// Second is a variant with fields: u8 and u16, since it's not specified, when calling
    /// [Indexed::from_discriminant] the values for both will be 0, which are their default
    /// values on [const_default::ConstDefault::DEFAULT]
    #[value("Second position")]
    Second(u8, u16),
    /// Third is a variant with fields: my_age: u8 and my_name:&'static str, as specified,
    /// calling [Indexed::from_discriminant] will result in those fields contanining
    /// my_age: 23, my_name: "Jorge"
    #[variant_initialize_uses(my_age: 23, my_name: "Jorge")]
    #[value("Third position")]
    Third { my_age: u8, my_name: &'static str },
}

#[derive(PartialEq)]
pub struct Planet {
    radius: f32,
    gravity: f32,
}

#[derive(PartialEq, Debug, Valued)]
#[enum_valued_as(Planet)]
#[enum_valued_features(DerefToValue, Delegators, ValueToVariantDelegators)]
enum Planets {
    #[value(Planet{ radius: 6357.0, gravity: 9.807 })]
    Earth,
    #[value(Planet{ radius: 3389.5, gravity: 3.71 })]
    Mars,
    #[value(Planet{ radius: 2439.7, gravity: 3.7 })]
    Mercury,
}

#[test]
fn example_test(){
    //Identifiers mechanics
    assert_eq!(Planets::Mars, Planets::from_discriminant(1));
    assert_eq!(Planets::Mercury.discriminant(), 2);

    //Value mechanics
    assert_eq!(Planets::Earth.value().radius, 6357.0);
    assert_eq!(Planets::Mars.gravity, 3.71);
    assert_eq!(Planets::Mercury, Planets::value_to_variant(&Planet{ radius: 2439.7, gravity: 3.7 }));
}