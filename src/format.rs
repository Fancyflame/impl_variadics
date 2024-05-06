use proc_macro2::{Group, Literal, Punct, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
    parenthesized,
    parse::{ParseStream, Parser},
    token::Paren,
    Error, Ident, Index, Result, Token,
};

use crate::{custom_ident::CustomIdMap, discard_parse_buffer};

#[derive(Clone, Copy)]
pub struct ArgInfo<'a> {
    pub len: u32,
    pub cidm: &'a CustomIdMap,
}

impl ArgInfo<'_> {
    fn get(&self, tokens: &mut TokenStream, id: Ident) -> Result<()> {
        match &*id.to_string() {
            "length" => {
                let mut lit = Literal::u32_unsuffixed(self.len);
                lit.set_span(id.span());
                lit.to_tokens(tokens);
            }
            "index" => match self.len.checked_sub(1) {
                Some(len) => Index {
                    index: len,
                    span: id.span(),
                }
                .to_tokens(tokens),
                None => {
                    return Err(non_zero_error(&id));
                }
            },
            _ => match self.len.checked_sub(1) {
                Some(index) => self.cidm.get_nth_id(&id, index)?.to_tokens(tokens),
                None => return Err(non_zero_error(&id)),
            },
        }
        Ok(())
    }
}

fn non_zero_error(id: &Ident) -> Error {
    Error::new_spanned(
        id,
        format!("cannot call `#{id}` while the loop count is zero, please try `#(#{id})*`"),
    )
}

pub fn handle_formatted(input: ParseStream, arg_info: ArgInfo) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();

    while !input.is_empty() {
        if !input.peek(Token![#]) {
            pass(input, &mut tokens, arg_info)?;
            continue;
        }

        let pound = input.parse::<Token![#]>()?;

        if input.peek(Ident) {
            let id = input.parse::<Ident>()?;
            arg_info.get(&mut tokens, id)?;
        } else if input.peek(Paren) {
            handle_repeater(input, &mut tokens, arg_info)?;
        } else {
            pound.to_tokens(&mut tokens);
        }
    }
    Ok(tokens)
}

fn handle_repeater(input: ParseStream, tokens: &mut TokenStream, arg_info: ArgInfo) -> Result<()> {
    let content;

    parenthesized!(content in input);

    let separator = if input.peek(Token![*]) {
        None
    } else {
        Some(input.parse::<Punct>()?)
    };

    input.parse::<Token![*]>()?;

    let mut is_first = true;
    for nth in 1..=arg_info.len {
        if is_first {
            is_first = false;
        } else {
            separator.to_tokens(tokens);
        }

        handle_formatted(
            &content.fork(),
            ArgInfo {
                len: nth,
                cidm: arg_info.cidm,
            },
        )?
        .to_tokens(tokens);
    }

    discard_parse_buffer(content);
    Ok(())
}

fn pass(input: ParseStream, output: &mut TokenStream, arg_info: ArgInfo) -> Result<()> {
    let tt = input.parse::<TokenTree>()?;
    match tt {
        TokenTree::Group(group) => {
            let mut new_group = Group::new(
                group.delimiter(),
                (|input: ParseStream| handle_formatted(input, arg_info)).parse2(group.stream())?,
            );
            new_group.set_span(group.span());
            new_group.to_tokens(output);
        }
        _ => tt.to_tokens(output),
    }
    Ok(())
}
