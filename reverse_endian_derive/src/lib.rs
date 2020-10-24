extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, Data, Fields, GenericParam, Generics, Ident, Index};

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(reverse_endian::ReverseEndian));
        }
    }

    generics
}

fn reverse_endian(name: &Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span() =>
                        #name: reverse_endian::ReverseEndian::reverse_endian(self.#name)
                    }
                });
                quote! {
                    #name {
                        #(#recurse,)*
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {f.span()=>
                        reverse_endian::ReverseEndian::reverse_endian(&self.#index)
                    }
                });
                quote! {
                    #name(#(#recurse, )*)
                }
            }
            Fields::Unit => quote!( #name{} ),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
