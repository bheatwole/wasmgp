use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
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

    pub fn for_value_types<'a>(&'a self, crate_path: &Path) -> VarListTypeValueTypes<'a> {
        VarListTypeValueTypes {
            crate_path: crate_path.clone(),
            list: self,
        }
    }

    pub fn for_call_fn_args<'a>(&'a self) -> VarListTypeCallFnArgs<'a> {
        VarListTypeCallFnArgs { list: self }
    }

    pub fn for_call_args<'a>(&'a self) -> VarListTypeCallArgs<'a> {
        VarListTypeCallArgs { list: self }
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
        tokens.extend(quote!((#contents)));
    }
}

pub struct VarListTypeValueTypes<'a> {
    crate_path: Path,
    list: &'a VarListType,
}

impl<'a> ToTokens for VarListTypeValueTypes<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = Span::call_site();
        let crate_path = &self.crate_path;

        // The inner part of the list is a punctuated list of wasmgp::ValueType::?
        let mut contents: Punctuated<_, Token!(,)> = Punctuated::new();
        for var in self.list.vars.iter() {
            let vt = match var.to_string().as_str() {
                "i32" | "u32" => Ident::new("I32", span.clone()),
                "i64" | "u64" => Ident::new("I64", span.clone()),
                "f32" => Ident::new("F32", span.clone()),
                "f64" => Ident::new("F64", span.clone()),
                _ => panic!("'var' should always be one of [i32, i64, u32, u64, f32, f64]"),
            };
            contents.push(quote! {
                #crate_path::ValueType::#vt
            });
        }
        tokens.extend(quote!(vec![#contents]));
    }
}

/// Prints the var_list formatted for the arguments of `fn call(&self, v1: u32, v2: i32)`
pub struct VarListTypeCallFnArgs<'a> {
    list: &'a VarListType,
}

impl<'a> ToTokens for VarListTypeCallFnArgs<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (i, var) in self.list.vars.iter().enumerate() {
            let var_name: Ident = syn::parse_str::<Ident>(&format!("v{}", i + 1)).unwrap();
            tokens.extend(quote!(, #var_name: #var));
        }
    }
}

/// Prints the var_list formatted for the arguments of `self.func.call(store.deref_mut(), (v1, v2))?;`
pub struct VarListTypeCallArgs<'a> {
    list: &'a VarListType,
}

impl<'a> ToTokens for VarListTypeCallArgs<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut contents: Punctuated<_, Token!(,)> = Punctuated::new();
        for i in 0..self.list.vars.len() {
            let var_name: Ident = syn::parse_str::<Ident>(&format!("v{}", i + 1)).unwrap();
            contents.push(quote!(#var_name));
        }

        tokens.extend(quote!((#contents)));
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
