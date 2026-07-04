#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Repeat {
    pub(crate) min: u32,
    pub(crate) max: Option<u32>,
}

#[derive(Debug)]
pub(crate) enum Atom {
    Start,
    End,
    Digit,
    Literal(char),
    WordChar,
    CharGroup { chars: Vec<char>, negated: bool },
    CapturingGroup,
}

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) atom: Atom,
    pub(crate) repeat: Repeat,
}

impl Node {
    pub(crate) fn new(atom: Atom) -> Self {
        Self {
            atom,
            repeat: Repeat {
                min: 1,
                max: Some(1),
            },
        }
    }
}
