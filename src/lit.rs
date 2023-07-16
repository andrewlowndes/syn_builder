use proc_macro2::Span;
use syn::{Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr};

pub trait IntoLit {
    fn into_lit(self) -> Lit;
}

impl IntoLit for Lit {
    fn into_lit(self) -> Lit {
        self
    }
}

pub fn lit_str(value: &str) -> LitStr {
    LitStr::new(value, Span::call_site())
}

impl IntoLit for LitStr {
    fn into_lit(self) -> Lit {
        Lit::Str(self)
    }
}

pub fn lit_byte_str(value: &[u8]) -> LitByteStr {
    LitByteStr::new(value, Span::call_site())
}

impl IntoLit for LitByteStr {
    fn into_lit(self) -> Lit {
        Lit::ByteStr(self)
    }
}

pub fn lit_byte(value: u8) -> LitByte {
    LitByte::new(value, Span::call_site())
}

impl IntoLit for LitByte {
    fn into_lit(self) -> Lit {
        Lit::Byte(self)
    }
}

pub fn lit_char(value: char) -> LitChar {
    LitChar::new(value, Span::call_site())
}

impl IntoLit for LitChar {
    fn into_lit(self) -> Lit {
        Lit::Char(self)
    }
}

pub fn lit_int(value: &str) -> LitInt {
    LitInt::new(value, Span::call_site())
}

impl IntoLit for LitInt {
    fn into_lit(self) -> Lit {
        Lit::Int(self)
    }
}

pub fn lit_float(value: &str) -> LitFloat {
    LitFloat::new(value, Span::call_site())
}

impl IntoLit for LitFloat {
    fn into_lit(self) -> Lit {
        Lit::Float(self)
    }
}

pub fn lit_bool(value: bool) -> LitBool {
    LitBool::new(value, Span::call_site())
}

impl IntoLit for LitBool {
    fn into_lit(self) -> Lit {
        Lit::Bool(self)
    }
}
