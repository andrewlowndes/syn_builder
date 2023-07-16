use crate::{attrs_builder, mutability_builder, qself_builder, IntoIdent, IntoPath, IntoType};
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

pub trait PatIdentBuilder {
    fn by_ref(self, by_ref: bool) -> Self;
    fn subpat(self, subpat: impl IntoPat) -> Self;
}

impl PatIdentBuilder for PatIdent {
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

pub fn pat_paren(pat: impl IntoPat) -> PatParen {
    PatParen {
        attrs: Default::default(),
        paren_token: Default::default(),
        pat: pat.into_pat().into(),
    }
}

attrs_builder!(PatParen);

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

pub fn pat_rest() -> PatRest {
    PatRest {
        attrs: Default::default(),
        dot2_token: Default::default(),
    }
}

attrs_builder!(PatRest);

pub fn pat_slice<P: IntoPat>(elems: impl IntoIterator<Item = P>) -> PatSlice {
    PatSlice {
        attrs: Default::default(),
        bracket_token: Default::default(),
        elems: FromIterator::from_iter(elems.into_iter().map(IntoPat::into_pat)),
    }
}

attrs_builder!(PatSlice);

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

pub trait PatStructBuilder {
    fn rest(self, rest: PatRest) -> Self;
}

impl PatStructBuilder for PatStruct {
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

pub fn pat_type(pat: impl IntoPat, ty: impl IntoType) -> PatType {
    PatType {
        attrs: Default::default(),
        pat: pat.into_pat().into(),
        colon_token: Default::default(),
        ty: ty.into_type().into(),
    }
}

attrs_builder!(PatType);

pub fn pat_wild() -> PatWild {
    PatWild {
        attrs: Default::default(),
        underscore_token: Default::default(),
    }
}

attrs_builder!(PatWild);

pub fn field_pat(member: impl Into<Member>, pat: impl IntoPat) -> FieldPat {
    FieldPat {
        attrs: Default::default(),
        member: member.into(),
        colon_token: None,
        pat: pat.into_pat().into(),
    }
}

attrs_builder!(FieldPat);

pub trait FieldPatBuilder {
    fn colon_token(self, colon_token: bool) -> Self;
}

impl FieldPatBuilder for FieldPat {
    fn colon_token(self, colon_token: bool) -> Self {
        Self {
            colon_token: colon_token.then(Default::default),
            ..self
        }
    }
}
