extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn opaque(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let block = input.block;
    let sig = syn::MethodSig {
        constness: input.constness,
        unsafety: input.unsafety,
        asyncness: input.asyncness,
        abi: input.abi,
        ident: input.ident,
        decl: *input.decl,
    };

    let result = quote! {
        #sig {
            from_fn(|input| {
                #block.parse(input)
            })
        }
    };

    result.into()
}
