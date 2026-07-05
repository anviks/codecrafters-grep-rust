use crate::token::{Atom, Node, Repeat};

fn match_repeat(
    atom: &Atom,
    repeat: &Repeat,
    rest: &[Node],
    text: &[char],
    pos: usize,
) -> Option<usize> {
    let mut matched: usize = 0;
    while matched < repeat.max && pos + matched < text.len() && atom.matches(text[pos + matched]) {
        matched += 1;
    }

    if matched < repeat.min as usize {
        return None;
    }

    loop {
        if let Some(n) = match_here(rest, text, pos + matched) {
            break Some(n);
        }
        if matched == repeat.min as usize {
            break None;
        }
        matched -= 1;
    }
}

fn match_here(nodes: &[Node], text: &[char], pos: usize) -> Option<usize> {
    let Some((node, rest)) = nodes.split_first() else {
        return Some(pos);
    };

    match &node.atom {
        Atom::Start => {
            if pos == 0 {
                match_here(rest, text, pos)
            } else {
                None
            }
        }
        Atom::End => {
            if pos == text.len() {
                match_here(rest, text, pos)
            } else {
                None
            }
        }
        Atom::Group { alternatives } => {
            for alt in alternatives {
                if let Some(p) = match_here(alt, text, pos)
                    && let Some(p_rest) = match_here(rest, text, p)
                {
                    return Some(p_rest);
                }
            }

            None
        }
        atom => match_repeat(atom, &node.repeat, rest, text, pos),
    }
}

pub(crate) fn match_pattern(chars: &Vec<char>, pattern: &Vec<Node>) -> Vec<(usize, usize)> {
    let mut results = vec![];

    let mut start = 0;
    while start <= chars.len() {
        let matches = match_here(pattern, chars, start);
        if let Some(n) = matches {
            results.push((start, n));
            start = n;
        } else {
            start += 1;
        }
    }

    results
}
