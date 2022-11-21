use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

/// The VarListType is used when passing parameters to wasm_code or getting results. It can be represented differently
/// depending upon how many variables there are and where they are to be used
pub struct VarListType {
    vars: Vec<Ident>, // each Ident must be one of [i32, i64, u32, u64, f32, f64]
}

impl VarListType {
    pub fn from_fn_args(item_fn: &ItemFn) -> Result<VarListType> {
        let mut vars = vec![];

        for input in item_fn.sig.inputs.iter() {
            match input {
                FnArg::Typed(pat_type) => {
                    read_vars_from_type(pat_type.ty.as_ref(), &mut vars, false)?
                }
                _ => {
                    return Err(Error::new(
                        input.span(),
                        "all arguments must be one of [i32, i64, u32, u64, f32, f64]",
                    ))
                }
            }
        }

        Ok(VarListType { vars })
    }

    pub fn from_fn_results(item_fn: &ItemFn) -> Result<VarListType> {
        let mut vars = vec![];

        match &item_fn.sig.output {
            ReturnType::Default => {}
            ReturnType::Type(_, box_type) => {
                read_vars_from_type(box_type.as_ref(), &mut vars, true)?
            }
        }

        Ok(VarListType { vars })
    }

    pub fn for_generic_params<'a>(&'a self) -> VarListTypeGenericParams<'a> {
        VarListTypeGenericParams { list: self }
    }
}

pub struct VarListTypeGenericParams<'a> {
    list: &'a VarListType,
}

impl<'a> ToTokens for VarListTypeGenericParams<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // If there is exactly one variable, it is output without any grouping
        if self.list.vars.len() == 1 {
            let v = self.list.vars.first().unwrap();
            tokens.append(v.clone());
            return;
        }

        // In all other cases, (including zero vars) the variable list looks like a tuple.
        let mut contents: Punctuated<Ident, Token!(,)> = Punctuated::new();
        for var in self.list.vars.iter() {
            contents.push(var.clone());
        }

        // Make a new token stream with the contents of the tuple
        let mut inner_ts = TokenStream::new();
        contents.to_tokens(&mut inner_ts);

        // Make a parenthesized group from the inner token stream
        tokens.append(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            inner_ts,
        )))
    }
}

fn read_vars_from_type(ty: &Type, vars: &mut Vec<Ident>, is_tuple_allowed: bool) -> Result<()> {
    match ty {
        Type::Path(path) => read_vars_from_type_path(path, vars),
        Type::Tuple(tuple) => {
            if is_tuple_allowed {
                for elem in tuple.elems.iter() {
                    read_vars_from_type(elem, vars, false)?;
                }
                Ok(())
            } else {
                Err(Error::new(
                    ty.span(),
                    "invalid type: must be one of [i32, i64, u32, u64, f32, f64]",
                ))
            }
        }
        _ => Err(Error::new(
            ty.span(),
            "invalid type: must be one of [i32, i64, u32, u64, f32, f64]",
        )),
    }
}

fn read_vars_from_type_path(path: &TypePath, vars: &mut Vec<Ident>) -> Result<()> {
    if path.path.segments.len() != 1 {
        return Err(Error::new(
            path.span(),
            "invalid type: must be one of [i32, i64, u32, u64, f32, f64]",
        ));
    }

    let segment = path.path.segments.first().unwrap();
    match segment.arguments {
        PathArguments::None => {}
        _ => {
            return Err(Error::new(
                segment.span(),
                "invalid type: must be one of [i32, i64, u32, u64, f32, f64]",
            ));
        }
    }

    let ident = segment.ident.clone();
    if ident == "i32"
        || ident == "i64"
        || ident == "u32"
        || ident == "u64"
        || ident == "f32"
        || ident == "f64"
    {
        vars.push(ident);
    } else {
        return Err(Error::new(
            segment.span(),
            "invalid type: must be one of [i32, i64, u32, u64, f32, f64]",
        ));
    }

    Ok(())
}