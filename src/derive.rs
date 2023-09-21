use crate::{
    attrs_builder,
    macros::{AttrsPropsBuilder, VisPropsBuilder},
    vis_builder, IntoFields, IntoIdent,
};
use syn::{
    Data, DataEnum, DataStruct, DataUnion, DeriveInput, FieldsNamed, Generics, Variant, Visibility,
};

pub fn derive_input(ident: impl IntoIdent, data: impl IntoData) -> DeriveInput {
    DeriveInput {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        ident: ident.into_ident(),
        generics: Default::default(),
        data: data.into_data(),
    }
}

attrs_builder!(DeriveInput);
vis_builder!(DeriveInput);

pub trait DeriveInputBuilder: AttrsPropsBuilder + VisPropsBuilder {
    fn new(ident: impl IntoIdent, data: impl IntoData) -> Self;
    fn generics(self, generics: Generics) -> Self;
}

impl DeriveInputBuilder for DeriveInput {
    fn new(ident: impl IntoIdent, data: impl IntoData) -> Self {
        derive_input(ident, data)
    }

    fn generics(self, generics: Generics) -> Self {
        Self { generics, ..self }
    }
}

pub trait IntoData {
    fn into_data(self) -> Data;
}

impl IntoData for Data {
    fn into_data(self) -> Data {
        self
    }
}

pub fn data_struct(fields: impl IntoFields) -> DataStruct {
    DataStruct {
        struct_token: Default::default(),
        fields: fields.into_fields(),
        semi_token: None,
    }
}

pub trait DataStructBuilder {
    fn new(fields: impl IntoFields) -> Self;
}

impl DataStructBuilder for DataStruct {
    fn new(fields: impl IntoFields) -> Self {
        data_struct(fields)
    }
}

impl IntoData for DataStruct {
    fn into_data(self) -> Data {
        Data::Struct(self)
    }
}

pub fn data_enum<V: Into<Variant>>(variants: impl IntoIterator<Item = V>) -> DataEnum {
    DataEnum {
        enum_token: Default::default(),
        brace_token: Default::default(),
        variants: FromIterator::from_iter(variants.into_iter().map(Into::into)),
    }
}

pub trait DataEnumBuilder {
    fn new<V: Into<Variant>>(variants: impl IntoIterator<Item = V>) -> Self;
}

impl DataEnumBuilder for DataEnum {
    fn new<V: Into<Variant>>(variants: impl IntoIterator<Item = V>) -> Self {
        data_enum(variants)
    }
}

impl IntoData for DataEnum {
    fn into_data(self) -> Data {
        Data::Enum(self)
    }
}

pub fn data_union(fields: impl Into<FieldsNamed>) -> DataUnion {
    DataUnion {
        union_token: Default::default(),
        fields: fields.into(),
    }
}

pub trait DataUnionBuilder {
    fn new(fields: impl Into<FieldsNamed>) -> Self;
}

impl DataUnionBuilder for DataUnion {
    fn new(fields: impl Into<FieldsNamed>) -> Self {
        data_union(fields)
    }
}

impl IntoData for DataUnion {
    fn into_data(self) -> Data {
        Data::Union(self)
    }
}
