#[derive(Debug)]
pub(crate) enum Token {
    Start,
    End,
    Digit,
    Literal(char),
    WordChar,
    CharGroup { chars: Vec<char>, negated: bool },
    CapturingGroup,
}
