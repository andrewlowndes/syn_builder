use crate::{attrs_builder, IntoExpr, IntoIdent, IntoPath, IntoType};
use proc_macro2::TokenStream;
use syn::{
    BoundLifetimes, ConstParam, GenericParam, Generics, Lifetime, LifetimeParam, PredicateLifetime,
    PredicateType, TraitBound, TraitBoundModifier, TypeParam, TypeParamBound, WhereClause,
    WherePredicate,
};

pub fn generics<P: IntoGenericParam>(params: impl IntoIterator<Item = P>) -> Generics {
    Generics {
        lt_token: Some(Default::default()),
        params: FromIterator::from_iter(
            params.into_iter().map(IntoGenericParam::into_generic_param),
        ),
        gt_token: Some(Default::default()),
        ..Default::default()
    }
}

pub trait GenericsBuilder {
    fn where_clause(self, where_clause: impl Into<WhereClause>) -> Self;
}

impl GenericsBuilder for Generics {
    fn where_clause(self, where_clause: impl Into<WhereClause>) -> Self {
        Self {
            where_clause: Some(where_clause.into()),
            ..self
        }
    }
}

pub trait IntoGenericParam {
    fn into_generic_param(self) -> GenericParam;
}

impl IntoGenericParam for GenericParam {
    fn into_generic_param(self) -> GenericParam {
        self
    }
}

pub fn lifetime_param(lifetime: impl Into<Lifetime>) -> LifetimeParam {
    LifetimeParam {
        attrs: Default::default(),
        lifetime: lifetime.into(),
        colon_token: None,
        bounds: Default::default(),
    }
}

attrs_builder!(LifetimeParam);

impl IntoGenericParam for LifetimeParam {
    fn into_generic_param(self) -> GenericParam {
        GenericParam::Lifetime(self)
    }
}

pub trait LifetimeParamBuilder {
    fn bounds<B: Into<Lifetime>>(self, bounds: impl IntoIterator<Item = B>) -> Self;
}

impl LifetimeParamBuilder for LifetimeParam {
    fn bounds<B: Into<Lifetime>>(self, bounds: impl IntoIterator<Item = B>) -> Self {
        Self {
            colon_token: Some(Default::default()),
            bounds: bounds.into_iter().map(Into::into).collect(),
            ..self
        }
    }
}

pub fn type_param(ident: impl IntoIdent) -> TypeParam {
    TypeParam {
        attrs: Default::default(),
        ident: ident.into_ident(),
        colon_token: None,
        bounds: Default::default(),
        eq_token: None,
        default: None,
    }
}

attrs_builder!(TypeParam);

impl IntoGenericParam for TypeParam {
    fn into_generic_param(self) -> GenericParam {
        GenericParam::Type(self)
    }
}

pub trait TypeParamBuilder {
    fn bounds<B: IntoTypeParamBound>(self, bounds: impl IntoIterator<Item = B>) -> Self;
    fn default(self, default: impl IntoType) -> Self;
}

impl TypeParamBuilder for TypeParam {
    fn bounds<B: IntoTypeParamBound>(self, bounds: impl IntoIterator<Item = B>) -> Self {
        Self {
            colon_token: Some(Default::default()),
            bounds: FromIterator::from_iter(
                bounds
                    .into_iter()
                    .map(IntoTypeParamBound::into_type_param_bound),
            ),
            ..self
        }
    }

    fn default(self, default: impl IntoType) -> Self {
        Self {
            eq_token: Some(Default::default()),
            default: Some(default.into_type()),
            ..self
        }
    }
}

pub fn const_param(ident: impl IntoIdent, ty: impl IntoType) -> ConstParam {
    ConstParam {
        attrs: Default::default(),
        const_token: Default::default(),
        ident: ident.into_ident(),
        colon_token: Default::default(),
        ty: ty.into_type(),
        eq_token: None,
        default: None,
    }
}

attrs_builder!(ConstParam);

impl IntoGenericParam for ConstParam {
    fn into_generic_param(self) -> GenericParam {
        GenericParam::Const(self)
    }
}

pub trait ConstParamBuilder {
    fn default(self, default: impl IntoExpr) -> Self;
}

