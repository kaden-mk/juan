/*
* Reference Text:
* module main
*
* import std.io
*
* fn main()
*   io.println("Hello, World")
* end*/

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Fn,
    Module,
    Import,
    End,

    Dot,
    LParen,
    RParen,

    Identifier(String),
    Str(String),

    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token(pub TokenKind);
