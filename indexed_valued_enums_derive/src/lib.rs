#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
extern crate proc_macro;

use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use proc_macro::TokenStream;

use proc_macro2::{Ident, Punct};
use quote::{quote, ToTokens};
use syn::{Attribute, DataEnum, DeriveInput, parse_macro_input, Type};
use syn::Data;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

const INCORRECT_VALUED_AS_FORMAT_ERROR_MESSAGE: &'static str = "Wrong syntax of attribute '#[valued_as(*type*)]', it must have one and just one type as content, like:\n\n\
                  #[derive(Valued)]\n#[enum_valued_as(*your type*)]\nenum your_enums_name {{\n\t...\n}} ";

/// Implements the 'Indexed' and 'Valued' traits for an enum, allowing to get a discriminant / index
/// and a value for each variant through the functions 'discriminant' and 'value', and get this
/// variant back using the functions 'from_discriminant_opt' and 'value_to_variant_opt'. <br><br>
///
/// **Note**: This requires the 'derive' feature on your Cargo.toml, like
/// ```indexed_valued_enums = { version =  "1.0.0", features=["derive", ...] }```.<br><br>
///
/// Attributes:
///
/// | Attribute | Target | Contents description |
/// |---|---|---|
/// | #[enum_valued_as(type)] | Enum | Type of your variant’s values. <br><br> This is silently an Attribute macro that adds ‘#[repr(usize)]’ to your enum, rather than a simple attribute, it’s used is also reserved if in the future new features should be born that require to modify your enum silently, if so, changes will appear both here and in the [enum_valued_as] documentation.  |
/// | #[unvalued_default<br>(default value)] | Enum | Default value for variants whose value isn’t specified. |
/// | #[enum_valued_features<br>(extra features)] | Enum | List of extra features, you can find a detailed list of every extra feature in this crate’s index. |
/// | #[value(This variant’s value)] | Variant | Value this variant will resolve to when calling the ‘value’ function. |
/// | #[variant_initialize_uses<br>(Field default values)] | Variant with fields | Specifies the contents of the field of said. |
///
/// <br>
///
/// ## Step-by-step detailed explanation
///
/// **Basic implementation**: Add the derive [indexed_valued_enums::Valued] macro and then write the
/// #[enum_valued_as(*Value type*)] attribute indicating the type your variants will resolve to,
/// then on each variant write an attribute #[value(*this variants value*)]. this way: <br><br>
///
/// ```rust ignore
/// use indexed_valued_enums::{Valued, enum_valued_as};
///
/// #[derive(Valued)]
/// #[enum_valued_as(u8)]
/// pub enum MyEnum{
///     #[value(10)]
///     Variant1,
///     #[value(20)]
///     Variant2,
/// }
/// ```
/// <br>
///
/// **Add extra functionality**: Below the Derive declaration you can write the attribute
/// #[enum_valued_features(*Your desired features*)] which will automatically implement certain
/// traits or functions which will become helpful, you can check these features on the section
/// [extra features](#extra-features).<br>
///
/// ```rust ignore
/// ...
/// /// Adding 'Delegators' allows to call most of functions at
/// /// compile-time, being able to get values and variants easily
/// #[enum_valued_features(DerefToValue, Delegators)]
/// pub enum MyEnum{
///     ...
/// }
/// ```
/// <br>
///
/// **Don't repeat yourself**: For variants whose variants values are often repeated or irrelevant
/// you can use the attribute #[unvalued_default(*Your default value*)] which will make all these
/// unvalued variants to resolve into said value.<br>
///
/// ```rust ignore
/// ...
/// #[unvalued_default(50)]
/// pub enum MyEnum{
///     /// This variant's value will resolve to 10 as it is specified.
///     #[value(10)]
///     Variant1,
///     /// This variant's value will resolve to the default of 50 as a value it is not specified.
///     Variant2,
/// }
/// ```
/// <br>
///
/// **Variant's with fields can be added too!** Unlike the declarative macro, this one is compatible
/// with variants with fields, be them named or unnamed, but they have a downside: since the 
/// [Indexed::from_discriminant] function must return a constant value for each variants, we also 
/// need to create those variants with values at compile, when this situation arises you have two 
/// options:
///
/// * Use the #[variant_initialize_uses(*Your default value*)]: Here you write the default contents
/// for these variants, for example, if one was ```IP{host: &'static str, port: u16}```, you could
/// write: #[variant_initialize_uses(host: "localhost", port: 8080)]<br><br>
/// * If the values on of the variant implement [const_default::ConstDefault]: You can simply add
/// const-default in your Cargo.toml like ```const-default = { version =  "1.0.0" }``` and when this
/// variant gets resolved from [Indexed::from_discriminant], it will return it with all fields as
/// specified in [const_default::ConstDefault].
///
/// ```rust ignore
/// ...
/// pub enum MyEnum{
///     /// When applying [from::discriminant] to 0, it will return MyEnum::Variant1(23, "Jorge")
///     #[variant_initialize_uses(23, "Jorge")]
///     Variant1(u8, &'static str),
///     /// Since the attribute #[variant_initialize_uses] isn't specified, when applying
///     /// [from::discriminant] to 1, it will return MyEnum::Variant2{age: 0}, as ConstDefault
///     /// for u8 returns 0 
///     Variant2{age:u8},
/// }
/// ```
/// <br>
///
/// ## Examples
///
/// A simple example using this macro could look like this:
///
/// ```rust ignore
/// use indexed_valued_enums::{Valued, enum_valued_as};
///
/// #[derive(Valued)]
/// #[enum_valued_as(&'static str)]
/// pub enum Number{
///     #[value("Zero position")]
///     Zero,
///     #[value("First position")]
///     First,
///     #[value("Second position")]
///     Second,
///     #[value("Third position")]
///     Third,
/// }
/// ```
/// <br>
/// A more complex example could look like:
///
/// ```rust ignore
/// use indexed_valued_enums::{Valued, enum_valued_as};
///
/// #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
/// #[derive(Valued)]
/// #[enum_valued_as(&'static str)]
/// #[enum_valued_features(Clone, DerefToValue, Delegators, ValueToVariantDelegators)]
/// #[unvalued_default("My default string")]
/// pub enum Number{
///     /// Zero doesn't have a value, so it's value will resolve to "My default string"
///     Zero,
///     #[value("First position")]
///     First,
///     /// Second is a variant with fields: u8 and u16, since it's not specified, when calling
///     /// [Indexed::from_discriminant] the values for both will be 0, which are their default
///     /// values on [const_default::ConstDefault::DEFAULT]
///     #[value("Second position")]
///     Second(u8, u16),
///     /// Third is a variant with fields: my_age: u8 and my_name:&'static str, as specified,
///     /// calling [Indexed::from_discriminant] will result in those fields contanining
///     /// my_age: 23, my_name: "Jorge"
///     #[variant_initialize_uses(my_age: 23, my_name: "Jorge")]
///     #[value("Third position")]
///     Third{my_age: u8, my_name:&'static str},
/// }
///
///
/// ```
#[proc_macro_derive(Valued, attributes(enum_valued_features, unvalued_default, variant_initialize_uses, value))]
pub fn derive_macro_describe(input: TokenStream) -> TokenStream {
    /*    let cloned_input = input.clone();
    print_info("Derive input info", &*format!("{:#?}\n", parse_macro_input!(cloned_input as DeriveInput)));*/
    let DeriveInput { attrs, ident, data, .. } = parse_macro_input!(input as DeriveInput);
    match data {
        Data::Struct(_) | Data::Union(_) => panic!("The 'Valued' derive macro targets c-like enums, not structs or union, consider removing '#[Derive(Valued)]' for this type"),
        Data::Enum(my_enum) => return derive_enum(&attrs, &ident, my_enum),
    };
}

