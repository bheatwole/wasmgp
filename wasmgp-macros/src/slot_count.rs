use syn::parse::{Parse, ParseStream};
use syn::*;

pub struct SlotCount {
    pub is_signed: bool,
    pub slot_counts: Vec<u8>,
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

            if input.is_empty() {
                break;
            }
            let _comma: Token![,] = input.parse()?;
        }

        Ok(SlotCount {
            is_signed,
            slot_counts,
        })
    }
}
