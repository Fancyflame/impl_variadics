#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    braced,
    buffer::Cursor,
    parse::{ParseBuffer, ParseStream, Parser},
    punctuated::Punctuated,
    Index, LitInt, LitStr, RangeLimits, Result, Token,
};

use crate::{
    custom_ident::CustomIdMap,
    format::{handle_formatted, ArgInfo},
};

mod custom_ident;
mod format;

#[proc_macro]
pub fn impl_variadics(tokens: TokenStream) -> TokenStream {
    let parser =
        |input: ParseStream| Punctuated::<_, Token![;]>::parse_terminated_with(input, impl_one);

    let ts = match parser.parse(tokens) {
        Ok(i) => i.into_iter(),
        Err(e) => return e.into_compile_error().into(),
    };

    quote! {
        #(#ts)*
    }
    .into()
}

fn impl_one(input: ParseStream) -> Result<TokenStream2> {
    let start = if input.peek(LitInt) {
        input.parse::<Index>()?.index
    } else {
        0
    };

    let end = if input.peek(Token![..]) || input.peek(Token![..=]) {
        let range_limits = input.parse::<RangeLimits>()?;
        let end = input.parse::<Index>()?.index;
        match range_limits {
            RangeLimits::Closed(_) => end + 1,
            RangeLimits::HalfOpen(_) => end,
        }
    } else {
        start + 1
    };

    let mut custom_ids = CustomIdMap::new();
    while input.peek(LitStr) {
        custom_ids.parse(input.parse::<LitStr>()?)?;
    }

    input.parse::<Token![=>]>()?;
    let content;
    braced!(content in input);

    let mut output = TokenStream2::new();
    for len in start..end {
        handle_formatted(
            &content.fork(),
            ArgInfo {
                len,
                cidm: &custom_ids,
            },
        )?
        .to_tokens(&mut output);
    }
    discard_parse_buffer(content);

    Ok(output)
}

fn discard_parse_buffer(buffer: ParseBuffer) {
    let _ = buffer.step(|_| Ok(((), Cursor::empty())));
}
