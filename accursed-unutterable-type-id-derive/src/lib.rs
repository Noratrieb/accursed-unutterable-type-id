use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AccursedUnutterablyTypeIdentified)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // SAFETY: We literally are the proc macro. and we have made sure that no duplicate type ids
    // will ever happen, right? :ferrisClueless:
    let expanded = quote! {
        unsafe impl ::accursed_unutterable_type_id::AccursedUnutterablyTypeIdentified for #name {
            fn type_id() -> ::accursed_unutterable_type_id::AccursedUnutterableTypeId {
                ::accursed_unutterable_type_id::AccursedUnutterableTypeId(
                    ::accursed_unutterable_type_id::InternalAccursedUnutterableTypeId::new(
                        0
                    )
                )
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
