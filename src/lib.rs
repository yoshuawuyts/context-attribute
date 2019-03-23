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
#[cfg(not(test))] // NOTE: exporting main breaks tests, we should file an issue.
pub fn context(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let attrs = &input.attrs[0];
    let constness = &input.constness;
    let unsafety = &input.unsafety;
    let asyncness = &input.asyncness;
    let args = &input.decl.inputs;
    let ret = &input.decl.output;
    let name = &input.ident;
    let body = &input.block;

    let inner_args: Vec<proc_macro2::Ident> = args.iter().map(|arg| {
        dbg!(arg);
        syn::Ident::new("self", proc_macro2::Span::call_site())
    }).collect();

    dbg!(inner_args);

    let result = quote! {
        #constness #unsafety #asyncness fn #name() #ret {
            #input

            #name()
        }
    };

    result.into()
}
