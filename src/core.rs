#![allow(non_camel_case_types)]
#![allow(dead_code)]

static JS_VERSION: &str = "3.0.0";

const JS_EXPR_MAX: u8 = 20;
const JS_GC_THRESHOLD: f32 = 0.75;

pub(crate) type JsOff = u32;
pub(crate) type JsVal = u64;

pub(crate) enum Token {
    ERR, EOF, IDENTIFIER, NUMBER, STRING, SEMICOLON,
    LPAREN, RPAREN, LBRACE, RBRACE, BREAK, CASE, CATCH,
    CLASS, CONST, CONTINUE, DEFAULT, DELETE, DO, ELSE,
    FINALLY, FOR, FUNC, IF, IN, INSTANCEOF, LET, NEW,
    RETURN, SWITCH, THIS, THROW, TRY, VAR, VOID, WHILE,
    WITH, YIELD, UNDEF, NULL, TRUE, FALSE, DOT, CALL,
    POSTINC, POSTDEC, NOT, TILDE, TYPEOF, UPLUS, UMINUS,
    EXP, MUL, DIV, REM, PLUS, MINUS, SHL, SHR, ZSHR, LT,
    LE, GT, GE, EQ, NE, AND, XOR, OR, LAND, LOR, COLON,
    Q, ASSIGN, PLUS_ASSIGN, MINUS_ASSIGN, MUL_ASSIGN,
    DIV_ASSIGN, REM_ASSIGN, SHL_ASSIGN, SHR_ASSIGN,
    ZSHR_ASSIGN, AND_ASSIGN, XOR_ASSIGN, OR_ASSIGN, COMMA,
}

#[derive(Debug)]
pub(crate) enum Type {
    OBJ, PROP, STR, UNDEF, NULL, NUM,
    BOOL, FUNC, CODEREF, CFUNC, ERR
}

pub(crate) fn type_str(typ: Type) -> String {
    format!("{:#?}", typ)
}

/*
pub (crate) fn make_val(typ: Type, data: u64) -> JsVal {
    0x7ff0u64 << 48u64 | (typ << 48) | data & 0xffffffffffffu64
}
*/

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


pub(crate) fn parse_keyword(buffer: &str) -> Token {
    let value = buffer.chars().nth(0).unwrap_or_else(|| 0 as char);
    match value {
        'b' if "break" == buffer => Token::BREAK,
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

pub(crate) fn skip_to_next(code: &str, len: JsOff, n: JsOff) -> JsOff {
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

