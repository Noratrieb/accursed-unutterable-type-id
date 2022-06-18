use std::{env, fs, path::PathBuf};

use fslock::LockFile;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, GenericParam};

const DEFAULT_LOCK_FILE_NAME: &str =
    "accursed-unutterable-type-id-global-store-oh-god-is-this-cursed-dont-touch-it-LOCK";

const DEFAULT_FILE_NAME: &str =
    "accursed-unutterable-type-id-global-store-oh-god-is-this-cursed-dont-touch-it";

#[proc_macro_derive(AccursedUnutterablyTypeIdentified)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let dir_path = match env::var("ACCURSED_UNUTTERABLE_TYPE_ID_DIR") {
        Ok(string) => PathBuf::from(string),
        Err(_) => {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| fail("`CARGO_MANIFEST_DIR` environment variable not found"));
            let manifest_dir = PathBuf::from(manifest_dir);
            manifest_dir.join("target")
        }
    };

    let lock_path = dir_path.join(DEFAULT_LOCK_FILE_NAME);
    let file_path = dir_path.join(DEFAULT_FILE_NAME);

    let mut file = LockFile::open(&dir_path.join(lock_path))
        .unwrap_or_else(|_| fail("failed to open global lock file"));

    file.lock().unwrap_or_else(|_| fail("failed to lock file"));

    let old = fs::read_to_string(&file_path).unwrap_or_else(|_| "0".to_string()); // lmao

    let old: u64 = old.trim().parse().unwrap_or(0); // highly dangerous indeed

    let new_value = old
        .checked_add(1)
        .unwrap_or_else(|| fail("integer overflow. you have too many derives"));

    fs::write(&file_path, new_value.to_string())
        .unwrap_or_else(|_| fail("failed to write new number"));

    let _ = file.unlock();

    let name = input.ident;
    let generics1 = input.generics.params.iter().map(|p| match p {
        GenericParam::Type(ty) => {
            let name = &ty.ident;
            let bounds = ty.bounds.iter();
            quote!(#name: #(#bounds +)* 'static)
        }
        other => other.to_token_stream(),
    });
    let generics2 = input.generics.params.iter().map(|p| match p {
        GenericParam::Const(const_param) => const_param.ident.to_token_stream(),
        GenericParam::Type(ty) => ty.ident.to_token_stream(),
        other => other.to_token_stream(),
    });

    let where_clause = input.generics.where_clause;

    // SAFETY: We literally are the proc macro. and we have made sure that no duplicate type ids
    // will ever happen, right? :ferrisClueless:
    let expanded = quote! {
        unsafe impl<#(#generics1),*> ::accursed_unutterable_type_id::AccursedUnutterablyTypeIdentified for #name<#(#generics2),*>
        #where_clause
        {
            fn type_id() -> ::accursed_unutterable_type_id::AccursedUnutterableTypeId {
                ::accursed_unutterable_type_id::AccursedUnutterableTypeId::__internal_new(
                    ::accursed_unutterable_type_id::InternalAccursedUnutterableTypeId::__internal_new(
                        #new_value
                    )
                )
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn fail(msg: impl Into<String>) -> ! {
    let msg = msg.into();

    panic!("Failed to run accursed-unutterable-type-id proc macro with error '{msg}'. \
    Set the `ACCURSED_UNUTTERABLE_TYPE_ID_DIR` environment variable to a file path of your choice to fix this issue. \
    cargo clean could help as well.");
}
