#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::core::*;


// JS Engine
pub struct Js<'a> {
    rss: JsOff,          // Max observed Rust stack size
    lwm: JsOff,         // JS RAM low watermark: min free RAM observed
    code: &'a str,      // Current parsed code snippet
    err_msg: &'a str,   // Error message placeholder
    tok: Token,            // Last parsed token value
    consumed: bool,       // Indicator that last parsed token consumed
    flags: Flags,       // Execution flags, see FLAGS enum above
    c_len: JsOff,       // Code snippet length 
    pos: JsOff,         // Current parsing position
    t_off: JsOff,       // Offset of the last parsed token
    t_len: JsOff,       // Length of the last parsed token
    no_gc: JsOff,       // Entity offset to exclude from GC
    t_val: JsVal,      // Holds last parsed numeric or string literal value
    scope: JsVal,      // Current scope
    mem: Vec<u8>,            // Available JS memory
    size: JsOff,        // Memory size
    brk: JsOff,         // Current mem usage boundary
    gc_t: JsOff,        // GC thresold. If brk > gct, trigger GC
    max_ss: JsOff,      // Maximum allowed stack size usage
    stk: Box<u8>,       // Stack pointer at the beginning of Js::eval()
}


/// Api
impl<'a> Js<'a> {
    /// Create a new Js instance
    pub fn new(buffer: &[u8]) -> Js {
        todo!("Implement")
    }

    /// Execute Js code passed as &str
    pub fn eval(&self, code: &str) -> JsVal {
        todo!("Implement")
    }
    
    /// Return the global object
    pub fn glob(&self) -> JsVal {
        todo!("Implement")
    }

    /// Stringify Js value
    pub fn str(&self, val: JsVal) -> &'a str {
        todo!("Implement")
    }

    /// Checks arguments validity
    pub fn chk_args(val: &JsVal, i: i32, args: &'a str) {
        todo!("Implement")
    }

    /// Set max stack size
    pub fn setmaxss(&mut self, max: isize) {
        self.max_ss = max as u32;
    }

    /// Set GC trigger threshold
    pub fn setgct(&mut self, gct: isize) {
        self.gc_t = gct as u32;
    }

    pub fn stats(&self, total: &isize, min: &isize, css: &isize) {
        todo!("Implement")
    }

    /// Print debug info.
    pub fn dump(&self) {
        todo!("Implement")
    }
}

/// Api
impl<'a> Js<'a> {
    /// All Methods with the get_ prefix get objects from `Js` and return them as rust objects.
    /// Serializing? Essentially

    /// Return Js value type
    pub fn get_type(val: JsVal) -> u32 {
        todo!()
    }

    /// Return Js number
    pub fn get_num(val: JsVal) -> f64 {
        todo!()
    }

    /// Return Js boolean
    pub fn get_bool(val: JsVal) -> bool {
        todo!()
    }

    /// Return Js string
    pub fn get_str(val: JsVal, len: &isize) -> &'static str {
        todo!()
    }

    /// All Methods with the make_ prefix make `Js` objects(values) directly from Rust values.
    /// Serializing? Essentially

    /// Create Js undefined
    pub fn make_undef() -> JsVal {
        todo!()
    }

    /// Create Js null
    pub fn make_null() -> JsVal {
        todo!()
    }

    /// Create Js true
    pub fn make_true() -> JsVal {
        todo!()
    }
    
    /// Create Js false
    pub fn make_false() -> JsVal {
        todo!()
    }

    /// Create Js string
    pub fn make_str(string: &str, len: isize) -> JsVal {
        todo!()
    }

    /// Create Js number
    pub fn make_num(num: f64) {
        todo!()
    }

    /// Create Js error
    pub fn make_err(fmt: &str) -> JsVal {
        make_val(Type::ERR, 0)
    }

    /// Create Js function
    pub fn make_fun() {
        todo!()
    }

    /// Create Js object
    pub fn make_object() {
        todo!()
    }

    /// Set Js object attribute
    pub fn set_object(num: JsVal, val: &str, ano: JsVal) {
        todo!()
    }
}

