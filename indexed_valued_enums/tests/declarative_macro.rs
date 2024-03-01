use indexed_valued_enums::create_indexed_valued_enum;

create_indexed_valued_enum! {
    #[derive(Eq, PartialEq, Debug)]
    ##[features(Clone, Delegators, ValueToVariantDelegators, DerefToValue)]
    enum Number valued as NumberDescription;
    Zero, NumberDescription { description: "Zero position", index: 0 },
    First, NumberDescription { description: "First position", index: 1 },
    Second, NumberDescription { description: "Second position", index: 2 },
    Third, NumberDescription { description: "Third position", index: 3 }
}

#[derive(PartialEq)]
struct NumberDescription {
    description: &'static str,
    index: u16,
}

#[test]
fn test() {
    assert_eq!(Number::Zero.discriminant(), 0);
    assert_eq!(Number::First.value().description, "First position");
    assert_eq!(Number::First.index,1);
    assert_eq!(Number::Second.clone(), Number::Second);
    assert_eq!(Number::Third, Number::value_to_variant(
        &NumberDescription { description: "Third position", index: 3 }));
    assert!(Number::value_to_variant_opt(
        &NumberDescription { description: "Fourth position", index: 4 }).is_none());
}