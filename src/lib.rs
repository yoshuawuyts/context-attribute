//! Set the error context using doc comments.
//!
//! ## Example
//!
//! ```rust
//! println!("hello world");
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples)]
#![cfg_attr(test, deny(warnings))]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

/// Use a doc comment to annotate the failure context of a function or try
/// block.
///
/// # Examples
///
/// ```
/// use context_attribute::context;
/// use failure::{ensure, ResultExt};
///
/// fn main() -> Result<(), failure::Error> {
///     let _ = square(2)?;
///     let _ = square(5)?;
///     let _ = square(11)?;
/// }
///
/// /// Square a number if it's less than 10.
/// #[context]
/// fn square(num: usize) -> Result<String, >{
///     ensure!(num < 10, "Number was larger than 10");
///     num * num
/// }
/// ```
#[proc_macro_attribute]
pub fn context(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let attrs = &input.attrs;
    let doc = attrs.iter().find(|attr| format!("{}", attr.path.segments.first().unwrap().value().ident) == "doc");
    let doc = match doc {
        Some(doc) => {
            let mut iter = doc.clone().tts.into_iter().skip(1);
            iter.next().unwrap()
        },
        None => return TokenStream::from(quote_spanned! {
            input.span() => compile_error!("no doc comment provided")
        }),
    };


    let vis = &input.vis;
    let constness = &input.constness;
    let unsafety = &input.unsafety;
    let asyncness = &input.asyncness;
    let abi = &input.abi;

    let generics = &input.decl.generics;
    let name = &input.ident;
    let inputs = &input.decl.inputs;
    let output = &input.decl.output;
    let body = &input.block.stmts;

    let args: Vec<syn::Pat> = inputs.pairs().filter_map(|pair| {
        match pair.into_value() {
            syn::FnArg::Captured(arg) => Some(arg.pat.clone()),
            _ => return None,
        }
    }).collect();

    let result = quote! {
        #(#attrs)*
        #vis #constness #unsafety #asyncness #abi fn #generics #name(#(#inputs)*) #output {
            #constness #unsafety #asyncness #abi fn #generics #name(#(#inputs)*) #output {
                #(#body)*
            }
            Ok(#name(#(#args)*).context(#doc.trim())?)
        }
    };

    result.into()
}
