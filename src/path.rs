use crate::{
    macros::OutputPropsBuilder, output_builder, IntoExpr, IntoIdent, IntoMeta, IntoType,
    IntoTypeParamBound,
};
use proc_macro2::Ident;
use syn::{
    AngleBracketedGenericArguments, AssocConst, AssocType, Constraint, Expr, GenericArgument,
    Lifetime, Meta, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf,
    ReturnType, Type,
};

pub trait PathGenericsBuilder {
    fn generics(self, args: impl Into<AngleBracketedGenericArguments>) -> Self;
}

macro_rules! generics_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::path::PathGenericsBuilder for $name {
                fn generics(self, generics: impl Into<syn::AngleBracketedGenericArguments>) -> Self {
                    Self {
                        generics: Some(generics.into()),
                        ..self
                    }
                }
            }
        )*
    }
}

pub fn path<S: Into<PathSegment>>(segments: impl IntoIterator<Item = S>) -> Path {
    Path {
        leading_colon: None,
        segments: FromIterator::from_iter(segments.into_iter().map(Into::into)),
    }
}

impl IntoMeta for Path {
    fn into_meta(self) -> syn::Meta {
        Meta::Path(self)
    }
}

pub trait IntoPath {
    fn into_path(self) -> Path;
}

impl IntoPath for Path {
    fn into_path(self) -> Path {
        self
    }
}

impl IntoPath for Ident {
    fn into_path(self) -> Path {
        Path::from(self)
    }
}

impl IntoPath for &str {
    fn into_path(self) -> Path {
        Path::from(self.into_ident())
    }
}

pub trait PathBuilder {
    fn new<S: Into<PathSegment>>(segments: impl IntoIterator<Item = S>) -> Self;
    fn leading_colon(self, leading_colon: bool) -> Self;
}

impl PathBuilder for Path {
    fn new<S: Into<PathSegment>>(segments: impl IntoIterator<Item = S>) -> Self {
        path(segments)
    }

    fn leading_colon(self, leading_colon: bool) -> Self {
        Self {
            leading_colon: leading_colon.then(Default::default),
            ..self
        }
    }
}

pub fn path_segment(ident: impl IntoIdent) -> PathSegment {
    PathSegment {
        ident: ident.into_ident(),
        arguments: Default::default(),
    }
}

pub trait PathSeqmentBuilder {
    fn new(ident: impl IntoIdent) -> Self;
    fn arguments(self, arguments: impl IntoPathArguments) -> Self;
}

impl PathSeqmentBuilder for PathSegment {
    fn new(ident: impl IntoIdent) -> Self {
        path_segment(ident)
    }

    fn arguments(self, arguments: impl IntoPathArguments) -> Self {
        Self {
            arguments: arguments.into_path_arguments(),
            ..self
        }
    }
}

pub trait IntoPathArguments {
    fn into_path_arguments(self) -> PathArguments;
}

impl IntoPathArguments for PathArguments {
    fn into_path_arguments(self) -> PathArguments {
        self
    }
}

pub trait IntoGenericArgument {
    fn into_generic_argument(self) -> GenericArgument;
}

impl IntoGenericArgument for GenericArgument {
    fn into_generic_argument(self) -> GenericArgument {
        self
    }
}

impl IntoGenericArgument for Lifetime {
    fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::Lifetime(self)
    }
}

impl IntoGenericArgument for Type {
    fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::Type(self)
    }
}

impl IntoGenericArgument for Expr {
    fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::Const(self)
    }
}

impl IntoGenericArgument for AssocType {
    fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::AssocType(self)
    }
}

impl IntoGenericArgument for AssocConst {
    fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::AssocConst(self)
    }
}

impl IntoGenericArgument for Constraint {
    fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::Constraint(self)
    }
}

pub fn angle_bracketed_generic_arguments<A: IntoGenericArgument>(
    args: impl IntoIterator<Item = A>,
) -> AngleBracketedGenericArguments {
    AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(
            args.into_iter()
                .map(IntoGenericArgument::into_generic_argument),
        ),
        gt_token: Default::default(),
    }
}

impl IntoPathArguments for AngleBracketedGenericArguments {
    fn into_path_arguments(self) -> PathArguments {
        PathArguments::AngleBracketed(self)
    }
}

pub trait AngleBracketedGenericArgumentsBuilder {
    fn new<A: IntoGenericArgument>(args: impl IntoIterator<Item = A>) -> Self;
    fn colon2_token(self, colon2_token: bool) -> Self;
}

