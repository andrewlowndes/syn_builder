use crate::{IntoExpr, IntoMacroDelimiter, IntoPath};
use proc_macro2::TokenStream;
use syn::{token, AttrStyle, Attribute, Meta, MetaList, MetaNameValue};

pub fn attribute(meta: impl IntoMeta) -> Attribute {
    Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: meta.into_meta(),
    }
}

pub fn attr_style_inner() -> AttrStyle {
    AttrStyle::Inner(Default::default())
}

pub trait AttributeBuilder {
    fn new(meta: impl IntoMeta) -> Self;
    fn style(self, inside: bool) -> Self;
}

impl AttributeBuilder for Attribute {
    fn new(meta: impl IntoMeta) -> Self {
        attribute(meta)
    }
    fn style(self, inside: bool) -> Self {
        Self {
            style: if inside {
                AttrStyle::Inner(Default::default())
            } else {
                AttrStyle::Outer
            },
            ..self
        }
    }
}

pub trait IntoAttrStyle {
    fn into_attr_style(self) -> AttrStyle;
}

impl IntoAttrStyle for AttrStyle {
    fn into_attr_style(self) -> AttrStyle {
        self
    }
}

impl IntoAttrStyle for token::Not {
    fn into_attr_style(self) -> AttrStyle {
        AttrStyle::Inner(self)
    }
}

pub trait IntoMeta {
    fn into_meta(self) -> Meta;
}

impl IntoMeta for Meta {
    fn into_meta(self) -> Meta {
        self
    }
}

pub fn meta_list(
    path: impl IntoPath,
    delimiter: impl IntoMacroDelimiter,
    tokens: TokenStream,
) -> MetaList {
    MetaList {
        path: path.into_path(),
        delimiter: delimiter.into_macro_delimiter(),
        tokens,
    }
}

pub trait MetaListBuilder {
    fn new(path: impl IntoPath, delimiter: impl IntoMacroDelimiter, tokens: TokenStream) -> Self;
}

impl MetaListBuilder for MetaList {
    fn new(path: impl IntoPath, delimiter: impl IntoMacroDelimiter, tokens: TokenStream) -> Self {
        meta_list(path, delimiter, tokens)
    }
}

impl IntoMeta for MetaList {
    fn into_meta(self) -> Meta {
        Meta::List(self)
    }
}

pub fn meta_name_value(path: impl IntoPath, value: impl IntoExpr) -> MetaNameValue {
    MetaNameValue {
        path: path.into_path(),
        eq_token: Default::default(),
        value: value.into_expr(),
    }
}

impl IntoMeta for MetaNameValue {
    fn into_meta(self) -> Meta {
        Meta::NameValue(self)
    }
}

pub trait MetaNameValueBuilder {
    fn new(path: impl IntoPath, value: impl IntoExpr) -> Self;
}

impl MetaNameValueBuilder for MetaNameValue {
    fn new(path: impl IntoPath, value: impl IntoExpr) -> Self {
        meta_name_value(path, value)
    }
}
