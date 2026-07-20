use std::collections::HashMap;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Element(String),
    Digit(i32),
}
impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '(' => Self::Open,
            ')' => Self::Close,
            _ => panic!("Token constructed from invalid char"),
        }
    }
}
impl From<&mut Vec<char>> for Token {
    fn from(value: &mut Vec<char>) -> Self {
        match value[0] {
            c if c.is_ascii_digit() => Self::Digit(
                value.drain(..)
                    .rev()
                    .enumerate()
                    .fold(0_i32, |res, (i, c)| {
                        res + c.to_digit(10).unwrap() as i32 * 10_i32.pow(i as u32)
                    })
            ),
            c => Self::Element(value.drain(..).collect())
        }
    }
}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let tokens = Self::formula_to_tokens(formula);
        let counts = Self::count_els(&mut tokens.into_iter().peekable());
        let mut counts: Vec<(String, i32)> = counts.into_iter().collect();
        counts.sort_unstable();
        counts.into_iter()
            .map(|(mut s, c)| if c == 1 {
                s
            } else {
                s.push_str(&c.to_string());
                s
            })
            .collect()
    }

    fn formula_to_tokens(formula: String) -> Vec<Token> {
        let mut s = Vec::new();
        let mut tokens = Vec::new();
        for c in formula.chars() {
            match c {
                '(' | ')' => {
                    if !s.is_empty() {
                        tokens.push(Token::from(&mut s));
                    }
                    tokens.push(Token::from(c));
                }
                c if c.is_ascii_digit() => {
                    if s.last().is_some_and(|l| !l.is_ascii_digit()) {
                        tokens.push(Token::from(&mut s));
                    }
                    s.push(c);
                }
                c if c.is_ascii_uppercase() => {
                    if !s.is_empty() {
                        tokens.push(Token::from(&mut s));
                    }
                    s.push(c);
                }
                c => s.push(c),
            }
        }
        if !s.is_empty() {
            tokens.push(Token::from(&mut s));
        }
        println!("{:?}", tokens);
        tokens
    }

    fn count_els(tokens: &mut Peekable<IntoIter<Token>>) -> HashMap<String, i32> {
        let mut counts = HashMap::new();
        while let Some(t) = tokens.next() {
            match t {
                Token::Open => {
                    let inner_count = Self::count_els(tokens);
                    let d = if let Some(Token::Digit(n)) = tokens.peek() {
                        let n = *n;
                        tokens.next();
                        n
                    } else {
                        1
                    };
                    for (e, n) in inner_count.into_iter() {
                        counts.entry(e)
                            .and_modify(|c| *c += n * d)
                            .or_insert(n * d);
                    }
                }
                Token::Close => break,
                Token::Element(s) => {
                    let d = if let Some(Token::Digit(n)) = tokens.peek() {
                        let n = *n;
                        tokens.next();
                        n
                    } else {
                        1
                    };
                    counts.entry(s)
                        .and_modify(|n| *n += d)
                        .or_insert(d);
                }
                Token::Digit(n) => unreachable!(),
            }
        }
        println!("{:?}", counts);
        counts
    }
}