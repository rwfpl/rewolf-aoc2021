use std::{fs, iter};

#[derive(Debug, Clone, Copy)]
enum Token {
    Open,
    Close,
    Comma,
    Value(u32),
}

#[derive(Debug, Clone)]
struct SnailFishNumber {
    q: Vec<Token>,
}

impl From<&str> for SnailFishNumber {
    fn from(value: &str) -> Self {
        Self {
            q: value
                .chars()
                .map(|c| match c {
                    '[' => Token::Open,
                    ']' => Token::Close,
                    ',' => Token::Comma,
                    '0'..='9' => Token::Value(c as u32 - 0x30),
                    _ => panic!("invalid character {c}"),
                })
                .collect(),
        }
    }
}

impl ToString for SnailFishNumber {
    fn to_string(&self) -> String {
        self.q
            .iter()
            .map(|t| match t {
                Token::Open => "[".to_string(),
                Token::Close => "]".to_string(),
                Token::Comma => ",".to_string(),
                Token::Value(v) => v.to_string(),
            })
            .collect()
    }
}

impl SnailFishNumber {
    fn pop_value(&self, q: &mut Vec<Token>) -> u32 {
        if let Token::Value(rval) = q.pop().unwrap() {
            rval
        } else {
            panic!("why u no Token::Value ლ(ಠ益ಠლ)")
        }
    }

    fn pop_pair(&self, q: &mut Vec<Token>) -> (u32, u32) {
        q.pop(); // close
        let rval = self.pop_value(q);
        q.pop(); // comma
        let lval = self.pop_value(q);
        q.pop(); // open
        (lval, rval)
    }

    fn explode(&mut self) -> &mut Self {
        let mut q: Vec<Token> = Vec::new();
        let mut nest_level = 0;
        let mut comma = false;
        let mut explode_pair = false;
        let mut exploded = false;
        let mut rval_to_add = 0;
        self.q.iter().for_each(|t| match t {
            Token::Open => {
                nest_level += 1;
                comma = false;
                q.push(Token::Open);
            }
            Token::Close => {
                nest_level -= 1;
                comma = false;
                q.push(Token::Close);
                if explode_pair {
                    explode_pair = false;
                    exploded = true;
                    let (lval, rval) = self.pop_pair(&mut q);
                    rval_to_add = rval;
                    if let Some(ll) = q.iter_mut().rev().find(|t| matches!(t, Token::Value(_))) {
                        if let Token::Value(found) = ll {
                            *ll = Token::Value(*found + lval)
                        }
                    }
                    q.push(Token::Value(0));
                }
            }
            Token::Value(v) => {
                if nest_level >= 5 && comma && !exploded {
                    explode_pair = true;
                }
                comma = false;
                q.push(Token::Value(*v + rval_to_add));
                rval_to_add = 0;
            }
            Token::Comma => {
                comma = true;
                q.push(Token::Comma);
            }
        });
        self.q = q;
        self
    }

    fn split(&mut self) -> &mut Self {
        let mut q: Vec<Token> = Vec::new();
        let mut split = false;
        self.q.iter().for_each(|t| match t {
            Token::Open => {
                q.push(Token::Open);
            }
            Token::Close => {
                q.push(Token::Close);
            }
            Token::Value(v) => {
                if *v >= 10 && !split {
                    split = true;
                    q.push(Token::Open);
                    q.push(Token::Value(v / 2));
                    q.push(Token::Comma);
                    q.push(Token::Value(v - (v / 2)));
                    q.push(Token::Close);
                } else {
                    q.push(Token::Value(*v));
                }
            }
            Token::Comma => {
                q.push(Token::Comma);
            }
        });
        self.q = q;
        self
    }

