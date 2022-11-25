#![allow(non_camel_case_types)]
#![allow(dead_code)]

static JS_VERSION: &str = "3.0.0";

const JS_EXPR_MAX: u8 = 20;
const JS_GC_THRESHOLD: f32 = 0.75;

pub(crate) type JsOff = u32;
pub(crate) type JsVal = u64;

/*
const F_NOEXEC: usize = 1usize;
const F_LOOP: usize = 2usize;
const F_CALL: usize = 4usize;
const F_BREAK: usize = 8usize;
const F_RETURN: usize = 16usize;
*/

pub(crate) enum Flags {
    NOEXEC,     // Parse code, but not execute
    LOOP,       // We're inside the loop
    CALL,       // We're inside a function call
    BREAK,      // Exit the loop
    RETURN      // Return has been executed
}

#[derive(Clone, PartialEq)]
pub(crate) enum Token {
    ERR, EOF, IDENTIFIER, NUMBER, STRING, SEMICOLON,
    LPAREN, RPAREN, LBRACE, RBRACE, BREAK = 50, CASE, CATCH,
    CLASS, CONST, CONTINUE, DEFAULT, DELETE, DO, ELSE,
    FINALLY, FOR, FUNC, IF, IN, INSTANCEOF, LET, NEW,
    RETURN, SWITCH, THIS, THROW, TRY, VAR, VOID, WHILE,
    WITH, YIELD, UNDEF, NULL, TRUE, FALSE, DOT = 100, CALL,
    POSTINC, POSTDEC, NOT, TILDE, TYPEOF, UPLUS, UMINUS,
    EXP, MUL, DIV, REM, PLUS, MINUS, SHL, SHR, ZSHR, LT,
    LE, GT, GE, EQ, NE, AND, XOR, OR, LAND, LOR, COLON,
    Q, ASSIGN, PLUS_ASSIGN, MINUS_ASSIGN, MUL_ASSIGN,
    DIV_ASSIGN, REM_ASSIGN, SHL_ASSIGN, SHR_ASSIGN,
    ZSHR_ASSIGN, AND_ASSIGN, XOR_ASSIGN, OR_ASSIGN, COMMA,
}

// A JS memory stores different entities: objects, properties, strings
// All entities are packed to the beginning of a buffer.
// The `brk` marks the end of the used memory:
//
// | entity1 | entity2 | .... | entityN |  unused memory |
// |---------|---------|------| ------- | -------------- | 
// Js.mem                               Js.brk           Js.size
//
// LSB: Least Significant Bit
//
// Each entity is 4-byte aligned, therefore 2 LSB bits store entity type
//
// Object:    8 bytes: offset of the first property, offset of the upper obj
// Property:    8 bytes + val: 4 byte next property, 4 byte key offs, N byte value
// String:    4xN bytes: 4 byte len << 2, 4 byte-aligned 0-terminated data
//
// If Rust functions are imported, they use the upper part
// of memory as stack for passing params. Each argument is pushed to the top of the memory as
// JsVal, and Js.size is decreased by sizeof(JsVal), i.e. 8 bytes. When the function returns,
// Js.size is restored back. So Js.size is used as a stack pointer.
//


// Pack Js values into u64, float64
// 64bit "float64": 1 bit sign, 11 bits exponent, 52 bits mantissa
//
// seeeeeee|eeeemmmm|mmmmmmmm|mmmmmmmm|mmmmmmmm|mmmmmmmm|mmmmmmmm|mmmmmmmm
// 11111111|11110000|00000000|00000000|00000000|00000000|00000000|00000000 inf
// 11111111|11111000|00000000|00000000|00000000|00000000|00000000|00000000 qnan
//
// 11111111|1111tttt|vvvvvvvv|vvvvvvvv|vvvvvvvv|vvvvvvvv|vvvvvvvv|vvvvvvvv
//  NaN marker |type|  48-bit placeholder for values: pointers, strings
//
// On 64-bit platforms, pointers are really 48 bit only, so they can fit,
// provided they are sign extended

#[derive(Debug)]
pub(crate) enum Type {
    OBJ, PROP, STR, UNDEF, NULL, NUM,
    BOOL, FUNC, CODEREF, RFUNC, ERR
}

pub(crate) fn type_str(typ: Type) -> String {
    format!("{:#?}", typ)
}

pub(crate) fn make_val(typ: Type, data: u64) -> JsVal {
    0x7ff0u64 << 48u64 | ((typ as u8) << 48) as JsVal | data & 0xffffffffffffu64
}

