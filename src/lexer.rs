use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // Kulcsszavak
    #[token("fv")]
    KeywordFn,
    
    #[token("vált")]
    KeywordLet,
    
    #[token("konst")]
    KeywordConst,
    
    #[token("vissza")]
    KeywordReturn,
    
    #[token("ha")]
    KeywordIf,
    
    #[token("különben")]
    KeywordElse,
    
    #[token("míg")]
    KeywordWhile,
    
    #[token("szor")]
    KeywordFor,
    
    #[token("strukt")]
    KeywordStruct,
    
    #[token("behajt")]
    KeywordImpl,
    
    #[token("aszink")]
    KeywordAsync,
    
    #[token("bevár")]
    KeywordAwait,
    
    // Azonosítók
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
    
    // Literálok
    #[regex(r"-?\d+", |lex| lex.slice().parse().ok())]
    Int(i64),
    
    #[regex(r"-?\d+\.\d+", |lex| lex.slice().parse().ok())]
    Float(f64),
    
    #[regex(r#"([^"\\]|\\.*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    StringLit(String),
    
    #[regex(r"true|false", |lex| lex.slice().parse().ok())]
    Bool(bool),
    
    // Operátorok
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Star,
    
    #[token("/")]
    Slash,
    
    #[token("%")]
    Percent,
    
    #[token("=")]
    Equals,
    
    #[token("==")]
    DoubleEquals,
    
    #[token("!=")]
    NotEquals,
    
    #[token("<")]
    LessThan,
    
    #[token("<=")]
    LessOrEqual,
    
    #[token(">")]
    GreaterThan,
    
    #[token(">=")]
    GreaterOrEqual,
    
    #[token("!")]
    Bang,
    
    #[token("&&")]
    And,
    
    #[token("||")]
    Or,
    
    // Szimbólumok
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,
    
    #[token("{")]
    LBrace,
    
    #[token("}")]
    RBrace,
    
    #[token("[")]
    LSquare,
    
    #[token("]")]
    RSquare,
    
    #[token(",")]
    Comma,
    
    #[token(":")]
    Colon,
    
    #[token(";")]
    Semicolon,
    
    #[token("->")]
    Arrow,
    
    #[token(".")]
    Dot,
    
    #[token("..")]
    DoubleDot,
    
    // Kommentek és whitespace
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Comment,
    
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    
    // Hibakeresés
    #[error]
    Error
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input)
        }
    }
    
    pub fn span(&self) -> (usize, usize) {
        let span = self.inner.span();
        (span.start, span.end)
    }
    
    pub fn slice(&self) -> &'a str {
        self.inner.slice()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, (usize, usize));
    
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.inner.slice();
        let span = self.inner.span();
        Some((token, (span.start, span.end)))
    }
}
