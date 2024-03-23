use proc_macro2::{Group, Literal, Punct, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
    parenthesized,
    parse::{ParseStream, Parser},
    token::Paren,
    Error, Ident, Index, Result, Token,
};

use crate::{custom_ident::CustomIdMap, discard_parse_buffer};

pub fn handle_formatted(input: ParseStream, len: u32, cidm: &CustomIdMap) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();

    while !input.is_empty() {
        if !input.peek(Token![#]) {
            pass(input, &mut tokens, |input| {
                handle_formatted(input, len, cidm)
            })?;
            continue;
        }

        let pound = input.parse::<Token![#]>()?;
        if input.peek(Paren) {
            let content;
            parenthesized!(content in input);

            let separator = if input.peek(Token![*]) {
                None
            } else {
                Some(input.parse::<Punct>()?)
            };

            input.parse::<Token![*]>()?;

            let mut is_first = true;
            for nth in 0..len {
                if is_first {
                    is_first = false;
                } else {
                    separator.to_tokens(&mut tokens);
                }

                inside_repetition(&content.fork(), len, nth, cidm)?.to_tokens(&mut tokens);
            }
            discard_parse_buffer(content);
        } else if input.peek(Ident) {
            let id = input.parse::<Ident>()?;

            match &*id.to_string() {
                "length" => {
                    let mut lit = Literal::u32_unsuffixed(len);
                    lit.set_span(id.span());
                    lit.to_tokens(&mut tokens);
                }
                name @ "index" => {
                    return Err(Error::new_spanned(
                        id,
                        format!(
                            "calling `{name}` needs repetition, \
                            please consider use it in `#(...)`"
                        ),
                    ))
                }
                _ => {
                    cidm.get_nth_id(&id, 0)?;
                    return Err(Error::new_spanned(
                        id,
                        "custom identifier is exists, \
                            but it can only be used inside repetition",
                    ));
                }
            }
        } else {
            pound.to_tokens(&mut tokens);
        }
    }
    Ok(tokens)
}

fn pass<F>(input: ParseStream, output: &mut TokenStream, handle_group: F) -> Result<()>
where
    F: Fn(ParseStream) -> Result<TokenStream>,
{
    let tt = input.parse::<TokenTree>()?;
    match tt {
        TokenTree::Group(group) => {
            let mut new_group = Group::new(group.delimiter(), handle_group.parse2(group.stream())?);
            new_group.set_span(group.span());
            new_group.to_tokens(output);
        }
        _ => tt.to_tokens(output),
    }
    Ok(())
}

fn inside_repetition(
    input: ParseStream,
    len: u32,
    nth: u32,
    cidm: &CustomIdMap,
) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();

    while !input.is_empty() {
        if !input.peek(Token![#]) {
            pass(input, &mut tokens, |input| {
                inside_repetition(input, len, nth, cidm)
            })
            .unwrap();
            continue;
        }

        let pound = input.parse::<Token![#]>()?;
        if input.peek(Paren) {
            return Err(Error::new_spanned(
                input.parse::<TokenTree>()?,
                "there is no more repetition",
            ));
        } else if input.peek(Ident) {
            let id = input.parse::<Ident>()?;

            match &*id.to_string() {
                "length" => {
                    let mut lit = Literal::u32_unsuffixed(len);
                    lit.set_span(id.span());
                    lit.to_tokens(&mut tokens);
                }
                "index" => Index {
                    index: nth,
                    span: id.span(),
                }
                .to_tokens(&mut tokens),
                _ => cidm.get_nth_id(&id, nth)?.to_tokens(&mut tokens),
            }
        } else {
            pound.to_tokens(&mut tokens);
        }
    }

    Ok(tokens)
}
