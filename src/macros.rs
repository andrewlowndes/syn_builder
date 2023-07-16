use syn::{Attribute, Label, QSelf};

use crate::{IntoType, IntoVisibility};

pub trait AttrsBuilder {
    fn attr(self, attr: impl Into<Attribute>) -> Self;
    fn attrs<A: Into<Attribute>>(self, attrs: impl IntoIterator<Item = A>) -> Self;
}

#[macro_export]
macro_rules! attrs_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::AttrsBuilder for $name {
                fn attr(mut self, attr: impl Into<syn::Attribute>) -> Self {
                    self.attrs.push(attr.into());
                    self
                }

                fn attrs<A: Into<syn::Attribute>>(self, attrs: impl IntoIterator<Item = A>) -> Self {
                    Self {
                        attrs: attrs.into_iter().map(Into::into).collect(),
                        ..self
                    }
                }
            }
        )*
    }
}

pub trait MutabilityBuilder {
    fn mutability(self, mutability: bool) -> Self;
}

#[macro_export]
macro_rules! mutability_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::MutabilityBuilder for $name {
                fn mutability(self, mutability: bool) -> Self {
                    Self {
                        mutability: mutability.then(Default::default),
                        ..self
                    }
                }
            }
        )*
    }
}

pub trait QSelfBuilder {
    fn qself(self, qself: impl Into<QSelf>) -> Self;
}

#[macro_export]
macro_rules! qself_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::QSelfBuilder for $name {
                fn qself(self, qself: impl Into<syn::QSelf>) -> Self {
                    Self {
                        qself: Some(qself.into()),
                        ..self
                    }
                }
            }
        )*
    }
}

pub trait LabelBuilder {
    fn label(self, label: impl Into<Label>) -> Self;
}

#[macro_export]
macro_rules! label_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::LabelBuilder for $name {
                fn label(self, label: impl Into<syn::Label>) -> Self {
                    Self {
                        label: Some(label.into()),
                        ..self
                    }
                }
            }
        )*
    }
}

pub trait VisBuilder {
    fn vis(self, vis: impl IntoVisibility) -> Self;
}

#[macro_export]
macro_rules! vis_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::VisBuilder for $name {
                fn vis(self, vis: impl $crate::IntoVisibility) -> Self {
                    Self { vis: vis.into_visibility(), ..self }
                }
            }
        )*
    }
}

pub trait OutputBuilder {
    fn output(self, ty: impl IntoType) -> Self;
}

#[macro_export]
macro_rules! output_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::OutputBuilder for $name {
                fn output(self, ty: impl $crate::IntoType) -> Self {
                    Self {
                        output: syn::ReturnType::Type(Default::default(), ty.into_type().into()),
                        ..self
                    }
                }
            }
        )*
    }
}