fn derive_enum(attrs: &Vec<Attribute>, enum_name: &Ident, my_enum: DataEnum) -> TokenStream {
    let valued_as_attribute = find_attribute_last_in_path(&attrs, "enum_valued_as")
        .expect(&*format!("Could not find attribute 'valued_as(*type*)'\nRemember '#[derive(Valued)]' must appear before before #[valued_as(*your type*)], like:\n\n\
                  #[derive(Valued)]\n#[enum_valued_as(*your type*)]\nenum {enum_name} {{\n\t...\n}} "));
    let valued_as = valued_as_attribute.parse_args::<ValuedAsAttribute>()
        .expect(INCORRECT_VALUED_AS_FORMAT_ERROR_MESSAGE)
        .type_of_value;
    let unvalued_default = find_attribute(&attrs, "unvalued_default")
        .map(|unvalued_default| { &unvalued_default.tokens });

    let features = find_attribute(&attrs, "enum_valued_features")
        .map(|features_attr| features_attr.parse_args::<Features>().expect(&format!("Wrong syntax of attribute '#[enum_valued_features(*desired features*)]', it must contain just a set of your desired features, which can be consulted on the indexed_valued_enums::create_indexed_valued_enum macro\n\
                Your enum's should look like this, like:\n\n\
                  #[derive(Valued)]\n#[enum_valued_as({valued_as:?})]\n#[value(...)] <------- Your features here, like 'Delegators, ValueToVariantDelegators...' \nenum {enum_name} {{\n\t...\n}} "))
            .idents)
        .unwrap_or(Vec::new());

    let mut variants = Vec::with_capacity(my_enum.variants.len());
    let mut variants_values = Vec::with_capacity(my_enum.variants.len());
    let mut variants_fields_initializer = Vec::with_capacity(my_enum.variants.len());

    my_enum.variants.iter().for_each(|variant| {
        //print_info("variants", &format!("{variant:#?}"));
        let variant_name = &variant.ident;
        let variant_value = find_attribute(&variant.attrs, "value")
            .map(|variants_value_attr| { &variants_value_attr.tokens })
            .or_else(|| unvalued_default.clone())
            .expect(&format!("Could not find value for variant {variant_name}\n\n Consider adding a value like:\n\n\
                                          #[value(...)] <------- Your value of type {valued_as:?}\n{variant_name}\n\n\n Or add a default value for variants without values, like\n\n\
                                          #[derive(Valued)]\n#[enum_valued_as(*your type*)]\n#[unvalued_default(...)] <------- Your value of type\nenum {{\n\t...\n}} ", ));
        let variant_initialize_uses = find_attribute(&variant.attrs, "variant_initialize_uses")
            .map(|variants_value_attr| extract_token_stream_of_attribute(variants_value_attr));

        print_info(&format!("variant_initialize_uses of variant {enum_name}::{variant_name}"), &format!("{:#?}", variant_initialize_uses));

        let first_field_is_named = variant.fields.iter().next().map(|first_field| first_field.ident.is_some()).unwrap_or(false);

        let internal_fields_as_default = variant.fields
            .iter()
            .map(|field| {
                field.ident.as_ref()
                    .map(|field_name| quote!(#field_name (const_default::ConstDefault::DEFAULT)))
                    .unwrap_or_else(|| quote!((const_default::ConstDefault::DEFAULT)))
            })
            .reduce(|prev_token, next_token| quote!(#prev_token, #next_token));


        variants.push(&variant.ident);
        variants_values.push(variant_value);
        variants_fields_initializer.push(
            variant_initialize_uses.map(From::from).or(internal_fields_as_default)
                .map(|initializers| if first_field_is_named {
                    quote!(; named_field_initializers #initializers ;)
                } else {
                    quote!(; unnamed_field_initializers #initializers ;)
                })
                .unwrap_or_else(|| quote!())
        );
    });

    let output = quote! {
                indexed_valued_enums::create_indexed_valued_enum !(impl traits #enum_name #valued_as; #(#variants, #variants_values #variants_fields_initializer),*);
                indexed_valued_enums::create_indexed_valued_enum !(process features #enum_name, #valued_as; #(#features);*);
            };
    print_info("output_str", &format!("{:#?}", output.to_string()));
    output.into()
}

fn extract_token_stream_of_attribute(variants_value_attr: &Attribute) -> TokenStream {
    let mut token_stream = Option::None;
    variants_value_attr.parse_args_with(|input: ParseStream| {
        token_stream = Some(TokenStream::from(input.cursor().token_stream()));
        Ok(())
    });
    token_stream.unwrap()
}

fn find_attribute_last_in_path<'attr>(attrs: &'attr Vec<Attribute>, attribute_ident: &str) -> Option<&'attr Attribute> {
    attrs.iter()
        .filter(|attribute| attribute.path.segments.iter().last().is_some_and(|segment| segment.ident.to_string().eq(attribute_ident)))
        .next()
}

fn find_attribute<'attr>(attrs: &'attr Vec<Attribute>, attribute_ident: &str) -> Option<&'attr Attribute> {
    attrs.iter()
        .filter(|attribute| attribute.path.is_ident(attribute_ident))
        .next()
}

#[derive(Debug)]
struct ValuedAsAttribute {
    type_of_value: Type,
}

impl Parse for ValuedAsAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        print_info("Trying to parse valued as attribute", &format!("{input:#?}"));
        print_info("Trying to parse valued as attribute", &format!("{input:?}"));
        input.parse::<Type>().map(|parsed_type| {
            ValuedAsAttribute { type_of_value: parsed_type }
        })
    }
}

struct Features {
    idents: Vec<Ident>,
}

impl Parse for Features {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut idents = Vec::new();
        while !input.is_empty() {
            match input.parse::<Ident>() {
                Ok(ident) => idents.push(ident),
                Err(_) => {
                    if input.parse::<Punct>().is_err() {
                        return Err(syn::Error::new(input.span(), "Not a feature or a punctuation sign"));
                    }
                }
            }
        }
        Ok(Features { idents })
    }
}

/// Attribute macro used by the 'Valued' derive macro to indicate the type of your variant's values,
/// it poses as a simple derive macro, but it is used to modify your enum and prepare it for the
/// Indexed and Valued traits, currently, this only means adding '#[repr(usize)]' to your enum, and
/// while it is unprobable, this macro is still reserved for manipulating your enum if new features
/// were to need it, for this reason, this attribute should appear right after #[derive(Valued)] and
/// before any other attributes.
#[proc_macro_attribute]
pub fn enum_valued_as(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let mut res = quote!(#[repr(usize)]);
    res.extend(item);
    res.into()
}


const DEBUG: bool = false;

fn print_info(name: &str, info: &str) {
    if !DEBUG { return; }
    //eprintln!("--------------------- {} ---------------------\n", name);
    //eprintln!("{info}\n", );
    //eprintln!("-------------------------------------------------------------\n");
}
