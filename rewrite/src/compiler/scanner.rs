use crate::{
    compiler::scanner::{
        BinOP::{Different, Equals, Greater, GreaterEqual, IterConcat, Less, LessEqual},
        BitOp::{AND, LSH, NOT, OR, RSH, XOR},
        Keywords::{
            Const, Constexpr, Else, Extern, Fn, For, If, Let, Puts, Return, Syscall, Var, While,
        },
        MathOp::{IntDiv, Minus, Mod, Plus, Slash, Star},
        PBB::{Brace, Bracket, Paren},
        TokenKind::{
            And, Arrow, Bang, BinaryOperator, BitOperator, Bool, Close, Colon, Comma, Dot, Float,
            Hexa, Identifier, Integer, Keyword, MathOperator, Nil, Octal, Open, Or, Semi, Sign,
            Walrus, Wild,
        },
    },
    utils::compiler_outputs::CompilerOutputs,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
    If,
    Else,
    Constexpr,
    Const,
    Let,
    Var,
    Return,
    Syscall,
    Extern,
    Puts,
    For,
    Fn,
    While,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MathOp {
    IntDiv,
    Slash,
    Minus,
    Star,
    Plus,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOP {
    GreaterEqual,
    IterConcat,
    LessEqual,
    Different,
    Greater,
    Equals,
    Less,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BitOp {
    LSH,
    RSH,
    NOT,
    XOR,
    AND,
    OR,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PBB {
    Bracket,
    Paren,
    Brace,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    BinaryOperator(BinOP),
    MathOperator(MathOp),
    Identifier(String),
    BitOperator(BitOp),
    Keyword(Keywords),
    String(String),
    Integer(i128),
    Octal(u128),
    Float(f64),
    Close(PBB),
    Hexa(u128),
    Bool(bool),
    Open(PBB),
    Walrus,
    Arrow,
    Colon,
    Comma,
    Sign,
    Semi,
    Bang,
    Wild,
    Dot,
    And,
    Nil,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: isize,
}

#[derive(Debug, Clone, Default)]
pub struct Scanner {
    pub tokens: Vec<Token>,
    bytes: Vec<u8>,
    line: isize,
}

fn is_keyword(identifier: &str) -> Option<Keywords> {
    match identifier {
        "if" => Some(If),
        "else" => Some(Else),
        "constexpr" => Some(Constexpr),
        "const" => Some(Const),
        "let" => Some(Let),
        "var" => Some(Var),
        "return" => Some(Return),
        "extern" => Some(Extern),
        "syscall!" => Some(Syscall),
        "puts" => Some(Puts),
        "while" => Some(While),
        "for" => Some(For),
        "fn" => Some(Fn),
        _ => None,
    }
}

impl Scanner {
    pub fn new(scan: String) -> Self {
        Self {
            bytes: scan.bytes().collect(),
            ..Default::default()
        }
    }

    fn push(&mut self, tk: TokenKind) {
        self.tokens.push(Token {
            line: self.line,
            kind: tk,
        });
    }

    pub fn read(&mut self) {
        let mut i = 0;
        while i < self.bytes.len() {
            let ch = self.bytes[i];
            let second = *self.bytes.get(i + 1).unwrap_or(&0u8);

            let size = self.tokens.len();

            if ch.is_ascii_digit() && !(second == b'x' || second == b'o') {
                i += 1;
                let mut number = String::from(char::from(ch));
                while self.bytes[i].is_ascii_digit()
                    || ((!number.contains('.')) && self.bytes[i] == b'.')
                {
                    number.push(char::from(self.bytes[i]));
                    i += 1;
                }

                if number.contains('.') {
                    self.push(Float(number.parse::<f64>().unwrap()));
                } else {
                    self.push(Integer(number.parse::<i128>().unwrap()));
                }

                continue;
            }

            match ch {
                // FAZER O DEC E O INC
                b'-' if second == b'-' && self.tokens.last().unwrap().kind == Semi => {
                    for j in (i + 2)..self.bytes.len() {
                        if self.bytes[j] == b'\n' {
                            i = j;
                            self.line += 1;
                            break;
                        }
                    }
                    continue;
                }

                b'\n' | b'\r' | b' ' => {
                    if ch == b'\n' {
                        self.line += 1;
                    }

                    i += 1;
                    continue;
                }

                b'.' if second.is_ascii_digit() => {
                    i += 1;
                    let mut number = String::from("0.");
                    while self.bytes[i].is_ascii_digit() {
                        number.push(char::from(self.bytes[i]));
                        i += 1;
                    }

                    self.push(Float(number.parse::<f64>().unwrap()));
                    continue;
                }

                b'-' if second.is_ascii_digit() => {
                    i += 1;
                    let mut number = String::from("-");
                    while self.bytes[i].is_ascii_digit() {
                        number.push(char::from(self.bytes[i]));
                        i += 1;
                    }

                    self.push(Integer(number.parse::<i128>().unwrap()));
                    continue;
                }

                b'.' if second == b'.' => {
                    i += 1;
                    self.push(BinaryOperator(IterConcat))
                }

                b'-' if second == b'>' => {
                    i += 1;
                    self.push(Arrow)
                }

                b'>' if second == b'>' => {
                    i += 1;
                    self.push(BitOperator(RSH))
                }
                b'<' if second == b'<' => {
                    i += 1;
                    self.push(BitOperator(LSH))
                }

                b'>' if second == b'=' => {
                    i += 1;
                    self.push(BinaryOperator(GreaterEqual))
                }
                b'<' if second == b'=' => {
                    i += 1;
                    self.push(BinaryOperator(LessEqual))
                }
                b'!' if second == b'=' => {
                    i += 1;
                    self.push(BinaryOperator(Different))
                }
                b'=' if second == b'=' => {
                    i += 1;
                    self.push(BinaryOperator(Equals))
                }

                b':' if second == b'=' => {
                    i += 1;
                    self.push(Walrus)
                }

                b'/' if second == b'/' => {
                    i += 1;
                    self.push(MathOperator(IntDiv))
                }

                b'|' if second == b'|' => {
                    i += 1;
                    self.push(Or)
                }
                b'&' if second == b'&' => {
                    i += 1;
                    self.push(And)
                }

                b'0' if second == b'x' => {
                    i += 2;
                    let mut number = String::new();
                    while self.bytes[i].is_ascii_hexdigit() {
                        number.push(char::from(self.bytes[i]));
                        i += 1;
                    }

                    i -= 1;
                    self.push(Hexa(u128::from_str_radix(&number, 16).unwrap()));
                }

                b'0' if second == b'o' => {
                    i += 2;
                    let mut number = String::new();
                    while matches!(self.bytes[i], b'0'..=b'7') {
                        number.push(char::from(self.bytes[i]));
                        i += 1;
                    }
                    i -= 1;
                    self.push(Octal(u128::from_str_radix(&number, 8).unwrap()));
                }

                b'"' => {
                    i += 1;
                    let mut last: u8 = 0;
                    let mut string = String::new();
                    while (self.bytes[i] != b'"' && last != b'\\') || self.bytes[i] == b'\n' {
                        last = self.bytes[i];
                        string.push(char::from(self.bytes[i]));
                        i += 1;

                        if last == b'\n' {
                            CompilerOutputs::fatal(
                                "You can't to have a string with more than one Literal line",
                            );
                        }
                    }

                    self.push(TokenKind::String(string));
                }

                b',' => self.push(Comma),
                b':' => self.push(Colon),
                b';' => self.push(Semi),
                b'=' => self.push(Sign),
                b'!' => self.push(Bang),
                b'.' => self.push(Dot),
                b'_' => self.push(Wild),

                b'<' => self.push(BinaryOperator(Less)),
                b'>' => self.push(BinaryOperator(Greater)),

                b'*' => self.push(MathOperator(Star)),
                b'+' => self.push(MathOperator(Plus)),
                b'-' => self.push(MathOperator(Minus)),
                b'/' => self.push(MathOperator(Slash)),
                b'%' => self.push(MathOperator(Mod)),

                b'&' => self.push(BitOperator(AND)),
                b'~' => self.push(BitOperator(NOT)),
                b'^' => self.push(BitOperator(XOR)),
                b'|' => self.push(BitOperator(OR)),

                b'[' => self.push(Open(Bracket)),
                b'{' => self.push(Open(Brace)),
                b'(' => self.push(Open(Paren)),

                b']' => self.push(Close(Bracket)),
                b'}' => self.push(Close(Brace)),
                b')' => self.push(Close(Paren)),

                _ => {}
            };

            if ch.is_ascii_alphabetic() || ch == b'@' {
                i += 1;
                let mut identifier = String::from(char::from(ch));
                while self.bytes[i].is_ascii_alphanumeric() || self.bytes[i] == b'!' {
                    identifier.push(char::from(self.bytes[i]));
                    i += 1;

                    if self.bytes[i] == b'!' {
                        break;
                    }
                }

                if let Some(keyword) = is_keyword(&identifier) {
                    self.push(Keyword(keyword));
                    continue;
                }

                match identifier.as_str() {
                    "false" => self.push(Bool(false)),
                    "true" => self.push(Bool(true)),
                    "nil" => self.push(Nil),
                    _ => self.push(Identifier(identifier)),
                }

                continue;
            }

            if size < self.bytes.len() {
                i += 1;
                continue;
            }

            i += 1;
        }
    }
}
