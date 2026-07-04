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
    WildCard,
    CharGroup { chars: Vec<char>, negated: bool },
    Group { alternatives: Vec<Vec<Node>> },
}

impl Atom {
    pub(crate) fn matches(&self, char: char) -> bool {
        match &self {
            Atom::Start | Atom::End | Atom::Group { alternatives: _ } => false,
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
