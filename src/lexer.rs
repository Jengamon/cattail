pub use super::token::{LexerToken, Keyword, Symbol};
use std::convert::AsRef;
use std::str::Chars;
use std::iter::{Peekable, Iterator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerErrorType {
    Unspecified(String),
    EOF
}

#[derive(Debug, Clone)]
pub struct LexerError {
    ln: usize,
    col: usize,
    letype: LexerErrorType
}

impl LexerError {
    pub fn new_unspecified<S: AsRef<str>>(ln: usize, col: usize, text: S) -> LexerError {
        LexerError {
            ln, col,
            letype: LexerErrorType::Unspecified(text.as_ref().into())
        }
    }

    pub fn eof(ln: usize, col: usize) -> LexerError {
        LexerError { ln, col, letype: LexerErrorType::EOF}
    }

    pub fn is_eof(&self) -> bool {
        self.letype == LexerErrorType::EOF
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    ln: usize,
    col: usize,
    ecol: usize
}

impl Span {
    pub fn new(ln: usize, col: usize, ecol: usize) -> Span {
        Span { ln, col, ecol }
    }
}

impl Span {
    pub fn line(&self) -> usize { self.ln }
    pub fn start(&self) -> usize { self.col }
    pub fn end(&self) -> usize { self.ecol }
}

pub type Token = (LexerToken, Span);

pub struct Lexer<'a> {
    i: Peekable<Chars<'a>>,
    ln: usize,
    col: usize,

    // Used in iterator
    error: bool
}

pub type LexerResult<A> = Result<A, LexerError>;

impl<'a> Lexer<'a> {
    pub fn new(s: Chars<'a>) -> Lexer<'a> {
        Lexer {
            i: s.peekable(),
            ln: 1,
            col: 1,
            error: false
        }
    }

    // Basic lexer methods
    fn expect(&mut self) -> LexerResult<char> {
        match self.next() {
            Some(c) => Ok(c),
            None => Err(LexerError::eof(self.ln, self.col))
        }
    }

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.i.next() {
            match c {
                '\n' => {self.ln+=1;self.col=1;Some(c)},
                _ => {self.col+=1;Some(c)}
            }
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.i.peek().cloned()
    }

    pub fn at_end(&mut self) -> bool { self.peek().is_none() }

    // Main driver
    pub fn next_token(&mut self) -> LexerResult<Token> {
        self.eat_whitespace_and_comments();
        match self.expect()? {
            '=' => {
                let mut span = self.create_span();
                let tok = match self.peek() {
                    Some('=') => {self.next(); LexerToken::Symbol(Symbol::EqualEqual)},
                    _ => LexerToken::Symbol(Symbol::Equal)
                };
                span.ecol = self.col;
                Ok((tok, span))
            },
            '+' => {
                let mut span = self.create_span();
                let tok = match self.peek() {
                    Some('=') => {self.next(); LexerToken::Symbol(Symbol::PlusEqual)},
                    _ => LexerToken::Symbol(Symbol::Plus)
                };
                span.ecol = self.col;
                Ok((tok, span))
            },
            '*' => {
                let mut span = self.create_span();
                let tok = match self.peek() {
                    Some('=') => {self.next(); LexerToken::Symbol(Symbol::AsteriskEqual)},
                    _ => LexerToken::Symbol(Symbol::Asterisk)
                };
                span.ecol = self.col;
                Ok((tok, span))
            },
            '/' => {
                let mut span = self.create_span();
                let tok = match self.peek() {
                    Some('=') => {self.next(); LexerToken::Symbol(Symbol::ForwardSlashEqual)},
                    _ => LexerToken::Symbol(Symbol::ForwardSlash)
                };
                span.ecol = self.col;
                Ok((tok, span))
            },
            '>' => {
                let mut span = self.create_span();
                let tok = match self.peek() {
                    Some('=') => {self.next(); LexerToken::Symbol(Symbol::GTEqual)},
                    _ => LexerToken::Symbol(Symbol::GT)
                };
                span.ecol = self.col;
                Ok((tok, span))
            },
            '<' => {
                let mut span = self.create_span();
                let tok = match self.peek() {
                    Some('=') => {self.next(); LexerToken::Symbol(Symbol::LTEqual)},
                    _ => LexerToken::Symbol(Symbol::LT)
                };
                span.ecol = self.col;
                Ok((tok, span))
            },
            '-' => {
                let mut span = self.create_span();
                let tok = match self.peek() {
                    Some('=') => {self.next(); LexerToken::Symbol(Symbol::MinusEqual)},
                    Some('>') => {self.next(); LexerToken::Symbol(Symbol::ThinArrow)},
                    _ => LexerToken::Symbol(Symbol::Minus)
                };
                span.ecol = self.col;
                Ok((tok, span))
            },
            '{' => Ok((LexerToken::Symbol(Symbol::LBrace), self.create_span())),
            '}' => Ok((LexerToken::Symbol(Symbol::RBrace), self.create_span())),
            ':' => Ok((LexerToken::Symbol(Symbol::Colon), self.create_span())),
            ';' => Ok((LexerToken::Symbol(Symbol::Semicolon), self.create_span())),
            '(' => Ok((LexerToken::Symbol(Symbol::LParen), self.create_span())),
            ')' => Ok((LexerToken::Symbol(Symbol::RParen), self.create_span())),
            '.' => Ok((LexerToken::Symbol(Symbol::Dot), self.create_span())),
            ',' => Ok((LexerToken::Symbol(Symbol::Comma), self.create_span())),
            '[' => Ok((LexerToken::Symbol(Symbol::LBracket), self.create_span())),
            ']' => Ok((LexerToken::Symbol(Symbol::RBracket), self.create_span())),
            '^' => Ok((LexerToken::Symbol(Symbol::Caret), self.create_span())),
            c if c.is_ascii_lowercase() || "?_!@".contains(c) => self.identifier(c),
            c if c.is_ascii_uppercase() => self.typename(c),
            '0' => match self.peek() {
                Some('x') => {self.next(); self.hex_number()},
                Some('o') => {self.next(); self.octal_number()},
                Some('b') => {self.next(); self.base36_number()},
                _ => self.number('0')
            },
            c if c.is_ascii_digit() => self.number(c),
            o => Err(LexerError::new_unspecified(self.ln, self.col, format!("Unknown token: {}", o)))
        }
    }

    fn create_span(&self) -> Span {
        Span::new(self.ln, self.col, self.col)
    }

    fn eat_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\n' | '\r' | '\t' => {self.next();}, // Eat the token
                '#' => { // Comments
                    self.next(); // Eat the #
                    match self.peek() {
                        Some('{') => {
                            let mut level = 1; // Multiline
                            while level > 0 {
                                match self.next() {
                                    Some('#') => match self.next() {
                                        Some('{') => level += 1,
                                        Some('}') => level -= 1,
                                        _ => {}
                                    },
                                    None => break,
                                    _ => {}
                                }
                            }
                        },
                        _ => while let Some(c) = self.next() { // One-line
                            if c == '\n' {
                                break
                            }
                        }
                    }
                },
                _ => break
            }
        }
    }

    fn identifier(&mut self, c: char) -> LexerResult<Token> {
        let mut span = self.create_span();
        let mut ident = String::new();
        ident.push(c);
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || "?_!@".contains(c) {
                ident.push(self.expect()?)
            } else {
                break
            }
        }
        span.ecol = self.col;

        let tok = match ident.as_ref() {
            "new" => LexerToken::Keyword(Keyword::New),
            "return" => LexerToken::Keyword(Keyword::Return),
            _ => LexerToken::Identifier(ident)
        };
        Ok((tok, span))
    }

    fn typename(&mut self, c: char) -> LexerResult<Token> {
        let mut span = self.create_span();
        let mut ident = String::new();
        ident.push(c);
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() {
                ident.push(self.expect()?)
            } else {
                break
            }
        }
        span.ecol = self.col;

        Ok((LexerToken::Typename(ident), span))
    }

    fn hex_number(&mut self) -> LexerResult<Token> {
        let mut span = self.create_span();
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_digit(16) {
                ident.push(self.expect()?)
            } else {
                break
            }
        }
        span.ecol = self.col;

        Ok((LexerToken::Typename(ident), span))
    }
    fn octal_number(&mut self) -> LexerResult<Token> {
        let mut span = self.create_span();
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_digit(8) {
                ident.push(self.expect()?)
            } else {
                break
            }
        }
        span.ecol = self.col;

        Ok((LexerToken::Typename(ident), span))
    }
    fn base36_number(&mut self) -> LexerResult<Token> {
        let mut span = self.create_span();
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_digit(36) {
                ident.push(self.expect()?)
            } else {
                break
            }
        }
        span.ecol = self.col;

        Ok((LexerToken::Typename(ident), span))
    }
    fn number(&mut self, c: char) -> LexerResult<Token> {
        let mut span = self.create_span();
        let mut number = String::new();
        number.push(c);
        let mut see_dot = false;
        while let Some(c) = self.peek() {
            match c {
                '.' if !see_dot => {see_dot = true; number.push('.'); self.next();},
                c if c.is_ascii_digit() => { number.push(c); self.next(); }
                _ => break
            }
        }
        span.ecol = self.col;

        if number.ends_with('.') {
            Err(LexerError::new_unspecified(self.ln, self.col, format!("Malformed number: {}", number)))
        } else {
            Ok((LexerToken::Number(number), span))
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = LexerResult<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end() || self.error {
            None
        } else {
            let tok = self.next_token();
            self.error = tok.is_err();
            Some(tok)
        }
    }
}
