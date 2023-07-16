use proc_macro2::{Ident, Span};

pub trait IntoIdent {
    fn into_ident(self) -> Ident;
}

impl IntoIdent for Ident {
    fn into_ident(self) -> Ident {
        self
    }
}

impl IntoIdent for &str {
    fn into_ident(self) -> Ident {
        Ident::new(self, Span::call_site())
    }
}
