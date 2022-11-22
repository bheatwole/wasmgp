use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;
use syn::*;

/// The state used for wasm_code is either an empty tuple () or a specific named type
#[derive(Clone)]
pub enum StateType {
    Empty,
    Named(Ident),
}

impl StateType {
    pub fn empty() -> StateType {
        StateType::Empty
    }

    pub fn named(ident: Ident) -> StateType {
        StateType::Named(ident)
    }

    pub fn from_generics(generics: &Generics) -> Result<StateType> {
        // If there are no generics, return the empty type
        if generics.params.is_empty() {
            return Ok(StateType::empty());
        }

        // Where clauses are not handled
        if let Some(where_clause) = &generics.where_clause {
            return Err(Error::new(
                where_clause.span(),
                "where clause is not supported",
            ));
        }

        // More than one generic parameter is not handled
        if generics.params.len() > 1 {
            return Err(Error::new(
                generics.params.span(),
                "maximum of one state parameter",
            ));
        }

        // We checked for empty, so we should be able to get the first
        let param = generics.params.first().unwrap();

        // It must be a type param
        let type_param = match param {
            GenericParam::Type(tp) => tp,
            _ => {
                return Err(Error::new(
                    param.span(),
                    "generic parameter cannot contain liftime or const information",
                ))
            }
        };

        if type_param.attrs.len() > 0 || type_param.bounds.len() > 0 || type_param.default.is_some()
        {
            return Err(Error::new(
                type_param.span(),
                "generic parameter must be plain Ident",
            ));
        }

        return Ok(StateType::named(type_param.ident.clone()));
    }

    pub fn for_fn_args(&self) -> StateTypeFnArgs {
        StateTypeFnArgs {
            state_type: self.clone(),
        }
    }

    pub fn for_store_arg(&self) -> StateTypeStoreArg {
        StateTypeStoreArg {
            state_type: self.clone(),
        }
    }
}

impl ToTokens for StateType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            StateType::Empty => tokens.append(TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                TokenStream::new(),
            ))),
            StateType::Named(ident) => ident.to_tokens(tokens),
        }
    }
}

/// Allows turning StateType into the args for `fn new(state: StateType)`
pub struct StateTypeFnArgs {
    state_type: StateType,
}

impl ToTokens for StateTypeFnArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.state_type {
            StateType::Empty => {}
            StateType::Named(ident) => tokens.extend(quote!(state: #ident)),
        }
    }
}

/// Allows turning StateType into the args for `Store::new(&engine, StateType)`
pub struct StateTypeStoreArg {
    state_type: StateType,
}

impl ToTokens for StateTypeStoreArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.state_type {
            StateType::Empty => tokens.extend(quote!(())),
            StateType::Named(_) => tokens.extend(quote!(state)),
        }
    }
}
