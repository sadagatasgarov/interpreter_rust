use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,
    Minus,
    Bang,
    Asteriks,
    Slash,

    Lt,
    Gt,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False

}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Ident => write!(f, "Ident"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Assign => write!(f, "Assign"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Lparen => write!(f, "Lparen"),
            TokenKind::Rparen => write!(f, "Rparen"),
            TokenKind::Lbrace => write!(f, "Lbrace"),
            TokenKind::Rbrace => write!(f, "Rbrace"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Asteriks => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::True =>write!(f, "True"),
            TokenKind::False =>write!(f, "False"),
        }
    }
}



pub fn lookup_ident(identifier: &String) -> TokenKind {
    match identifier.as_str() {
        "fn" => TokenKind::Function,
        "let" => TokenKind::Let,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        _ => TokenKind::Ident 
    }
}