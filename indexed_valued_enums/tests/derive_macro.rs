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