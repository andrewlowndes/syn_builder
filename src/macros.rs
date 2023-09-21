use syn::{Attribute, Label, QSelf};

use crate::{IntoType, IntoVisibility};

pub trait AttrsPropsBuilder {
    fn attr(self, attr: impl Into<Attribute>) -> Self;
    fn attrs<A: Into<Attribute>>(self, attrs: impl IntoIterator<Item = A>) -> Self;
}

#[macro_export]
macro_rules! attrs_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::AttrsPropsBuilder for $name {
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

pub trait MutabilityPropsBuilder {
    fn mutability(self, mutability: bool) -> Self;
}

#[macro_export]
macro_rules! mutability_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::MutabilityPropsBuilder for $name {
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

pub trait QSelfPropsBuilder {
    fn qself(self, qself: impl Into<QSelf>) -> Self;
}

#[macro_export]
macro_rules! qself_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::QSelfPropsBuilder for $name {
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

pub trait LabelPropsBuilder {
    fn label(self, label: impl Into<Label>) -> Self;
}

#[macro_export]
macro_rules! label_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::LabelPropsBuilder for $name {
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

pub trait VisPropsBuilder {
    fn vis(self, vis: impl IntoVisibility) -> Self;
}

#[macro_export]
macro_rules! vis_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::VisPropsBuilder for $name {
                fn vis(self, vis: impl $crate::IntoVisibility) -> Self {
                    Self { vis: vis.into_visibility(), ..self }
                }
            }
        )*
    }
}

pub trait OutputPropsBuilder {
    fn output(self, ty: impl IntoType) -> Self;
}

#[macro_export]
macro_rules! output_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::macros::OutputPropsBuilder for $name {
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
