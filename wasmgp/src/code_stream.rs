use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CodeStream {
    Simple(Code),
    Begin(Code),
    End,
}

impl CodeStream {
    pub fn to_stream<'a, I>(code: I) -> Vec<CodeStream>
    where
        I: IntoIterator<Item = &'a Code>,
    {
        let mut stream = vec![];
        for item in code.into_iter() {
            match item {
                Code::If(instructions) => {
                    stream.push(CodeStream::Begin(If::new(instructions.if_not_zero(), vec![])));
                    let mut branch = CodeStream::to_stream(instructions.do_this());
                    stream.append(&mut branch);
                    stream.push(CodeStream::End);
                }
                Code::IfElse(instructions) => {
                    stream.push(CodeStream::Begin(IfElse::new(
                        instructions.if_not_zero(),
                        vec![],
                        vec![],
                    )));
                    let mut branch = CodeStream::to_stream(instructions.do_this());
                    stream.append(&mut branch);
                    stream.push(CodeStream::End);
                    let mut branch = CodeStream::to_stream(instructions.else_do_this());
                    stream.append(&mut branch);
                    stream.push(CodeStream::End);
                }
                Code::DoUntil(instructions) => {
                    stream.push(CodeStream::Begin(DoUntil::new(instructions.until_not_zero(), vec![])));
                    let mut branch = CodeStream::to_stream(instructions.do_this());
                    stream.append(&mut branch);
                    stream.push(CodeStream::End);
                }
                Code::DoWhile(instructions) => {
                    stream.push(CodeStream::Begin(DoWhile::new(instructions.while_not_zero(), vec![])));
                    let mut branch = CodeStream::to_stream(instructions.do_this());
                    stream.append(&mut branch);
                    stream.push(CodeStream::End);
                }
                Code::DoFor(instructions) => {
                    stream.push(CodeStream::Begin(DoFor::new(instructions.times(), vec![])));
                    let mut branch = CodeStream::to_stream(instructions.do_this());
                    stream.append(&mut branch);
                    stream.push(CodeStream::End);
                }
                _ => stream.push(CodeStream::Simple(item.clone())),
            }
        }
        stream
    }

    pub fn from_stream(stream: &mut std::vec::IntoIter<CodeStream>) -> Vec<Code> {
        let mut code = vec![];
        while let Some(item) = stream.next() {
            match item {
                CodeStream::Simple(c) => code.push(c.clone()),
                CodeStream::Begin(c) => match c {
                    Code::If(instructions) => {
                        let branch = CodeStream::from_stream(stream);
                        code.push(If::new(instructions.if_not_zero(), branch));
                    }
                    Code::IfElse(instructions) => {
                        let if_branch = CodeStream::from_stream(stream);
                        let else_branch = CodeStream::from_stream(stream);
                        code.push(IfElse::new(instructions.if_not_zero(), if_branch, else_branch));
                    }
                    Code::DoUntil(instructions) => {
                        let branch = CodeStream::from_stream(stream);
                        code.push(DoUntil::new(instructions.until_not_zero(), branch));
                    }
                    Code::DoWhile(instructions) => {
                        let branch = CodeStream::from_stream(stream);
                        code.push(DoWhile::new(instructions.while_not_zero(), branch));
                    }
                    Code::DoFor(instructions) => {
                        let branch = CodeStream::from_stream(stream);
                        code.push(DoFor::new(instructions.times(), branch));
                    }
                    _ => {
                        panic!("getting here indicates a logical flaw in the code")
                    }
                },
                CodeStream::End => {
                    return code;
                }
            }
        }
        code
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_to_stream_if_else() {
        let expected = vec![
            CodeStream::Begin(IfElse::new(5, vec![], vec![])),
            CodeStream::Simple(Add::new(0, 1, 2)),
            CodeStream::End,
            CodeStream::Simple(Add::new(2, 1, 0)),
            CodeStream::End,
        ];
        let code = vec![IfElse::new(5, vec![Add::new(0, 1, 2)], vec![Add::new(2, 1, 0)])];

        assert_eq!(CodeStream::to_stream(&code), expected);
    }

    #[test]
    fn test_from_stream_if_else() {
        // Without the 'End' elements, everything gets lumped into the If branch
        let stream = vec![
            CodeStream::Begin(IfElse::new(5, vec![], vec![])),
            CodeStream::Simple(Add::new(0, 1, 2)),
            CodeStream::Simple(Add::new(2, 1, 0)),
        ];
        let expected = vec![IfElse::new(5, vec![Add::new(0, 1, 2), Add::new(2, 1, 0)], vec![])];

        assert_eq!(CodeStream::from_stream(&mut stream.into_iter()), expected);
    }
}
