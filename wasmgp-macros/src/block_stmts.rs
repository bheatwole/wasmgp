use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::*;

/// Prints just the statements of a Block (excluding the surrounding braces {})
pub struct BlockStmts<'a> {
    block: &'a Box<Block>,
}

impl<'a> BlockStmts<'a> {
    pub fn new(block: &Box<Block>) -> BlockStmts {
        BlockStmts { block }
    }
}

impl<'a> ToTokens for BlockStmts<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for stmt in self.block.stmts.iter() {
            stmt.to_tokens(tokens);
        }
    }
}
