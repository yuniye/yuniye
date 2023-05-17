// Copyright (C) 2023-Present Divine Niiquaye Ibok.
// This file is part the of Yuniye Programming Language.

// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// This code defines a struct called Token which holds a tuple of three values:
/// 1. A Kind enum value
/// 2. The line of the source code where the token first was lexed
/// 3. The starting position of the token as column
///
/// # Example
///
/// ```
/// use yuniye::token::*;
///
/// let token = Token(Kind::Identifier(b"let"), 1, 5);
/// ```
/// This creates a token representing an identifier starting at line 1 and starting at column 5.
#[derive(PartialEq, Debug, Clone)]
pub struct Token(pub Kind, pub usize, pub usize);

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Token({}, {}:{})", self.0, self.1, self.2)
    }
}

/// The struct which distinguish valid token types
#[derive(PartialEq, Debug, Clone)]
pub enum Kind {
    Eof,
    Comment,
    LineComment(&'static [u8]),

    NewLine(usize),
    Indent(&'static [u8]),

    Identifier(&'static [u8]),
    Literal(LiteralKind),

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Period,
    Minus,
    Plus,
    Slash,
    Star,
    Percent,
    Amper,
    VBar,
    Colon,
    SemiColon,
    Assign,
    Not, // also as Bang
    Less,
    Greater,
    Elvis, // also as QuestionMark
    Caret,
    At,
    Dollar,
    Tilde,
    Increment,
    Decrement,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    AndAssign,
    OrAssign,
    CaretAssign,
    CoalesceAssign,
    Coalesce,
    Or,
    And,
    CoalesceAnd,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    LShift,
    RShift,
    Pipe,
    Attribute,
    Arrow,
    DoubleArrow,
    DoubleColon,
    Range,
    ElvisDot,
    SpaceShip,
    LShiftAssign,
    RShiftAssign,
    RangeInclusive,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    /// "12_u8", "0o100", "0b120i99", "1f32".
    Int(u8, Vec<u8>),
    /// "12.34f32", "1e3", but not "1f32".
    Float(Vec<u8>),
    /// "'a'", "'\\'", "'''", "';". True if starts with b
    Char(u32, bool),
    /// ""abc"", ""abc". True if starts with b
    Str(Vec<u8>, bool),
    /// "r"abc"", "r#"abc"#". True if starts with b
    RawStr(&'static [u8], bool),
}

impl Into<Kind> for LiteralKind {
    fn into(self) -> Kind {
        Kind::Literal(self)
    }
}


impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Eof => write!(f, "Eof"),
            Kind::Comment => write!(f, "Comment"),
            Kind::LineComment(comment) => write!(f, "DocComment({:?})", comment),

            Kind::NewLine(line) => write!(f, "NewLine({})", line),
            Kind::Indent(indent) => write!(f, "Indent([{}])", indent.iter().map(|b| b.to_string()).collect::<Vec<String>>().join(", ")),

            Kind::Identifier(bytes) => write!(f, "Name({})", unsafe { std::str::from_utf8_unchecked(bytes) }),
            Kind::Literal(kind) => {
                match kind {
                    LiteralKind::Char(b, is_byte) => write!(f, "Char({:?}, {})", unsafe { char::from_u32_unchecked(*b) }, is_byte),
                    LiteralKind::Str(bytes, is_byte) => write!(f, "Str({:?}, {})", unsafe { std::str::from_utf8_unchecked(bytes) }, is_byte),
                    LiteralKind::RawStr(bytes, is_byte) => write!(f, "RawStr({:?}, {})", unsafe { std::str::from_utf8_unchecked(bytes) }, is_byte),
                    LiteralKind::Int(base, number) => write!(f, "Int({}, {})", unsafe { String::from_utf8_unchecked(number.clone()) }, base),
                    LiteralKind::Float(number) => write!(f, "Float({:?})", unsafe { String::from_utf8_unchecked(number.clone()) }),
                }
            },

            Kind::LParen => write!(f, "("),
            Kind::RParen => write!(f, ")"),
            Kind::LBrace => write!(f, "{{"),
            Kind::RBrace => write!(f, "}}"),
            Kind::LBracket => write!(f, "["),
            Kind::RBracket => write!(f, "]"),
            Kind::Comma => write!(f, ","),
            Kind::Period => write!(f, "."),
            Kind::Minus => write!(f, "-"),
            Kind::Plus => write!(f, "+"),
            Kind::Slash => write!(f, "/"),
            Kind::Star => write!(f, "*"),
            Kind::Percent => write!(f, "%"),
            Kind::Amper => write!(f, "&"),
            Kind::VBar => write!(f, "|"),
            Kind::Colon => write!(f, ":"),
            Kind::SemiColon => write!(f, ";"),
            Kind::Assign => write!(f, "="),
            Kind::Not => write!(f, "!"),
            Kind::Less => write!(f, "<"),
            Kind::Greater => write!(f, ">"),
            Kind::Elvis => write!(f, "?"),
            Kind::Caret => write!(f, "^"),
            Kind::At => write!(f, "@"),
            Kind::Dollar => write!(f, "$"),
            Kind::Tilde => write!(f, "~"),
            Kind::Increment => write!(f, "++"),
            Kind::Decrement => write!(f, "--"),
            Kind::AddAssign => write!(f, "+="),
            Kind::SubAssign => write!(f, "-+"),
            Kind::MulAssign => write!(f, "*="),
            Kind::DivAssign => write!(f, "/="),
            Kind::ModAssign => write!(f, "%="),
            Kind::AndAssign => write!(f, "&="),
            Kind::OrAssign => write!(f, "|="),
            Kind::CaretAssign => write!(f, "^="),
            Kind::CoalesceAssign => write!(f, "?="),
            Kind::CoalesceAnd => write!(f, "?:"),
            Kind::Coalesce => write!(f, "??"),
            Kind::ElvisDot => write!(f, "?."),
            Kind::Or => write!(f, "||"),
            Kind::And => write!(f, "&&"),
            Kind::Equal => write!(f, "=="),
            Kind::NotEqual => write!(f, "!="),
            Kind::LessEqual => write!(f, "<="),
            Kind::GreaterEqual => write!(f, ">="),
            Kind::LShift => write!(f, "<<"),
            Kind::RShift => write!(f, ">>"),
            Kind::Pipe => write!(f, "|>"),
            Kind::Attribute => write!(f, "#["),
            Kind::Arrow => write!(f, "->"),
            Kind::DoubleArrow => write!(f, "=>"),
            Kind::DoubleColon => write!(f, "::"),
            Kind::Range => write!(f, ".."),
            Kind::RangeInclusive => write!(f, "..="),
            Kind::LShiftAssign => write!(f, "<<="),
            Kind::RShiftAssign => write!(f, ">>="),
            Kind::SpaceShip => write!(f, "<=>"),
        }
    }
}