    fn magnitude(&self) -> u32 {
        let mut q: Vec<Token> = Vec::new();
        let mut comma = false;
        let mut calc = false;
        while q.len() != 1 {
            self.q.iter().for_each(|t| match t {
                Token::Open => {
                    q.push(Token::Open);
                    comma = false;
                }
                Token::Close => {
                    q.push(Token::Close);
                    comma = false;
                    if calc {
                        let (lval, rval) = self.pop_pair(&mut q);
                        q.push(Token::Value(2 * rval + 3 * lval));
                    }
                }
                Token::Value(v) => {
                    q.push(Token::Value(*v));
                    if comma {
                        calc = true;
                    }
                    comma = false;
                }
                Token::Comma => {
                    q.push(Token::Comma);
                    comma = true;
                }
            });
        }
        self.pop_value(&mut q)
    }

    fn add(&mut self, other: &Self) -> &mut Self {
        if self.q.is_empty() {
            self.q = other.q.clone();
        } else {
            self.q = iter::once(&Token::Open)
            .chain(self.q.iter())
            .chain(iter::once(&Token::Comma))
            .chain(other.q.iter())
            .chain(iter::once(&Token::Close))
            .cloned()
            .collect::<Vec<Token>>();
        }
        self
    }

    fn normalize(&mut self) -> &mut Self {
        let mut plen = self.q.len();
        loop {
            self.explode();
            if self.q.len() != plen {
                plen = self.q.len();
                continue;
            }
            self.split();
            if self.q.len() != plen {
                plen = self.q.len();
                continue;
            }
            break;
        }
        self
    }
}

fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(SnailFishNumber::from)
        .fold(SnailFishNumber::from(""), |mut acc, v| {
            acc.add(&v).normalize().to_owned()
        })
        .normalize()
        .magnitude()
}

fn solution2(input: &str) -> u32 {
    let ns = input
        .lines()
        .map(SnailFishNumber::from)
        .collect::<Vec<SnailFishNumber>>();
    (0..ns.len())
        .map(|i| {
            (0..ns.len())
                .map(|j| {
                    if i != j {
                        ns[i].clone().add(&ns[j]).normalize().magnitude()
                    } else {
                        0
                    }
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[test]
fn test_magnitude() {
    assert_eq!(SnailFishNumber::from("[9,1]").magnitude(), 29);
    assert_eq!(SnailFishNumber::from("[[9,1],[1,9]]").magnitude(), 129);
    assert_eq!(
        SnailFishNumber::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
        3488
    );
}

#[test]
fn test_explode() {
    assert_eq!(
        SnailFishNumber::from("[[[[[9,8],1],2],3],4]")
            .explode()
            .to_string(),
        "[[[[0,9],2],3],4]"
    );

    assert_eq!(
        SnailFishNumber::from("[7,[6,[5,[4,[3,2]]]]]")
            .explode()
            .to_string(),
        "[7,[6,[5,[7,0]]]]"
    );

    assert_eq!(
        SnailFishNumber::from("[[6,[5,[4,[3,2]]]],1]")
            .explode()
            .to_string(),
        "[[6,[5,[7,0]]],3]"
    );

    assert_eq!(
        SnailFishNumber::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
            .explode()
            .to_string(),
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    );

    assert_eq!(
        SnailFishNumber::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
            .explode()
            .to_string(),
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
    );
}

#[test]
fn test_add() {
    assert_eq!(
        SnailFishNumber::from("[1,1]")
            .add(&SnailFishNumber::from("[2,2]"))
            .add(&SnailFishNumber::from("[3,3]"))
            .add(&SnailFishNumber::from("[4,4]"))
            .to_string(),
        "[[[[1,1],[2,2]],[3,3]],[4,4]]"
    );
}

#[test]
fn test_add_normalize() {
    assert_eq!(
        SnailFishNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]")
            .add(&SnailFishNumber::from("[1,1]"))
            .normalize()
            .to_string(),
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    );
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_18_sample.input").unwrap();
    assert_eq!(solution(&input), 4140);
    assert_eq!(solution2(&input), 3993);
    let input = fs::read_to_string("src/inputs/aoc_18.input").unwrap();
    assert_eq!(solution(&input), 3524);
    assert_eq!(solution2(&input), 4656);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_18.input").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}
