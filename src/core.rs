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
pub(crate) enum Types {
    OBJ, PROP, STR, UNDEF, NULL, NUM,
    BOOL, FUNC, CODEREF, CFUNC, ERR
}

pub(crate) fn type_str(typ: Types) -> String {
    format!("{:?}", typ)
}

pub(crate) fn parse_ident(buffer: &str, t_len: &JsOff) -> Token {
    if is_ident_begin(buffer.chars().nth(0).unwrap()) {
        loop {
            if *t_len < buffer.len() as u32 && is_ident_continue(buffer.chars().nth(*t_len as usize)) {
                *t_len += 1;
            }
            return parse_keyword(buffer)
        }
    }
    Token::ERR
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
