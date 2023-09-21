use crate::{
    attrs_builder,
    macros::{AttrsPropsBuilder, MutabilityPropsBuilder, QSelfPropsBuilder},
    mutability_builder, qself_builder, IntoIdent, IntoPath, IntoType,
};
use proc_macro2::TokenStream;
use syn::{
    FieldPat, Member, Pat, PatConst, PatIdent, PatLit, PatMacro, PatOr, PatParen, PatPath,
    PatRange, PatReference, PatRest, PatSlice, PatStruct, PatTuple, PatTupleStruct, PatType,
    PatWild,
};

pub trait IntoPat {
    fn into_pat(self) -> Pat;
}

impl IntoPat for Pat {
    fn into_pat(self) -> Pat {
        self
    }
}

macro_rules! impl_into_pat {
    ($($target:ident($type:ty),)*) => {
        $(
            impl IntoPat for $type {
                fn into_pat(self) -> Pat {
                    Pat::$target(self)
                }
            }
        )*
    };
}

impl_into_pat!(
    Const(PatConst),
    Ident(PatIdent),
    Lit(PatLit),
    Macro(PatMacro),
    Or(PatOr),
    Paren(PatParen),
    Path(PatPath),
    Range(PatRange),
    Reference(PatReference),
    Rest(PatRest),
    Slice(PatSlice),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Type(PatType),
    Verbatim(TokenStream),
    Wild(PatWild),
);

pub fn pat_ident(ident: impl IntoIdent) -> PatIdent {
    PatIdent {
        attrs: Default::default(),
        by_ref: None,
        mutability: None,
        ident: ident.into_ident(),
        subpat: None,
    }
}

attrs_builder!(PatIdent);
mutability_builder!(PatIdent);

pub trait PatIdentBuilder: AttrsPropsBuilder + MutabilityPropsBuilder {
    fn new(ident: impl IntoIdent) -> Self;
    fn by_ref(self, by_ref: bool) -> Self;
    fn subpat(self, subpat: impl IntoPat) -> Self;
}

impl PatIdentBuilder for PatIdent {
    fn new(ident: impl IntoIdent) -> Self {
        pat_ident(ident)
    }

    fn by_ref(self, by_ref: bool) -> Self {
        Self {
            by_ref: by_ref.then(Default::default),
            ..self
        }
    }

    fn subpat(self, subpat: impl IntoPat) -> Self {
        Self {
            subpat: Some((Default::default(), subpat.into_pat().into())),
            ..self
        }
    }
}

pub fn pat_or<C: IntoPat>(cases: impl IntoIterator<Item = C>) -> PatOr {
    PatOr {
        attrs: Default::default(),
        leading_vert: None,
        cases: FromIterator::from_iter(cases.into_iter().map(IntoPat::into_pat)),
    }
}

attrs_builder!(PatOr);

pub trait PatOrBuilder: AttrsPropsBuilder {
    fn new<C: IntoPat>(cases: impl IntoIterator<Item = C>) -> Self;
}

impl PatOrBuilder for PatOr {
    fn new<C: IntoPat>(cases: impl IntoIterator<Item = C>) -> Self {
        pat_or(cases)
    }
}

pub fn pat_paren(pat: impl IntoPat) -> PatParen {
    PatParen {
        attrs: Default::default(),
        paren_token: Default::default(),
        pat: pat.into_pat().into(),
    }
}

attrs_builder!(PatParen);

pub trait PatParenBuilder: AttrsPropsBuilder {
    fn new(pat: impl IntoPat) -> Self;
}

impl PatParenBuilder for PatParen {
    fn new(pat: impl IntoPat) -> Self {
        pat_paren(pat)
    }
}

pub fn pat_reference(pat: impl IntoPat) -> PatReference {
    PatReference {
        attrs: Default::default(),
        and_token: Default::default(),
        mutability: None,
        pat: pat.into_pat().into(),
    }
}

attrs_builder!(PatReference);
mutability_builder!(PatReference);

pub trait PatReferenceBuilder: AttrsPropsBuilder + MutabilityPropsBuilder {
    fn new(pat: impl IntoPat) -> Self;
}

impl PatReferenceBuilder for PatReference {
    fn new(pat: impl IntoPat) -> Self {
        pat_reference(pat)
    }
}

pub fn pat_rest() -> PatRest {
    PatRest {
        attrs: Default::default(),
        dot2_token: Default::default(),
    }
}

attrs_builder!(PatRest);

pub trait PatRestBuilder: AttrsPropsBuilder {
    fn new() -> Self;
}

impl PatRestBuilder for PatRest {
    fn new() -> Self {
        pat_rest()
    }
}

pub fn pat_slice<P: IntoPat>(elems: impl IntoIterator<Item = P>) -> PatSlice {
    PatSlice {
        attrs: Default::default(),
        bracket_token: Default::default(),
        elems: FromIterator::from_iter(elems.into_iter().map(IntoPat::into_pat)),
    }
}

