use crate::ValleyReceiver;
use darling::ast::Data;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::Type;

/// Generate field name which supports both un-named/named structs/enums
/// `ident` is Some if the container has named fields
/// `index` is the numerical index of the current field used in un-named containers
/// `prefix` is true in the case of variable declarations and match arms,
/// false when the raw field is required, for example a field access
fn gen_field_ident<T: ToString>(ident: Option<T>, index: usize, prefix: bool) -> TokenStream {
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
fn gen_struct_destruction<I: ToTokens, F: ToTokens>(
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

pub(crate) fn emit_store(input: &ValleyReceiver) -> Result<TokenStream, syn::Error> {
    match &input.data {
        Data::Enum(_) => emit_enum(input),
        Data::Struct(_) => emit_struct(input),
    }
}

fn emit_struct(input: &ValleyReceiver) -> Result<TokenStream, syn::Error> {
    let mut tokens = TokenStream::new();

    let input_ident = &input.ident;
    let name = Ident::new(
        &format!("{}Store", input.ident.to_string()),
        Span::call_site(),
    );

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = &input.data.as_ref().take_struct().unwrap();

    let mut field_idents = Vec::new();
    let mut field_decls = Vec::new();
    let mut field_news = Vec::new();
    let mut field_inserts = Vec::new();

    for (i, field) in fields.fields.iter().enumerate() {
        let field_name = gen_field_ident(field.ident.as_ref(), i, false);
        let index_name = gen_field_ident(field.ident.as_ref(), i, true);
        let field_type = &field.ty;
        let index_type: Type = syn::parse2(quote! {
            std::collections::HashMap<#field_type, Vec<std::rc::Rc<#input_ident #ty_generics>>>
        })?;

        field_decls.push(quote! {
            #index_name: #index_type,
        });

        field_news.push(quote! {
            #index_name: Default::default(),
        });

        field_inserts.push(quote! {
            let entry = self.#index_name.entry(#field_name).or_insert(Vec::new());
            entry.push(rc.clone());
        });

        field_idents.push(field_name);
    }

    let named = fields.style.is_struct();
    let destructured = gen_struct_destruction(named, &input.ident, &field_idents);

    tokens.extend(quote! {
        #[derive(Debug)]
        struct #name #ty_generics #where_clause {
            #(#field_decls)*
        }

        impl #impl_generics #name #ty_generics #where_clause {
            fn new() -> Self {
                Self {
                    #(#field_news)*
                }
            }

            fn insert(&mut self, item: #input_ident #ty_generics) {
                match item.clone() {
                    #destructured => {
                        let rc = std::rc::Rc::new(item);
                        #(#field_inserts)*
                    }
                }
            }
        }
    });

    Ok(tokens)
}

fn emit_enum(_input: &ValleyReceiver) -> Result<TokenStream, syn::Error> {
    todo!()
}
