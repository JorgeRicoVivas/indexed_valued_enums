extern crate proc_macro;

use proc_macro::TokenStream;
use std::mem;
use proc_macro2::{Ident, Punct};
use quote::{quote, ToTokens};
use syn::{Attribute, DataEnum, DeriveInput, parse_macro_input, Type};
use syn::Data;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

const INCORRECT_VALUED_AS_FORMAT_ERROR_MESSAGE: &'static str = "Wrong syntax of attribute '#[valued_as(*type*)]', it must have one and just one type as content, like:\n\n\
                  #[derive(Valued)]\n#[enum_valued_as(*your type*)]\nenum your_enums_name {{\n\t...\n}} ";

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
    eprintln!("--------------------- {} ---------------------\n", name);
    eprintln!("{info}\n", );
    eprintln!("-------------------------------------------------------------\n");
}
