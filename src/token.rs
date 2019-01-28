#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Keyword {
    Null,
    New,
    And,
    Or,
    True,
    False,
    Function,
    FunctionAt,
    Return,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Symbol {
    Equal,
    EqualEqual,
    LBrace,
    RBrace,
    Colon,
    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dot,
    Comma,
    Minus,
    MinusEqual,
    ThinArrow,
    Asterisk,
    AsteriskEqual,
    ForwardSlash,
    ForwardSlashEqual,
    Plus,
    PlusEqual,
    LT, GT, LTEqual, GTEqual,
    Not, NotEqual, Caret,
}

#[derive(Debug, Clone)]
pub enum LexerToken {
    Identifier(String),
    Keyword(Keyword),
    Number(String),
    Hex(String),
    Octal(String),
    Base36(String),
    Typename(String),
    Symbol(Symbol),
    EOF
}

impl LexerToken {
    pub fn as_identifier(&self) -> Option<String> {
        match self {
            &LexerToken::Identifier(ref s) => Some(s.clone()),
            _ => None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            &LexerToken::Number(ref s) => s.parse().ok(),
            _ => None
        }
    }
}
