use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::*;

pub struct SlotCount {
    pub is_signed: bool,
    pub slot_counts: Vec<u8>,
}

impl SlotCount {
    pub fn for_constructor<'a>(&'a self, crate_path: &Path) -> SlotCountConstructor<'a> {
        SlotCountConstructor {
            crate_path: crate_path.clone(),
            slot_count: self,
        }
    }
}

impl Parse for SlotCount {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut slot_counts = vec![];
        let mut is_signed = false;

        if !input.is_empty() {
            let flag: Ident = input.parse()?;
            is_signed = flag == "signed" || flag == "s" || flag == "i";
            if !is_signed {
                if !(flag == "unsigned" || flag == "u") {
                    return Err(Error::new(
                        flag.span(),
                        "expected one of (`signed`, `s`, `i`, `unsigned`, `u`)",
                    ));
                }
            }

            if !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
            }
        }

        while !input.is_empty() {
            let lit: LitInt = input.parse()?;
            let value = lit.base10_parse::<u8>()?;
            slot_counts.push(value);

            if input.is_empty() || slot_counts.len() == 4 {
                break;
            }
            let _comma: Token![,] = input.parse()?;
        }

        if !input.is_empty() {
            return Err(Error::new(input.span(), "maximum of four slots"));
        }

        Ok(SlotCount {
            is_signed,
            slot_counts,
        })
    }
}

impl ToTokens for SlotCount {
    // This one simply outputs `true` or `false` for is_signed
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = LitBool::new(self.is_signed, Span::call_site());
        value.to_tokens(tokens);
    }
}

pub struct SlotCountConstructor<'a> {
    crate_path: Path,
    slot_count: &'a SlotCount,
}

impl<'a> SlotCountConstructor<'a> {
    fn literal_for_slot(&self, index: usize) -> TokenTree {
        let value = if let Some(v) = self.slot_count.slot_counts.get(index) {
            *v
        } else {
            0
        };

        TokenTree::Literal(Literal::u8_unsuffixed(value))
    }
}

impl<'a> ToTokens for SlotCountConstructor<'a> {
    // #crate_path::SlotCount { i32: #, ... }
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lit_i32 = self.literal_for_slot(0);
        let lit_i64 = self.literal_for_slot(1);
        let lit_f32 = self.literal_for_slot(2);
        let lit_f64 = self.literal_for_slot(3);
        let path = &self.crate_path;

        let token_stream = quote!(#path::SlotCount {
            i32: #lit_i32,
            i64: #lit_i64,
            f32: #lit_f32,
            f64: #lit_f64,
        });
        tokens.extend(token_stream);
    }
}
