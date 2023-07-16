use crate::{attrs_builder, vis_builder, IntoFields, IntoIdent};
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

pub trait DeriveInputBuilder {
    fn generics(self, generics: Generics) -> Self;
}

impl DeriveInputBuilder for DeriveInput {
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

impl IntoData for DataUnion {
    fn into_data(self) -> Data {
        Data::Union(self)
    }
}