attrs_builder!(PatSlice);

pub trait PatSliceBuilder: AttrsPropsBuilder {
    fn new<P: IntoPat>(elems: impl IntoIterator<Item = P>) -> Self;
}

impl PatSliceBuilder for PatSlice {
    fn new<P: IntoPat>(elems: impl IntoIterator<Item = P>) -> Self {
        pat_slice(elems)
    }
}

pub fn pat_struct<F: Into<FieldPat>>(
    path: impl IntoPath,
    fields: impl IntoIterator<Item = F>,
) -> PatStruct {
    PatStruct {
        attrs: Default::default(),
        qself: None,
        path: path.into_path(),
        brace_token: Default::default(),
        fields: FromIterator::from_iter(fields.into_iter().map(Into::into)),
        rest: None,
    }
}

attrs_builder!(PatStruct);
qself_builder!(PatStruct);

pub trait PatStructBuilder: AttrsPropsBuilder + QSelfPropsBuilder {
    fn new<F: Into<FieldPat>>(path: impl IntoPath, fields: impl IntoIterator<Item = F>) -> Self;
    fn rest(self, rest: PatRest) -> Self;
}

impl PatStructBuilder for PatStruct {
    fn new<F: Into<FieldPat>>(path: impl IntoPath, fields: impl IntoIterator<Item = F>) -> Self {
        pat_struct(path, fields)
    }
    fn rest(self, rest: PatRest) -> Self {
        Self {
            rest: Some(rest),
            ..self
        }
    }
}

pub fn pat_tuple<E: IntoPat>(elems: impl IntoIterator<Item = E>) -> PatTuple {
    PatTuple {
        attrs: Default::default(),
        paren_token: Default::default(),
        elems: FromIterator::from_iter(elems.into_iter().map(IntoPat::into_pat)),
    }
}

attrs_builder!(PatTuple);

pub trait PatTupleBuilder: AttrsPropsBuilder {
    fn new<E: IntoPat>(elems: impl IntoIterator<Item = E>) -> Self;
}

impl PatTupleBuilder for PatTuple {
    fn new<E: IntoPat>(elems: impl IntoIterator<Item = E>) -> Self {
        pat_tuple(elems)
    }
}

pub fn pat_tuple_struct<E: IntoPat>(
    path: impl IntoPath,
    elems: impl IntoIterator<Item = E>,
) -> PatTupleStruct {
    PatTupleStruct {
        attrs: Default::default(),
        qself: None,
        path: path.into_path(),
        paren_token: Default::default(),
        elems: FromIterator::from_iter(elems.into_iter().map(IntoPat::into_pat)),
    }
}

attrs_builder!(PatTupleStruct);
qself_builder!(PatTupleStruct);

pub trait PatTupleStructBuilder: AttrsPropsBuilder + QSelfPropsBuilder {
    fn new<E: IntoPat>(path: impl IntoPath, elems: impl IntoIterator<Item = E>) -> Self;
}

impl PatTupleStructBuilder for PatTupleStruct {
    fn new<E: IntoPat>(path: impl IntoPath, elems: impl IntoIterator<Item = E>) -> Self {
        pat_tuple_struct(path, elems)
    }
}

pub fn pat_type(pat: impl IntoPat, ty: impl IntoType) -> PatType {
    PatType {
        attrs: Default::default(),
        pat: pat.into_pat().into(),
        colon_token: Default::default(),
        ty: ty.into_type().into(),
    }
}

attrs_builder!(PatType);

pub trait PatTypeBuilder: AttrsPropsBuilder {
    fn new(pat: impl IntoPat, ty: impl IntoType) -> Self;
}

impl PatTypeBuilder for PatType {
    fn new(pat: impl IntoPat, ty: impl IntoType) -> Self {
        pat_type(pat, ty)
    }
}

pub fn pat_wild() -> PatWild {
    PatWild {
        attrs: Default::default(),
        underscore_token: Default::default(),
    }
}

attrs_builder!(PatWild);

pub trait PatWildBuilder: AttrsPropsBuilder {
    fn new() -> Self;
}

impl PatWildBuilder for PatWild {
    fn new() -> Self {
        pat_wild()
    }
}

pub fn field_pat(member: impl Into<Member>, pat: impl IntoPat) -> FieldPat {
    FieldPat {
        attrs: Default::default(),
        member: member.into(),
        colon_token: None,
        pat: pat.into_pat().into(),
    }
}

attrs_builder!(FieldPat);

pub trait FieldPatBuilder: AttrsPropsBuilder {
    fn new(member: impl Into<Member>, pat: impl IntoPat) -> Self;
    fn colon_token(self, colon_token: bool) -> Self;
}

impl FieldPatBuilder for FieldPat {
    fn new(member: impl Into<Member>, pat: impl IntoPat) -> Self {
        field_pat(member, pat)
    }

    fn colon_token(self, colon_token: bool) -> Self {
        Self {
            colon_token: colon_token.then(Default::default),
            ..self
        }
    }
}
