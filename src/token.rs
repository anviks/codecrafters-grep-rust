#[derive(Debug)]
pub(crate) enum Token {
    Start,
    End,
    Digit,
    Literal(String),
    WordChar,
    CharGroup { chars: Vec<char>, negated: bool },
    CapturingGroup,
}
