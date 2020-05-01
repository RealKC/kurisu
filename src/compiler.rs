#![allow(dead_code)]

use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::scanner::{Scanner, Token, TokenType};
use crate::value::Value;

const DEBUG_PRINT_CODE: bool = false;

pub fn compile(source: &str) -> Option<Chunk> {
    let mut scanner = Scanner::new(source);
    let mut current_chunk = Chunk::new();
    let mut parser = Parser::new(&mut scanner, &mut current_chunk);

    parser.advance(); // prime the parser

    expression(&mut parser);

    parser.consume(TokenType::Eof, "Expected end of expression");

    end_compiler(&mut parser);
    if parser.had_error {
        return None;
    }

    Some(parser.current_chunk.clone())
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Precedence {
    None = 0,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<u8> for Precedence {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::Assignment,
            2 => Self::Or,
            3 => Self::And,
            4 => Self::Equality,
            5 => Self::Comparison,
            6 => Self::Term,
            7 => Self::Factor,
            8 => Self::Unary,
            9 => Self::Call,
            10 => Self::Primary,
            _ => unreachable!("bad precendence"),
        }
    }
}

fn expression(parser: &mut Parser) {
    parser.parse_precedence(Precedence::Assignment);
}

fn end_compiler(parser: &mut Parser) {
    parser.emit_return();

    if DEBUG_PRINT_CODE {
        if !parser.had_error {
            parser.current_chunk.disassemble("code");
        }
    }
}

fn number(parser: &mut Parser) {
    let value = parser.previous.name.parse::<Value>().unwrap();
    parser.emit_constant(value);
}

fn grouping(parser: &mut Parser) {
    expression(parser);
    parser.consume(TokenType::RightParen, "Expect ')' after expression");
}

fn unary(parser: &mut Parser) {
    let type_ = parser.previous.type_;

    parser.parse_precedence(Precedence::Unary);

    match type_ {
        TokenType::Minus => parser.emit_byte(OpCode::Negate as u8),
        _ => (), // unreachable
    }
}

fn binary(parser: &mut Parser) {
    let operator_type = parser.previous.type_;

    let rule = &RULES[operator_type as usize];
    parser.parse_precedence((rule.precedence as u8 + 1).into());

    match operator_type {
        TokenType::Plus => parser.emit_byte(OpCode::Add as u8),
        TokenType::Minus => parser.emit_byte(OpCode::Subtract as u8),
        TokenType::Star => parser.emit_byte(OpCode::Multiply as u8),
        TokenType::Slash => parser.emit_byte(OpCode::Divide as u8),
        _ => (), // unreachable
    }
}

type ParseFn = fn(&mut Parser) -> ();

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

static RULES: [ParseRule; 41] = [
    ParseRule {
        // TokenType::LeftParen
        prefix: Some(grouping),
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::RightParen
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::LeftBrace
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::RightBrace
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Comma
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Dot
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Minus
        prefix: Some(unary),
        infix: Some(binary),
        precedence: Precedence::Term,
    },
    ParseRule {
        // TokenType::Plus
        prefix: None,
        infix: Some(binary),
        precedence: Precedence::Term,
    },
    ParseRule {
        // TokenType::Semicolon
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Slash
        prefix: None,
        infix: Some(binary),
        precedence: Precedence::Factor,
    },
    ParseRule {
        // TokenType::Star
        prefix: None,
        infix: Some(binary),
        precedence: Precedence::Factor,
    },
    ParseRule {
        // TokenType::Bang
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::BangEqual
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Equal
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::EqualEqual
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Greater
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::GreaterEqual
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Less
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::LessEqual
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Identifier
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::String
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Number
        prefix: Some(number),
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::And
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Class
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Else
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::False
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::For
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Fun
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::If
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Nil
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Or
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Print
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Return
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Super
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::This
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::True
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Var
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::While
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Error
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::Eof
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        // TokenType::DefaultConstructed
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
];

struct Parser<'a> {
    previous: Token,
    current: Token,
    had_error: bool,
    panic_mode: bool,
    scanner: &'a mut Scanner<'a>,
    current_chunk: &'a mut Chunk,
}

impl<'a> Parser<'a> {
    fn new(scanner: &'a mut Scanner<'a>, current_chunk: &'a mut Chunk) -> Self {
        Parser {
            previous: Token::new(),
            current: Token::new(),
            scanner: scanner,
            current_chunk: current_chunk,
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = self.scanner.scan_token();
            if self.current.type_ != TokenType::Error {
                break;
            }
        }
    }

    fn consume(&mut self, type_: TokenType, message: &str) {
        if self.current.type_ == type_ {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn parse_precedence(&mut self, p: Precedence) {
        self.advance();
        let prefix_rule = RULES[self.previous.type_ as usize].prefix;
        if prefix_rule.is_none() {
            self.error("Expect expression");
            return;
        }

        prefix_rule.unwrap()(self);

        while p <= RULES[self.current.type_ as usize].precedence {
            self.advance();
            let infix_rule = RULES[self.previous.type_ as usize].infix;
            infix_rule.unwrap()(self);
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        self.current_chunk.append(byte, self.previous.line as u32);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_constant(&mut self, val: Value) {
        self.current_chunk
            .append_constant(val, self.previous.line as u32);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn error_at_current(&mut self, message: &str) {
        let token = self.current.clone();
        self.error_at(&token, message);
    }

    fn error(&mut self, message: &str) {
        let token = self.previous.clone();
        self.error_at(&token, message)
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        match token.type_ {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => (),
            _ => eprint!(" at {}", token.name),
        }

        eprintln!(" {}", message);
        self.had_error = true;
    }
}
