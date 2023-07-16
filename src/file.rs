use crate::{attrs_builder, IntoItem};
use syn::File;

pub fn file<I: IntoItem>(items: impl IntoIterator<Item = I>) -> File {
    File {
        shebang: None,
        attrs: Default::default(),
        items: items.into_iter().map(IntoItem::into_item).collect(),
    }
}

attrs_builder!(File);

pub trait FileBuilder {
    fn shebang(self, shebang: impl Into<String>) -> Self;
}

impl FileBuilder for File {
    fn shebang(self, shebang: impl Into<String>) -> Self {
        Self {
            shebang: Some(shebang.into()),
            ..self
        }
    }
}
