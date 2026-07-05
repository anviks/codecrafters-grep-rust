use crate::token::Atom::BackReference;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Repeat {
    pub(crate) min: usize,
    pub(crate) max: usize,
}

#[derive(Debug, Clone)]
pub(crate) enum Atom {
    Start,
    End,
    Digit,
    Literal(char),
    WordChar,
    WildCard,
    CharGroup {
        chars: Vec<char>,
        negated: bool,
    },
    Group {
        index: usize,
        alternatives: Vec<Vec<Node>>,
    },
    BackReference(usize),
}

impl Atom {
    pub(crate) fn matches(&self, char: char) -> bool {
        match &self {
            Atom::Start
            | Atom::End
            | Atom::Group {
                alternatives: _,
                index: _,
            }
            | BackReference(_) => false,
            Atom::Digit => char.is_digit(10),
            Atom::Literal(c) => char == *c,
            Atom::WordChar => char.is_ascii_alphanumeric() || char == '_',
            Atom::CharGroup { chars, negated } => {
                !*negated && chars.contains(&char) || *negated && !chars.contains(&char)
            }
            Atom::WildCard => true,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub(crate) atom: Atom,
    pub(crate) repeat: Repeat,
}

impl Node {
    pub(crate) fn new(atom: Atom) -> Self {
        Self {
            atom,
            repeat: Repeat { min: 1, max: 1 },
        }
    }
}
