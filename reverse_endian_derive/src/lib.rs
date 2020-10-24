extern crate proc;
extern crate quote;
extern crate syn;

use syn::{parse_quote, GenericParam, Generics};

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