impl<'a> Js<'a> {
    pub fn stmt(&self) -> JsVal {
        let res: JsVal = 0;

        if self.brk > self.gc_t { Js::<'a>::gc(); }

        match self.next() {
            Token::CASE | Token::CATCH | Token::CLASS | Token::CONST | Token::DEFAULT | Token::DELETE | Token::DO | Token::FINALLY | Token::IN | Token::INSTANCEOF | Token::NEW | Token::SWITCH | Token::THIS | Token::THROW | Token::TRY | Token::VAR | Token::VOID | Token::WITH | Token::WHILE | Token::YIELD => {
                res = Js::<'a>::make_err(format!("{} not implemented", 2).as_str());
            },
            Token::CONTINUE => { res = Js::<'a>::continue_(); },
            _ => ()
        }
        res
    }

    pub fn gc() {

    } 
}

// Internals
impl<'a> Js<'a> {
    fn next(&mut self) -> Token {
        if self.consumed { return self.tok.clone() }
        self.consumed = true;
        self.tok = Token::ERR;
        self.t_off = skip_to_next(self.code, self.c_len, self.pos);
        self.pos = self.t_off;
        self.t_len = 0;

        let b = format!("{}{}", self.code, self.t_off);
        let buf: &str = b.as_str();

        if self.t_off >= self.c_len {
            self.tok = Token::EOF;
            return self.tok.clone()
        }

        match buf.chars().nth(0).unwrap() {
            '?' => self.token(Token::Q, 1),
            ':' => self.token(Token::COLON, 1),
            '(' => self.token(Token::LPAREN, 1),
            ')' => self.token(Token::RPAREN, 1),
            '{' => self.token(Token::LBRACE, 1),
            '}' => self.token(Token::RBRACE, 1),
            ';' => self.token(Token::SEMICOLON, 1),
            ',' => self.token(Token::COMMA, 1),
            '!' => {
                if self.look(buf, 1, '=') &&
                    self.look(buf, 2, '=') {
                    self.token(Token::NE, 3)
                } else {
                    self.token(Token::NOT, 1)
                }
            },
            '.' => self.token(Token::DOT, 1),
            '~' => self.token(Token::TILDE, 1),
            '-' => {
                if self.look(buf, 1, '-') {
                    self.token(Token::POSTDEC, 2)
                } else if self.look(buf, 1, '='){
                    self.token(Token::MINUS_ASSIGN, 2)
                } else {
                    self.token(Token::MINUS, 1)
                }
            },
            '+' => {
                if self.look(buf, 1, '+') {
                    self.token(Token::POSTINC, 2)
                } else if self.look(buf, 1, '=') {
                    self.token(Token::PLUS_ASSIGN, 2)
                } else {
                    self.token(Token::PLUS, 1)
                }
            },
            '*' => {
                if self.look(buf, 1, '*') {
                    self.token(Token::EXP, 2)
                } else if self.look(buf, 1, '=') {
                    self.token(Token::MUL_ASSIGN, 1)
                } else {
                    self.token(Token::MUL, 1)
                }
            },
            '/' => {
                if self.look(buf, 1, '=') {
                    self.token(Token::DIV_ASSIGN, 2)
                } else {
                    self.token(Token::DIV, 1)
                }
            },
            '%' => {
                if self.look(buf, 1, '=') {
                    self.token(Token::REM_ASSIGN, 2)
                } else {
                    self.token(Token::REM, 1)
                }
            },
            '&' => {
                if self.look(buf, 1, '&') {
                    self.token(Token::LAND, 2)
                } else if self.look(buf, 2, '=') {
                    self.token(Token::AND_ASSIGN, 2)
                } else {
                    self.token(Token::AND, 1)
                }
            },
            '|' => {
                if self.look(buf, 1, '|') {
                    self.token(Token::LOR, 2)
                } else if self.look(buf, 1, '=') {
                    self.token(Token::OR_ASSIGN, 1)
                } else {
                    self.token(Token::OR, 1)
                }
            },
            '=' => {
                if self.look(buf, 1, '=') &&
                    self.look(buf, 2, '=') {
                    self.token(Token::EQ, 3)
                } else {
                    self.token(Token::ASSIGN, 1)
                }
            },
            '<' => {
                if self.look(buf, 1, '<') && self.look(buf, 2, '=') {
                    self.token(Token::SHL_ASSIGN, 3)
                } else if self.look(buf, 1, '<') {
                    self.token(Token::SHL, 2)
                } else if self.look(buf, 1, '=') {
                    self.token(Token::LE, 2)
                } else {
                    self.token(Token::LT, 1)
                }
            },
            '>' => {
                if self.look(buf, 1, '>') && self.look(buf, 2, '=') {
                    self.token(Token::SHR_ASSIGN, 3)
                } else if self.look(buf, 1, '>') {
                    self.token(Token::SHR, 2)
                } else if self.look(buf, 1, '=') {
                    self.token(Token::GE, 2);
                } else {
                    self.token(Token::GT, 1)
                }
            },
            '^' => {
                if self.look(buf, 1, '=') {
                    self.token(Token::XOR_ASSIGN, 2)
                } else {
                    self.token(Token::XOR, 1)
                }
            },
            '"' | '\'' => {
                let c_n = buf.chars().nth(0).unwrap();

                self.t_len += 1;

                while (self.t_off + self.t_len) < self.c_len && buf.chars().nth(self.t_len as usize).unwrap() != c_n {
                    let mut inc: u8 = 1;

                    if buf.chars().nth(self.t_len as usize).unwrap() == '\\' {
                        if self.t_off + self.t_len + 2 > self.c_len {
                            break;
                        }
                        inc = 2;
                        if buf.chars().nth(self.t_len as usize + 1).unwrap() == 'x' {
                            if self.t_off + self.t_len + 4 > self.c_len {
                                break;
                            }
                            inc = 4;
                        }
                    }
                    self.t_len += inc as u32;
                }
                if c_n == buf.chars().nth(self.t_len as usize).unwrap() {
                    self.tok = Token::STRING;
                    self.t_len += 1;
                }
            },
            '0'..='9' => {
                self.t_val = tok_val(str_to_double(buf));
                // TODO(lsm): protect against OOB access
                self.token(Token::NUMBER, buf.len() as JsOff)
            },
            _ => {
                self.tok = parse_ident(buf, &mut self.t_len);
            },
        };

        self.pos = self.t_off + self.t_len;
        self.tok.clone()
    }

    fn look(&self, buf: &str, offset: u8, ch: char) -> bool {
        (self.t_off + offset as u32) < self.c_len && buf.chars().nth(offset as usize).unwrap() == ch
    }

    fn token(&mut self, tok: Token, len: JsOff) {
        self.tok = tok;
        self.t_len = len;
    }

    fn look_ahead(&mut self) -> Token {
        let old: Token = self.tok.clone();
        let tok: Token;

        let pos: JsOff = self.pos;

        self.consumed = true;
        tok = self.next();

        self.pos = pos;
        self.tok = old;
        tok
    }

    fn make_scope(&mut self) {
        match self.flags {
            Flags::NOEXEC => panic!("[ELK]: No Exec Has Been Set"),
            _ => ()
        }

        let prev: JsOff = v_data(self.scope) as u32;
        // self.scope = Js::<'a>::make_object(prev);
    }

    fn load_off(&self, off: usize) -> JsOff {
        let mut v: JsOff = 0;
        assert!(self.brk <= self.size);
        std::mem::replace(&mut v, self.mem[off] as u32)
    }

    fn upper(&self, scope: JsVal) -> JsVal {
        make_val(Type::OBJ, self.load_off(v_data(scope) + std::mem::size_of::<JsOff>()) as u64)
    }

    fn delete_scope(&mut self) {
        self.scope = self.upper(self.scope);
    }

    fn create_block(&self, create_scope: bool) -> JsVal {
        let res: JsVal = Js::<'a>::make_undef();

        if create_scope { self.make_scope() };
        self.consumed = true;

        while self.next() != Token::EOF && self.next() != Token::RBRACE && !is_err(res) {
            let t = self.tok;

            let res = Js::<'a>::stmt();

            if !is_err(res) && t != Token::LBRACE && t != Token::IF && t != Token::WHILE && self.tok != Token::SEMICOLON {
                res = Js::<'a>::make_err("; expected");
                break;
            }
        }
        if create_scope { self.delete_scope() }
        res
    }

    fn lkp(&self, obj: JsVal, buf: &str, len: usize) {
        let off: JsOff = self.load_off(v_data(obj)) & !3u32;
    }
}

#[cfg(test)]
mod tests {
}
