use crate::{attrs_builder, macros::AttrsPropsBuilder, IntoItem};
use syn::File;

pub fn file<I: IntoItem>(items: impl IntoIterator<Item = I>) -> File {
    File {
        shebang: None,
        attrs: Default::default(),
        items: items.into_iter().map(IntoItem::into_item).collect(),
    }
}

attrs_builder!(File);

pub trait FileBuilder: AttrsPropsBuilder {
    fn new<I: IntoItem>(items: impl IntoIterator<Item = I>) -> Self;
    fn shebang(self, shebang: impl Into<String>) -> Self;
}

impl FileBuilder for File {
    fn new<I: IntoItem>(items: impl IntoIterator<Item = I>) -> Self {
        file(items)
    }

    fn shebang(self, shebang: impl Into<String>) -> Self {
        Self {
            shebang: Some(shebang.into()),
            ..self
        }
    }
}
