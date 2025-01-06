mod argument;
mod details;

use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{parse2, spanned::Spanned, Block, Error, ItemFn, Result, Signature, Type};
use {argument::Argument, details::CommandDetails};
use crate::util;

use self::details::InputOptions;

/// The implementation of the command macro, this macro modifies the provided function body to allow
/// parsing all function arguments and wraps it into a command struct, registering all command names,
/// types and descriptions.
pub fn command(macro_attrs: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    let fun = parse2::<ItemFn>(input)?;

    let ItemFn {
        mut attrs,
        vis,
        mut sig,
        mut block,
    } = fun;

    if sig.inputs.is_empty() {
        // The function must have at least one argument, which must be an `SlashContext`
        return Err(Error::new(
            sig.inputs.span(),
            "Expected at least SlashContext as a parameter",
        ));
    }

    let input_options = InputOptions::new(macro_attrs, &sig.ident)?;

    // The name of the function
    let ident = sig.ident.clone();
    // The name the function will have after macro execution
    let fn_ident = quote::format_ident!("_{}", &sig.ident);
    sig.ident = fn_ident.clone();

    let (context_ident, context_type) = get_context_type_and_ident(&sig)?;
    let output = util::get_return_type(&sig)?;
    let returnable = util::get_returnable_trait();

    // Get the hook macro so we can fit the function into a normal fn pointer
    let extract_output = util::get_hook_macro();
    let command_path = util::get_command_path();

    let args = parse_arguments(
        &mut sig,
        &mut block,
        context_ident,
        input_options.chat
    )?;
    let opts = CommandDetails::parse(input_options, &mut attrs)?;

    Ok(quote::quote! {
        pub fn #ident() -> #command_path<#context_type, <#output as #returnable>::Ok, <#output as #returnable>::Err> {
            #command_path::new(#fn_ident)
                #opts
                #(#args)*
        }

        #[#extract_output]
        #(#attrs)*
        #vis #sig #block
    })
}

/// Prepares the given function to parse the required arguments
pub fn parse_arguments(
    sig: &mut Signature,
    block: &mut Block,
    ctx_ident: Ident,
    chat_command: bool
) -> Result<Vec<Argument>> {
    let mut arguments = Vec::new();
    while sig.inputs.len() > 1 {
        arguments.push(Argument::new(
            sig.inputs.pop().unwrap().into_value(),
            chat_command
        )?);
    }

    arguments.reverse();

    let (names, types, renames) = (
        arguments.iter().map(|s| &s.ident).collect::<Vec<_>>(),
        arguments.iter().map(|s| &s.ty).collect::<Vec<_>>(),
        arguments
            .iter()
            .map(|s| {
                if let Some(renaming) = s.attributes.as_ref().map(|a| a.renaming.as_ref()).flatten() {
                    renaming.inner().clone()
                } else {
                    s.ident.to_string()
                }
            })
            .collect::<Vec<_>>(),
    );

    if !names.is_empty() {
        // The original block of the function
        let b = &block;

        // Modify the block to parse arguments
        *block = parse2(quote::quote! {{
            let (#(#names),*) = {
                let mut __options = ::vesper::iter::DataIterator::new(#ctx_ident);

                #(let #names =
                    __options.named_parse::<#types>(#renames).await?;)*

                if __options.len() > 0 {
                    return Err(
                        ::vesper::prelude::ParseError::StructureMismatch("Too many arguments received".to_string()).into()
                    );
                }

                (#(#names),*)
            };

            #b
        }})?;
    }

    Ok(arguments)
}


/// Gets the identifier and the type of the first argument of a function, which must be an
/// `SlashContext`
pub fn get_context_type_and_ident(sig: &Signature) -> Result<(Ident, Type)> {
    let arg = match sig.inputs.iter().next() {
        None => {
            return Err(Error::new(
                sig.inputs.span(),
                "Expected SlashContext as first parameter",
            ))
        }
        Some(c) => c,
    };

    let ctx_ident = util::get_ident(&util::get_pat(arg)?.pat)?;

    let ty = util::get_bracketed_generic(arg, true, |ty| {
        if let Type::Infer(_) = ty {
            Err(Error::new(
                sig.inputs.span(),
                "SlashContext must have a known type",
            ))
        } else {
            Ok(ty.clone())
        }
    })?;

    let ty = match ty {
        None => Err(Error::new(arg.span(), "SlashContext type must be set")),
        Some(ty) => Ok(ty),
    }?;

    Ok((ctx_ident, ty))
}
