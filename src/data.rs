use crate::{attrs_builder, vis_builder, IntoExpr, IntoIdent, IntoType};
use syn::{Field, FieldMutability, Fields, FieldsNamed, FieldsUnnamed, Variant, Visibility};

pub fn variant(ident: impl IntoIdent) -> Variant {
    Variant {
        attrs: Default::default(),
        ident: ident.into_ident(),
        fields: Fields::Unit,
        discriminant: None,
    }
}

attrs_builder!(Variant);

pub trait VariantBuilder {
    fn discriminant(self, expr: impl IntoExpr) -> Self;
    fn fields(self, fields: impl IntoFields) -> Self;
}

impl VariantBuilder for Variant {
    fn discriminant(self, expr: impl IntoExpr) -> Self {
        Self {
            discriminant: Some((Default::default(), expr.into_expr())),
            ..self
        }
    }

    fn fields(self, fields: impl IntoFields) -> Self {
        Self {
            fields: fields.into_fields(),
            ..self
        }
    }
}

pub trait IntoFields {
    fn into_fields(self) -> Fields;
}

impl IntoFields for Fields {
    fn into_fields(self) -> Fields {
        self
    }
}

pub fn fields_named<F: Into<Field>>(fields: impl IntoIterator<Item = F>) -> FieldsNamed {
    FieldsNamed {
        brace_token: Default::default(),
        named: FromIterator::from_iter(fields.into_iter().map(Into::into)),
    }
}

impl IntoFields for FieldsNamed {
    fn into_fields(self) -> Fields {
        Fields::Named(self)
    }
}

pub fn fields_unamed<F: Into<Field>>(fields: impl IntoIterator<Item = F>) -> FieldsUnnamed {
    FieldsUnnamed {
        paren_token: Default::default(),
        unnamed: FromIterator::from_iter(fields.into_iter().map(Into::into)),
    }
}

impl IntoFields for FieldsUnnamed {
    fn into_fields(self) -> Fields {
        Fields::Unnamed(self)
    }
}

pub fn field(ty: impl IntoType) -> Field {
    Field {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: None,
        colon_token: None,
        ty: ty.into_type(),
    }
}

attrs_builder!(Field);
vis_builder!(Field);

pub trait FieldBuilder {
    fn ident(self, ident: impl IntoIdent) -> Self;
}

impl FieldBuilder for Field {
    fn ident(self, ident: impl IntoIdent) -> Self {
        Self {
            ident: Some(ident.into_ident()),
            colon_token: Some(Default::default()),
            ..self
        }
    }
}