impl ConstParamBuilder for ConstParam {
    fn default(self, default: impl IntoExpr) -> Self {
        Self {
            eq_token: Some(Default::default()),
            default: Some(default.into_expr()),
            ..self
        }
    }
}

pub fn bound_lifetimes<L: IntoGenericParam>(
    lifetimes: impl IntoIterator<Item = L>,
) -> BoundLifetimes {
    BoundLifetimes {
        lifetimes: FromIterator::from_iter(
            lifetimes
                .into_iter()
                .map(IntoGenericParam::into_generic_param),
        ),
        ..Default::default()
    }
}

pub trait IntoTypeParamBound {
    fn into_type_param_bound(self) -> TypeParamBound;
}

impl IntoTypeParamBound for TypeParamBound {
    fn into_type_param_bound(self) -> TypeParamBound {
        self
    }
}

impl IntoTypeParamBound for Lifetime {
    fn into_type_param_bound(self) -> TypeParamBound {
        TypeParamBound::Lifetime(self)
    }
}

impl IntoTypeParamBound for TokenStream {
    fn into_type_param_bound(self) -> TypeParamBound {
        TypeParamBound::Verbatim(self)
    }
}

pub fn trait_bound(path: impl IntoPath) -> TraitBound {
    TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: path.into_path(),
    }
}

impl IntoTypeParamBound for TraitBound {
    fn into_type_param_bound(self) -> TypeParamBound {
        TypeParamBound::Trait(self)
    }
}

pub trait TraitBoundBuilder {
    fn lifetimes(self, lifetimes: impl Into<BoundLifetimes>) -> Self;
    fn modifier(self, maybe: bool) -> Self;
}

impl TraitBoundBuilder for TraitBound {
    fn lifetimes(self, lifetimes: impl Into<BoundLifetimes>) -> Self {
        Self {
            lifetimes: Some(lifetimes.into()),
            ..self
        }
    }

    fn modifier(self, maybe: bool) -> Self {
        Self {
            modifier: if maybe {
                TraitBoundModifier::Maybe(Default::default())
            } else {
                TraitBoundModifier::None
            },
            ..self
        }
    }
}

pub fn where_clause<P: IntoWherePredicate>(predicates: impl IntoIterator<Item = P>) -> WhereClause {
    WhereClause {
        where_token: Default::default(),
        predicates: FromIterator::from_iter(
            predicates
                .into_iter()
                .map(IntoWherePredicate::into_where_predicate),
        ),
    }
}

pub trait IntoWherePredicate {
    fn into_where_predicate(self) -> WherePredicate;
}

impl IntoWherePredicate for WherePredicate {
    fn into_where_predicate(self) -> WherePredicate {
        self
    }
}

pub fn predicate_lifetime<B: Into<Lifetime>>(
    lifetime: impl Into<Lifetime>,
    bounds: impl IntoIterator<Item = B>,
) -> PredicateLifetime {
    PredicateLifetime {
        lifetime: lifetime.into(),
        colon_token: Default::default(),
        bounds: FromIterator::from_iter(bounds.into_iter().map(Into::into)),
    }
}

impl IntoWherePredicate for PredicateLifetime {
    fn into_where_predicate(self) -> WherePredicate {
        WherePredicate::Lifetime(self)
    }
}

pub fn predicate_type<B: IntoTypeParamBound>(
    bounded_ty: impl IntoType,
    bounds: impl IntoIterator<Item = B>,
) -> PredicateType {
    PredicateType {
        lifetimes: None,
        bounded_ty: bounded_ty.into_type(),
        colon_token: Default::default(),
        bounds: FromIterator::from_iter(
            bounds
                .into_iter()
                .map(IntoTypeParamBound::into_type_param_bound),
        ),
    }
}

impl IntoWherePredicate for PredicateType {
    fn into_where_predicate(self) -> WherePredicate {
        WherePredicate::Type(self)
    }
}

pub trait PredicateTypeBuilder {
    fn lifetimes(self, lifetimes: impl Into<BoundLifetimes>) -> Self;
}

impl PredicateTypeBuilder for PredicateType {
    fn lifetimes(self, lifetimes: impl Into<BoundLifetimes>) -> Self {
        Self {
            lifetimes: Some(lifetimes.into()),
            ..self
        }
    }
}