impl AngleBracketedGenericArgumentsBuilder for AngleBracketedGenericArguments {
    fn new<A: IntoGenericArgument>(args: impl IntoIterator<Item = A>) -> Self {
        angle_bracketed_generic_arguments(args)
    }

    fn colon2_token(self, colon2_token: bool) -> Self {
        Self {
            colon2_token: colon2_token.then(Default::default),
            ..self
        }
    }
}

pub fn assoc_type(ident: impl IntoIdent, ty: impl IntoType) -> AssocType {
    AssocType {
        ident: ident.into_ident(),
        generics: None,
        eq_token: Default::default(),
        ty: ty.into_type(),
    }
}

generics_builder!(AssocType);

pub trait AssocTypeBuilder: PathGenericsBuilder {
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self;
}

impl AssocTypeBuilder for AssocType {
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self {
        assoc_type(ident, ty)
    }
}

pub fn assoc_const(ident: impl IntoIdent, value: impl IntoExpr) -> AssocConst {
    AssocConst {
        ident: ident.into_ident(),
        generics: None,
        eq_token: Default::default(),
        value: value.into_expr(),
    }
}

generics_builder!(AssocConst);

pub trait AssocConstBuilder: PathGenericsBuilder {
    fn new(ident: impl IntoIdent, value: impl IntoExpr) -> Self;
}

impl AssocConstBuilder for AssocConst {
    fn new(ident: impl IntoIdent, value: impl IntoExpr) -> Self {
        assoc_const(ident, value)
    }
}

pub fn constraint<B: IntoTypeParamBound>(
    ident: impl IntoIdent,
    bounds: impl IntoIterator<Item = B>,
) -> Constraint {
    Constraint {
        ident: ident.into_ident(),
        generics: None,
        colon_token: Default::default(),
        bounds: FromIterator::from_iter(
            bounds
                .into_iter()
                .map(IntoTypeParamBound::into_type_param_bound),
        ),
    }
}

generics_builder!(Constraint);

pub trait ConstraintBuilder: PathGenericsBuilder {
    fn new<B: IntoTypeParamBound>(
        ident: impl IntoIdent,
        bounds: impl IntoIterator<Item = B>,
    ) -> Self;
}

impl ConstraintBuilder for Constraint {
    fn new<B: IntoTypeParamBound>(
        ident: impl IntoIdent,
        bounds: impl IntoIterator<Item = B>,
    ) -> Self {
        constraint(ident, bounds)
    }
}

pub fn parenthesized_generic_arguments<I: IntoType>(
    inputs: impl IntoIterator<Item = I>,
) -> ParenthesizedGenericArguments {
    ParenthesizedGenericArguments {
        paren_token: Default::default(),
        inputs: FromIterator::from_iter(inputs.into_iter().map(IntoType::into_type)),
        output: ReturnType::Default,
    }
}

output_builder!(ParenthesizedGenericArguments);

pub trait ParenthesizedGenericArgumentsBuilder: OutputPropsBuilder {
    fn new<I: IntoType>(inputs: impl IntoIterator<Item = I>) -> Self;
}

impl ParenthesizedGenericArgumentsBuilder for ParenthesizedGenericArguments {
    fn new<I: IntoType>(inputs: impl IntoIterator<Item = I>) -> Self {
        parenthesized_generic_arguments(inputs)
    }
}

impl IntoPathArguments for ParenthesizedGenericArguments {
    fn into_path_arguments(self) -> PathArguments {
        PathArguments::Parenthesized(self)
    }
}

pub fn q_self(ty: impl IntoType, position: impl Into<usize>) -> QSelf {
    QSelf {
        lt_token: Default::default(),
        ty: ty.into_type().into(),
        position: position.into(),
        as_token: None,
        gt_token: Default::default(),
    }
}

pub trait QSelfBuilder {
    fn new(ty: impl IntoType, position: impl Into<usize>) -> Self;
    #[allow(clippy::wrong_self_convention)]
    fn as_token(self, as_token: bool) -> Self;
}

impl QSelfBuilder for QSelf {
    fn new(ty: impl IntoType, position: impl Into<usize>) -> Self {
        q_self(ty, position)
    }

    fn as_token(self, as_token: bool) -> Self {
        Self {
            as_token: as_token.then(Default::default),
            ..self
        }
    }
}
