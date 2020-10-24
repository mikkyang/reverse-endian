extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Fields, GenericParam,
    Generics, Ident, Index,
};

#[proc_macro_derive(ReverseEndian)]
pub fn derive_reverse_endian(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let reversed = reverse_endian(&name, &input.data);

    let output = quote! {
        impl #impl_generics reverse_endian::ReverseEndian for #name #type_generics #where_clause {
            fn reverse_endian(self) -> Self {
                #reversed
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

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

fn reverse_endian(name: &Ident, data: &Data) -> proc_macro2::TokenStream {
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
