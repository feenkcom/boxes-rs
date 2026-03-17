use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::{
    Error, FnArg, ItemFn, Pat, PatIdent, PathArguments, ReturnType, Token, Type, TypePath,
    TypeReference, Visibility,
};

struct WrapperArg {
    ident: syn::Ident,
    mutable: bool,
}

struct OwnedArg {
    ident: syn::Ident,
}

#[proc_macro_attribute]
pub fn ffi(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);

    match expand_ffi(function) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_ffi(function: ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    if function.sig.asyncness.is_some() {
        return Err(Error::new_spanned(
            function.sig.asyncness,
            "#[value_box_derive::ffi] does not support async functions",
        ));
    }

    if function.sig.constness.is_some() {
        return Err(Error::new_spanned(
            function.sig.constness,
            "#[value_box_derive::ffi] does not support const functions",
        ));
    }

    if function.sig.abi.is_some() {
        return Err(Error::new_spanned(
            &function.sig.abi,
            "#[value_box_derive::ffi] expects a Rust function without an existing ABI",
        ));
    }

    if !matches!(function.sig.output, ReturnType::Default) {
        return Err(Error::new_spanned(
            &function.sig.output,
            "#[value_box_derive::ffi] currently supports only functions returning ()",
        ));
    }

    if !matches!(function.vis, Visibility::Public(_)) {
        return Err(Error::new_spanned(
            &function.vis,
            "#[value_box_derive::ffi] expects a public function",
        ));
    }

    let mut transformed_inputs = Punctuated::<FnArg, Token![,]>::new();
    let mut wrapper_args = Vec::<WrapperArg>::new();
    let mut owned_args = Vec::<OwnedArg>::new();

    for input in function.sig.inputs.iter() {
        match input {
            FnArg::Receiver(receiver) => {
                return Err(Error::new_spanned(
                    receiver,
                    "#[value_box_derive::ffi] does not support methods",
                ));
            }
            FnArg::Typed(argument) => {
                let pattern = match argument.pat.as_ref() {
                    Pat::Ident(PatIdent { ident, .. }) => ident.clone(),
                    other => {
                        return Err(Error::new_spanned(
                            other,
                            "#[value_box_derive::ffi] expects simple identifier arguments",
                        ));
                    }
                };

                if let Type::Reference(TypeReference {
                    elem,
                    mutability,
                    ..
                }) = argument.ty.as_ref()
                {
                    let arg_mutability = mutability.as_ref();
                    transformed_inputs.push(syn::parse_quote! {
                        #arg_mutability #pattern: ::value_box::BorrowedPtr<#elem>
                    });
                    wrapper_args.push(WrapperArg {
                        ident: pattern,
                        mutable: mutability.is_some(),
                    });
                } else if is_owned_ptr_type(argument.ty.as_ref()) {
                    let owned_type = argument.ty.as_ref();
                    transformed_inputs.push(syn::parse_quote! {
                        #pattern: ::value_box::OwnedPtr<#owned_type>
                    });
                    owned_args.push(OwnedArg { ident: pattern });
                } else {
                    transformed_inputs.push(FnArg::Typed(argument.clone()));
                }
            }
        }
    }

    let attrs = &function.attrs;
    let vis = &function.vis;
    let ident = &function.sig.ident;
    let wrapped_body = wrap_body(&wrapper_args, &function.block);
    let owned_arg_uses = owned_args.iter().map(|arg| {
        let ident = &arg.ident;
        quote! { let _ = &#ident; }
    });
    let body = if wrapper_args.is_empty() {
        quote! {
            #(#owned_arg_uses)*
            #wrapped_body
        }
    } else {
        quote! {
            use ::value_box::ReturnBoxerResult as _;
            #(#owned_arg_uses)*
            #wrapped_body.log();
        }
    };

    Ok(quote! {
        #(#attrs)*
        #[unsafe(no_mangle)]
        #vis extern "C" fn #ident(#transformed_inputs) {
            #body
        }
    })
}

fn wrap_body(wrapper_args: &[WrapperArg], body: &syn::Block) -> proc_macro2::TokenStream {
    if wrapper_args.is_empty() {
        return quote! { #body };
    }

    let mut current = proc_macro2::TokenStream::new();

    for (index, arg) in wrapper_args.iter().enumerate().rev() {
        let ident = &arg.ident;
        let binding = ident;
        let innermost = index == wrapper_args.len() - 1;

        current = match (arg.mutable, innermost) {
            (false, true) => quote! {
                #ident.with_ref_ok(|#binding| #body)
            },
            (true, true) => quote! {
                #ident.with_mut_ok(|#binding| #body)
            },
            (false, false) => quote! {
                #ident.with_ref(|#binding| { #current })
            },
            (true, false) => quote! {
                #ident.with_mut(|#binding| { #current })
            },
        };
    }

    current
}

fn is_owned_ptr_type(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath { qself: None, path }) => {
            let Some(segment) = path.segments.last() else {
                return false;
            };

            if segment.ident == "Box" && matches!(segment.arguments, PathArguments::AngleBracketed(_))
            {
                return true;
            }

            !is_primitive_ident(&segment.ident)
        }
        _ => false,
    }
}

fn is_primitive_ident(ident: &syn::Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        "bool"
            | "char"
            | "str"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "f32"
            | "f64"
    )
}
