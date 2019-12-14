//!  Set the error [`context`] using doc comments.
//!
//! This is useful because instead of writing manual error messages to provide context to an error, it
//! automatically derives it from doc comments. This works especially well for async contexts, where
//! stack traces may not be persisted past yield points and thread boundaries. But contexts do.
//!
//! [`context`]: https://docs.rs/failure/0.1.5/failure/trait.ResultExt.html#tymethod.context
//!
//! ## Examples
//!
//! ```no_run
//! use context_attribute::context;
//! use failure::{ensure, ResultExt};
//!
//! /// Square a number if it's less than 10.
//! #[context]
//! fn square(num: usize) -> Result<usize, failure::Error> {
//!     ensure!(num < 10, "Number was too large");
//!     Ok(num * num)
//! }
//!
//! fn main() -> Result<(), failure::Error> {
//!     let args = std::env::args();
//!     ensure!(args.len() == 2, "usage: square <num>");
//!     let input = args.skip(1).next().unwrap().parse()?;
//!
//!     println!("result is {}", square(input)?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ```sh
//! $ cargo run --example square 12
//! Error: ErrorMessage { msg: "Number was too large" }
//! Square a number if it's less than 10.
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples)]
#![cfg_attr(test, deny(warnings))]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, ReturnType};

/// Use a doc comment to annotate the failure context of a function or try
/// block.
///
/// # Examples
///
/// ```should_panic
/// use context_attribute::context;
/// use failure::{ensure, ResultExt};
///
/// fn main() -> Result<(), failure::Error> {
///     let _ = square(2)?;
///     let _ = square(5)?;
///     let _ = square(11)?;
///
///     Ok(())
/// }
///
/// /// Square a number if it's less than 10.
/// #[context]
/// fn square(num: usize) -> Result<usize, failure::Error>{
///     ensure!(num < 10, "Number was larger than 10");
///     Ok(num * num)
/// }
/// ```
#[proc_macro_attribute]
pub fn context(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let context_msg = {
        match generate_context_msg(&attr, &input) {
            Ok(msg) => msg,
            Err(e) => {
                panic!(e);
            }
        }
    };

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block.stmts;
    let output_type = match &input.sig.output {
        ReturnType::Default => {
            return TokenStream::from(quote_spanned! {
                input.sig.output.span() => compile_error!("no return type provided")
            })
        }

        ReturnType::Type(_, ty) => &*ty,
    };

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            let mut _fn = || {
                #(#body)*
            };

            let result: #output_type = _fn();
            Ok(result.context(#context_msg)?)
        }
    };

    TokenStream::from(result)
}

fn pick_doc(input: &syn::ItemFn) -> Result<String, String> {
    let attrs = &input.attrs;
    let doc = {
        let doc = attrs
            .iter()
            .find(|attr| format!("{}", attr.path.segments.first().unwrap().ident) == "doc");
        if let Some(doc) = doc {
            doc
        } else {
            return Err(format!("could not find doc"));
        }
    };
    let mut iter = doc.clone().tokens.into_iter().skip(1);
    Ok(iter
        .next()
        .unwrap()
        .to_string()
        .trim_matches('"')
        .trim()
        .to_string())
}

fn pick_fn_name(input: &syn::ItemFn) -> String {
    input.sig.ident.to_string()
}

#[derive(Debug)]
enum ContextFormat {
    Doc,
    FnName,
    Msg(String),
}

impl ContextFormat {
    fn from(attr: &TokenStream) -> Result<Self, String> {
        let attrs = attr.to_string().trim().to_owned();

        if attrs.is_empty() {
            return Ok(ContextFormat::Doc);
        }

        if attrs == "doc" {
            return Ok(ContextFormat::Doc);
        }

        if attrs == "fn" {
            return Ok(ContextFormat::FnName);
        }

        if attrs.contains(":") {
            let v: Vec<&str> = attrs.split(":").collect();
            let (name, msg) = (v[0], v[1]);
            let msg = msg.trim().trim_matches('"').trim();
            if name.trim() == "msg" {
                return Ok(ContextFormat::Msg(msg.to_string()));
            }
            return Err(format!(
                "invalid name {} only support format like msg : xxx now",
                name
            ));
        }
        return Err(format!("only support format like #[context]\n#[context(fn)]\n#[context(doc)]\n#[context(msg:xxx)]now"));
    }
}

fn generate_context_msg(attr: &TokenStream, input: &syn::ItemFn) -> Result<String, String> {
    let context_format = ContextFormat::from(attr)?;
    match context_format {
        ContextFormat::Doc => {
            let doc = pick_doc(input)?;
            return Ok(doc);
        }
        ContextFormat::FnName => {
            let fn_name = pick_fn_name(input);
            return Ok(format!("call {} fail", fn_name));
        }
        ContextFormat::Msg(msg) => {
            return Ok(msg);
        }
    }
}
