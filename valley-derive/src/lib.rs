use darling::{ast, FromDeriveInput, FromField, FromVariant};

pub(crate) mod store;
mod utils;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(), supports(struct_any, enum_any))]
pub(crate) struct ValleyReceiver {
    vis: syn::Visibility,
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<VariantReceiver, FieldReceiver>,
}

#[derive(Debug, FromField)]
#[darling(attributes(valley))]
struct FieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    index: bool,
}

#[derive(Debug, FromVariant)]
#[darling(attributes())]
struct VariantReceiver {
    ident: syn::Ident,
    fields: ast::Fields<FieldReceiver>,
    discriminant: Option<syn::Expr>,
}

#[proc_macro_derive(ValleyStore, attributes(valley))]
pub fn proc_valley_store(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let receiver = match ValleyReceiver::from_derive_input(&input) {
        Ok(receiver) => receiver,
        Err(err) => return err.write_errors().into(),
    };

    store::emit_store(&receiver)
        .map_or_else(|e| e.to_compile_error(), |tks| tks)
        .into()
}
