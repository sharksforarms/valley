use crate::ValleyReceiver;
use darling::ast::Data;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn emit_store(input: &ValleyReceiver) -> Result<TokenStream, syn::Error> {
    match &input.data {
        Data::Enum(_) => emit_enum(input),
        Data::Struct(_) => emit_struct(input),
    }
}

fn emit_struct(input: &ValleyReceiver) -> Result<TokenStream, syn::Error> {
    let mut tokens = TokenStream::new();

    tokens.extend(quote! {});

    Ok(tokens)
}

fn emit_enum(input: &ValleyReceiver) -> Result<TokenStream, syn::Error> {
    todo!()
}
