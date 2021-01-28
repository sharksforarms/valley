use crate::utils::{gen_field_ident, gen_struct_destruction};
use crate::ValleyReceiver;
use darling::ast::Data;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Type;

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
    let mut lookup_functions = Vec::new();

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


        let lookup_fn = Ident::new(&format!("lookup_{}", &field_name), Span::call_site());
        lookup_functions.push(quote! {
            fn #lookup_fn(&mut self, item: &#field_type) -> &Vec<std::rc::Rc<#input_ident #ty_generics>> {
                self.#index_name.get(item).unwrap()
            }
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

            #(#lookup_functions)*

        }
    });

    Ok(tokens)
}

fn emit_enum(_input: &ValleyReceiver) -> Result<TokenStream, syn::Error> {
    todo!()
}
