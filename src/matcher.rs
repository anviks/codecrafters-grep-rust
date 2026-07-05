use crate::token::{Atom, Node, Repeat};

fn match_repeat(
    atom: &Atom,
    repeat: &Repeat,
    rest: &[Node],
    text: &[char],
    pos: usize,
    captures: &mut Vec<Option<(usize, usize)>>,
) -> Option<usize> {
    let mut matched: usize = 0;
    while matched < repeat.max && pos + matched < text.len() && atom.matches(text[pos + matched]) {
        matched += 1;
    }

    if matched < repeat.min as usize {
        return None;
    }

    loop {
        if let Some(n) = match_here(rest, text, pos + matched, captures) {
            break Some(n);
        }
        if matched == repeat.min as usize {
            break None;
        }
        matched -= 1;
    }
}

fn match_here(
    nodes: &[Node],
    text: &[char],
    pos: usize,
    captures: &mut Vec<Option<(usize, usize)>>,
) -> Option<usize> {
    let Some((node, rest)) = nodes.split_first() else {
        return Some(pos);
    };

    match &node.atom {
        Atom::Start => {
            if pos == 0 {
                match_here(rest, text, pos, captures)
            } else {
                None
            }
        }
        Atom::End => {
            if pos == text.len() {
                match_here(rest, text, pos, captures)
            } else {
                None
            }
        }
        Atom::Group {
            alternatives,
            index,
        } => {
            for alt in alternatives {
                if let Some(p) = match_here(alt, text, pos, captures) {
                    if *index > 0 {
                        captures[*index - 1] = Some((pos, p));
                    }
                    let mut combined = alt.clone();
                    combined.extend_from_slice(rest);
                    if let Some(p_rest) = match_here(&combined, text, pos, captures) {
                        return Some(p_rest);
                    }
                }
            }

            None
        }
        Atom::BackReference(index) => {
            let (start, end) = match captures[*index - 1] {
                Some(m) => m,
                None => {
                    eprintln!("grep: Invalid back reference");
                    return None;
                }
            };

            if &text[start..end] == &text[pos..pos + end - start]
                && let Some(p_rest) = match_here(rest, text, pos + end - start, captures)
            {
                return Some(p_rest);
            }

            None
        }
        atom => match_repeat(atom, &node.repeat, rest, text, pos, captures),
    }
}

pub(crate) fn match_pattern(
    chars: &Vec<char>,
    pattern: &Vec<Node>,
    group_count: usize,
) -> Vec<(usize, usize)> {
    let mut results = vec![];

    let mut start = 0;
    while start <= chars.len() {
        let mut captures: Vec<Option<(usize, usize)>> = vec![None; group_count];
        let matches = match_here(pattern, chars, start, &mut captures);
        if let Some(n) = matches {
            results.push((start, n));
            start = n;
        } else {
            start += 1;
        }
    }

    results
}
