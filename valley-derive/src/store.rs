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

    // if no index attr is specified, create index for all fields
    let create_all_index = fields.iter().all(|f| !f.index);

    let field_idents = fields
        .iter()
        .enumerate()
        .map(|(i, f)| gen_field_ident(f.ident.as_ref(), i, false))
        .collect::<Vec<_>>();

    let named = fields.style.is_struct();
    let destructured = gen_struct_destruction(named, &input.ident, &field_idents);

    let mut field_decls = Vec::new();
    let mut field_news = Vec::new();
    let mut field_inserts = Vec::new();
    let mut lookup_functions = Vec::new();
    let mut index_names = Vec::new();

    // generate indices
    for (i, field) in fields.fields.iter().enumerate() {
        if create_all_index || field.index {
            let field_ident = gen_field_ident(field.ident.as_ref(), i, false);
            let field_type = &field.ty;

            let lookup_fn = Ident::new(&format!("lookup_{}", &field_ident), Span::call_site());
            let index_name = gen_field_ident(field.ident.as_ref(), i, true);
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
                let entry = self.#index_name.entry(#field_ident).or_insert(Vec::new());
                entry.push(rc.clone());
            });

            lookup_functions.push(quote! {
                fn #lookup_fn(&mut self, item: &#field_type) -> &Vec<std::rc::Rc<#input_ident #ty_generics>> {
                    self.#index_name.get(item).unwrap()
                }
            });

            index_names.push(index_name);
        }
    }

    // create a phantom marker for non-index fields w/ non-indexed lookup
    //
    // We do this so that we don't get errors if field_type
    // is generic over T or a 'lifetime and throws
    // errors about not being used. Maybe there's a better way?
    for (i, field) in fields.fields.iter().enumerate() {
        if !create_all_index && !field.index {
            let field_ident = gen_field_ident(field.ident.as_ref(), i, false);
            let field_type = &field.ty;
            let lookup_fn = Ident::new(&format!("lookup_{}", &field_ident), Span::call_site());

            let phantom_field = Ident::new(
                &format!("_phantom_{}", field_ident.to_string()),
                Span::call_site(),
            );

            field_decls.push(quote! {
                #phantom_field: std::marker::PhantomData<#field_type>,
            });

            field_news.push(quote! {
                #phantom_field: Default::default(),
            });

            // for lookup of non-index: pick some index and do iterative search
            let some_index = index_names.as_slice().first().unwrap();
            lookup_functions.push(quote! {
                fn #lookup_fn(&mut self, item: &#field_type) -> Vec<std::rc::Rc<#input_ident #ty_generics>> {
                    self.#some_index.values()
                    .map(|vals| {
                        vals.iter()
                            .filter(|v| &v.#field_ident == item)
                            .map(|rc| rc.clone())
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect()
                }
            });
        }
    }

    tokens.extend(quote! {
        #[derive(Debug)]
        struct #name #impl_generics #where_clause {
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

    // println!("{:#?}", tokens.to_string());
    Ok(tokens)
}

fn emit_enum(_input: &ValleyReceiver) -> Result<TokenStream, syn::Error> {
    todo!()
}
