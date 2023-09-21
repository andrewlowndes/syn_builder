use proc_macro2::TokenStream;
use syn::{
    token::{Brace, Bracket, Paren},
    Macro, MacroDelimiter,
};

use crate::IntoPath;

pub fn r#macro(path: impl IntoPath, tokens: impl Into<TokenStream>) -> Macro {
    Macro {
        path: path.into_path(),
        bang_token: Default::default(),
        delimiter: MacroDelimiter::Paren(Default::default()),
        tokens: tokens.into(),
    }
}

pub trait MacroBuilder {
    fn new(path: impl IntoPath, tokens: impl Into<TokenStream>) -> Self;
}

impl MacroBuilder for Macro {
    fn new(path: impl IntoPath, tokens: impl Into<TokenStream>) -> Self {
        r#macro(path, tokens)
    }
}

pub fn macro_delimiter_paren_variant() -> MacroDelimiter {
    MacroDelimiter::Paren(Default::default())
}

pub fn macro_delimiter_brace_variant() -> MacroDelimiter {
    MacroDelimiter::Brace(Default::default())
}

pub fn macro_delimiter_bracket_variant() -> MacroDelimiter {
    MacroDelimiter::Bracket(Default::default())
}

pub trait IntoMacroDelimiter {
    fn into_macro_delimiter(self) -> MacroDelimiter;
}

impl IntoMacroDelimiter for MacroDelimiter {
    fn into_macro_delimiter(self) -> MacroDelimiter {
        self
    }
}

impl IntoMacroDelimiter for Paren {
    fn into_macro_delimiter(self) -> MacroDelimiter {
        MacroDelimiter::Paren(self)
    }
}

impl IntoMacroDelimiter for Brace {
    fn into_macro_delimiter(self) -> MacroDelimiter {
        MacroDelimiter::Brace(self)
    }
}

impl IntoMacroDelimiter for Bracket {
    fn into_macro_delimiter(self) -> MacroDelimiter {
        MacroDelimiter::Bracket(self)
    }
}
