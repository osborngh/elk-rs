#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::core::{
    JsOff,
    JsVal,
};

pub(crate) enum Flags {
    NOEXEC,     // Parse code, but not execute
    LOOP,       // We're inside the loop
    CALL,       // We're inside a function call
    BREAK,      // Exit the loop
    RETURN      // Return has been executed
}

// JS Engine
pub struct Js<'a> {
    rss: JsOff,          // Max observed Rust stack size
    lwm: JsOff,         // JS RAM low watermark: min free RAM observed
    code: &'a str,      // Current parsed code snippet
    err_msg: &'a str,   // Error message placeholder
    tok: u8,            // Last parsed token value
    consumed: u8,       // Indicator that last parsed token consumed
    flags: Flags,       // Execution flags, see FLAGS enum above
    c_len: JsOff,       // Code snippet length 
    pos: JsOff,         // Current parsing position
    t_off: JsOff,       // Offset of the last parsed token
    t_len: JsOff,       // Length of the last parsed token
    no_gc: JsOff,       // Entity offset to exclude from GC
    t_val: JsVal,      // Holds last parsed numeric or string literal value
    scope: JsVal,      // Current scope
    mem: u8,            // Available JS memory
    size: JsOff,        // Memory size
    brk: JsOff,         // Current mem usage boundary
    gc_t: JsOff,        // GC thresold. If brk > gct, trigger GC
    max_ss: JsOff,      // Maximum allowed stack size usage
    stk: Box<u8>,       // Stack pointer at the beginning of Js::eval()
}


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
    pub fn setmaxss(&self, i: isize) {
        todo!("Implement")
    }

    /// Set GC trigger threshold
    pub fn setgct(&self, gct: isize) {
        todo!("Implement")
    }

    pub fn stats(&self, total: &isize, min: &isize, css: &isize) {
        todo!("Implement")
    }

    /// Print debug info.
    pub fn dump(&self) {
        todo!("Implement")
    }
}

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
    pub fn get_str(js: &Js, val: JsVal, len: &isize) -> &'static str {
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
    pub fn make_str(js: &Js, string: &str, len: isize) -> JsVal {
        todo!()
    }

    /// Create Js number
    pub fn make_num(num: f64) {
        todo!()
    }

    /// Create Js error
    pub fn make_err(js: &Js, fmt: &str, args: Vec<&str>) {
        todo!()
    }

    /// Create Js function
    pub fn make_fun() {
        todo!()
    }

    /// Create Js object
    pub fn make_object(js: &Js) {
        todo!()
    }

    /// Set Js object attribute
    pub fn set_object(js: &Js, num: JsVal, val: &str, ano: JsVal) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
}
