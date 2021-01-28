use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// Generate field name which supports both un-named/named structs/enums
/// `ident` is Some if the container has named fields
/// `index` is the numerical index of the current field used in un-named containers
/// `prefix` is true in the case of variable declarations and match arms,
/// false when the raw field is required, for example a field access
pub fn gen_field_ident<T: ToString>(ident: Option<T>, index: usize, prefix: bool) -> TokenStream {
    let field_name = match ident {
        Some(field_name) => field_name.to_string(),
        None => {
            let index = syn::Index::from(index);
            let prefix = if prefix { "index_" } else { "" };
            format!("{}{}", prefix, quote! { #index })
        }
    };

    field_name.parse().unwrap()
}

/// Generate struct destruction
///
/// - Named: `#ident { ref fields }`
/// - Unnamed: `#ident ( ref fields )`
pub fn gen_struct_destruction<I: ToTokens, F: ToTokens>(
    named: bool,
    ident: I,
    field_idents: &[F],
) -> TokenStream {
    if named {
        quote! {
            #ident {
                #(#field_idents),*
            }
        }
    } else {
        quote! {
            #ident (
                #(#field_idents),*
            )
        }
    }
}