pub(crate) fn v_data(v: JsVal) -> usize {
    (v & !(0x7fffu64 << 48u64 as JsVal)) as usize
}


// Utilities
fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
}

fn is_ident_begin(c: char) -> bool {
    c == '_' || c == '$' || is_alpha(c)
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_ident_continue(c: char) -> bool {
    c == '_' || c == '$' || is_alpha(c) || is_digit(c)
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\n' || c == '\t'
}


pub(crate) fn parse_keyword(buffer: &str) -> Token {
    let value = buffer.chars().nth(0).unwrap();
    match value {
        'b' if "break" == buffer.to_owned() => Token::BREAK,
        'c' => {
            match buffer {
                "class" => Token::CLASS,
                "case" => Token::CASE,
                "catch" => Token::CATCH,
                "const" => Token::CONST,
                "continue" => Token::CONTINUE,
                _ => Token::ERR,
            }
        },
        'd' => {
            match buffer {
                "do" => Token::DO,
                "default" => Token::DEFAULT,
                _ => Token::ERR,
            }
        },
        'e' if "else" == buffer => Token::ELSE,
        'f' => {
            match buffer {
                "for" => Token::FOR,
                "function" => Token::FUNC,
                "finally" => Token::FINALLY,
                "false" => Token::FALSE,
                _ => Token::ERR,
            }
        },
        'i' => {
            match buffer {
                "if" => Token::IF,
                "in" => Token::IN,
                "instanceof" => Token::INSTANCEOF,
                _ => Token::ERR,
            }
        },
        'l' if "let" == buffer => Token::LET,
        'n' => {
            match buffer {
                "new" => Token::NEW,
                "null" => Token::NULL,
                _ => Token::ERR,
            }
        },
        'r' if "return" == buffer => Token::RETURN,
        's' if "switch" == buffer => Token::SWITCH,
        't' => {
            match buffer {
                "try" => Token::TRY,
                "this" => Token::THIS,
                "throw" => Token::THROW,
                "true" => Token::TRUE,
                "typeof" => Token::TYPEOF,
                _ => Token::ERR,
            }
        },
        'u' if "undefined" == buffer => Token::UNDEF,
        'v' => {
            match buffer {
                "var" => Token::VAR,
                "void" => Token::VOID,
                _ => Token::ERR,
            }
        },
        'w' => {
            match buffer {
                "while" => Token::WHILE,
                "with" => Token::WITH,
                _ => Token::ERR,
            }
        },
        'y' if "yield" == buffer => Token::YIELD,
        _ => Token::IDENTIFIER,
    }
}

pub(crate) fn parse_ident(buffer: &str, t_len: &mut JsOff) -> Token {
    if is_ident_begin(buffer.chars().nth(0).unwrap()) {
        loop {
            if *t_len < buffer.len() as u32 && is_ident_continue(buffer.chars().nth(*t_len as usize).unwrap()) {
                *t_len += 1;
            }
            return parse_keyword(buffer)
        }
    }
    Token::ERR
}

pub(crate) fn skip_to_next(code: &str, len: JsOff, mut n: JsOff) -> JsOff {
    let n_u = n as usize;
    while n < len {
        let c = code.chars().nth(n_u).unwrap();
        let c_n = code.chars().nth(n_u + 1).unwrap();

        if is_space(c) {
            n += 1;
        } else if (n + 1 < len) && c == '/' && c_n == '/' {
            n += 2;
            while n < len && c != '\n' {
                n += 1;
            }
        } else if (n + 3 < len) && c == '/' && c_n == '*' {
            n += 4;
            while n < len && (code.chars().nth(n_u - 2).unwrap() != '*' || code.chars().nth(n_u - 1).unwrap() != '/') {
                n += 1;
            }
        } else {
            break;
        }
    }
    n
}

pub(crate) fn is_nan(v: JsVal) -> bool {
    (v >> 52u64) == 0x7ffu64
}

pub(crate) fn v_type(v: JsVal) -> u8 {
    if is_nan(v) { ((v >> 48u64) & 15u64) as u8 } else { Token::NUMBER as u8 }
}

pub(crate) fn tok_val(d: f64) -> JsVal {
    d as JsVal
}

pub(crate) fn is_err(v: JsVal) -> bool {
    v_type(v) == Token::ERR as u8
}

pub(crate) fn str_to_double(buf: &str) -> f64 {
    buf.parse::<f64>().unwrap()
}
