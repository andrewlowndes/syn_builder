use crate::{
    attrs_builder,
    macros::{AttrsPropsBuilder, MutabilityPropsBuilder, OutputPropsBuilder, QSelfPropsBuilder},
    mutability_builder, output_builder, qself_builder, IntoExpr, IntoIdent, IntoPath,
    IntoTypeParamBound,
};
use proc_macro2::TokenStream;
use syn::{
    Abi, BareFnArg, BareVariadic, BoundLifetimes, Lifetime, LitStr, Macro, ReturnType, Type,
    TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeInfer, TypeMacro, TypeNever, TypeParen,
    TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple,
};

pub trait IntoType {
    fn into_type(self) -> Type;
}

impl IntoType for Type {
    fn into_type(self) -> Type {
        self
    }
}

macro_rules! impl_into_type {
    ($($target:ident($type:ty),)*) => {
        $(
            impl IntoType for $type {
                fn into_type(self) -> Type {
                    Type::$target(self)
                }
            }
        )*
    };
}

impl_into_type!(
    Array(TypeArray),
    BareFn(TypeBareFn),
    Group(TypeGroup),
    ImplTrait(TypeImplTrait),
    Infer(TypeInfer),
    Macro(TypeMacro),
    Never(TypeNever),
    Paren(TypeParen),
    Path(TypePath),
    Ptr(TypePtr),
    Reference(TypeReference),
    Slice(TypeSlice),
    TraitObject(TypeTraitObject),
    Tuple(TypeTuple),
    Verbatim(TokenStream),
);

pub fn type_array(elem: impl IntoType, len: impl IntoExpr) -> TypeArray {
    TypeArray {
        bracket_token: Default::default(),
        elem: elem.into_type().into(),
        semi_token: Default::default(),
        len: len.into_expr(),
    }
}

pub trait TypeArrayBuilder {
    fn new(elem: impl IntoType, len: impl IntoExpr) -> Self;
}

impl TypeArrayBuilder for TypeArray {
    fn new(elem: impl IntoType, len: impl IntoExpr) -> Self {
        type_array(elem, len)
    }
}

pub fn type_bare_fn<I: Into<BareFnArg>>(inputs: impl IntoIterator<Item = I>) -> TypeBareFn {
    TypeBareFn {
        lifetimes: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        paren_token: Default::default(),
        inputs: FromIterator::from_iter(inputs.into_iter().map(Into::into)),
        variadic: None,
        output: ReturnType::Default,
    }
}

output_builder!(TypeBareFn);

pub trait TypeBareFnBuilder: OutputPropsBuilder {
    fn new<I: Into<BareFnArg>>(inputs: impl IntoIterator<Item = I>) -> Self;
    fn lifetimes(self, lifetimes: impl Into<BoundLifetimes>) -> Self;
    fn unsafety(self, unsafety: bool) -> Self;
    fn abi(self, abi: impl Into<Abi>) -> Self;
    fn variadic(self, variadic: impl Into<BareVariadic>) -> Self;
}

impl TypeBareFnBuilder for TypeBareFn {
    fn new<I: Into<BareFnArg>>(inputs: impl IntoIterator<Item = I>) -> Self {
        type_bare_fn(inputs)
    }

    fn lifetimes(self, lifetimes: impl Into<BoundLifetimes>) -> Self {
        Self {
            lifetimes: Some(lifetimes.into()),
            ..self
        }
    }

    fn unsafety(self, unsafety: bool) -> Self {
        Self {
            unsafety: unsafety.then(Default::default),
            ..self
        }
    }

    fn abi(self, abi: impl Into<Abi>) -> Self {
        Self {
            abi: Some(abi.into()),
            ..self
        }
    }

    fn variadic(self, variadic: impl Into<BareVariadic>) -> Self {
        Self {
            variadic: Some(variadic.into()),
            ..self
        }
    }
}

pub fn type_group(elem: impl IntoType) -> TypeGroup {
    TypeGroup {
        group_token: Default::default(),
        elem: elem.into_type().into(),
    }
}

pub trait TypeGroupBuilder {
    fn new(elem: impl IntoType) -> Self;
}

impl TypeGroupBuilder for TypeGroup {
    fn new(elem: impl IntoType) -> Self {
        type_group(elem)
    }
}

pub fn type_impl_trait<B: IntoTypeParamBound>(
    bounds: impl IntoIterator<Item = B>,
) -> TypeImplTrait {
    TypeImplTrait {
        impl_token: Default::default(),
        bounds: FromIterator::from_iter(
            bounds
                .into_iter()
                .map(IntoTypeParamBound::into_type_param_bound),
        ),
    }
}

pub trait TypeImplTraitBuilder {
    fn new<B: IntoTypeParamBound>(bounds: impl IntoIterator<Item = B>) -> Self;
}

impl TypeImplTraitBuilder for TypeImplTrait {
    fn new<B: IntoTypeParamBound>(bounds: impl IntoIterator<Item = B>) -> Self {
        type_impl_trait(bounds)
    }
}

pub fn type_infer() -> TypeInfer {
    TypeInfer {
        underscore_token: Default::default(),
    }
}

pub trait TypeInferBuilder {
    fn new() -> Self;
}

impl TypeInferBuilder for TypeInfer {
    fn new() -> Self {
        type_infer()
    }
}

pub fn type_macro(mac: impl Into<Macro>) -> TypeMacro {
    TypeMacro { mac: mac.into() }
}

pub trait TypeMacroBuilder {
    fn new(mac: impl Into<Macro>) -> Self;
}

impl TypeMacroBuilder for TypeMacro {
    fn new(mac: impl Into<Macro>) -> Self {
        type_macro(mac)
    }
}

pub fn type_never() -> TypeNever {
    TypeNever {
        bang_token: Default::default(),
    }
}

pub trait TypeNeverBuilder {
    fn new() -> Self;
}

impl TypeNeverBuilder for TypeNever {
    fn new() -> Self {
        type_never()
    }
}

pub fn type_paren(elem: impl IntoType) -> TypeParen {
    TypeParen {
        paren_token: Default::default(),
        elem: elem.into_type().into(),
    }
}

pub trait TypeParenBuilder {
    fn new(elem: impl IntoType) -> Self;
}

impl TypeParenBuilder for TypeParen {
    fn new(elem: impl IntoType) -> Self {
        type_paren(elem)
    }
}

pub fn type_path(path: impl IntoPath) -> TypePath {
    TypePath {
        qself: None,
        path: path.into_path(),
    }
}

qself_builder!(TypePath);

pub trait TypePathBuilder: QSelfPropsBuilder {
    fn new(path: impl IntoPath) -> Self;
}

impl TypePathBuilder for TypePath {
    fn new(path: impl IntoPath) -> Self {
        type_path(path)
    }
}

pub fn type_ptr_const(elem: impl IntoType) -> TypePtr {
    TypePtr {
        star_token: Default::default(),
        const_token: Some(Default::default()),
        mutability: None,
        elem: elem.into_type().into(),
    }
}

pub fn type_ptr_mut(elem: impl IntoType) -> TypePtr {
    TypePtr {
        star_token: Default::default(),
        const_token: None,
        mutability: Some(Default::default()),
        elem: elem.into_type().into(),
    }
}

pub trait TypePtrBuilder {
    fn new_const(elem: impl IntoType) -> Self;
    fn new_mut(elem: impl IntoType) -> Self;
}

impl TypePtrBuilder for TypePtr {
    fn new_const(elem: impl IntoType) -> Self {
        type_ptr_const(elem)
    }
    fn new_mut(elem: impl IntoType) -> Self {
        type_ptr_mut(elem)
    }
}

pub fn type_reference(elem: impl IntoType) -> TypeReference {
    TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: elem.into_type().into(),
    }
}

mutability_builder!(TypeReference);

pub trait TypeReferenceBuilder: MutabilityPropsBuilder {
    fn new(elem: impl IntoType) -> Self;
    fn lifetime(self, lifetime: impl Into<Lifetime>) -> Self;
}

impl TypeReferenceBuilder for TypeReference {
    fn new(elem: impl IntoType) -> Self {
        type_reference(elem)
    }

    fn lifetime(self, lifetime: impl Into<Lifetime>) -> Self {
        Self {
            lifetime: Some(lifetime.into()),
            ..self
        }
    }
}

pub fn type_slice(elem: impl IntoType) -> TypeSlice {
    TypeSlice {
        bracket_token: Default::default(),
        elem: elem.into_type().into(),
    }
}

pub trait TypeSliceBuilder {
    fn new(elem: impl IntoType) -> Self;
}

impl TypeSliceBuilder for TypeSlice {
    fn new(elem: impl IntoType) -> Self {
        type_slice(elem)
    }
}

pub fn type_trait_object<B: IntoTypeParamBound>(
    bounds: impl IntoIterator<Item = B>,
) -> TypeTraitObject {
    TypeTraitObject {
        dyn_token: Default::default(),
        bounds: FromIterator::from_iter(
            bounds
                .into_iter()
                .map(IntoTypeParamBound::into_type_param_bound),
        ),
    }
}

pub trait TypeTraitObjectBuilder {
    fn new<B: IntoTypeParamBound>(bounds: impl IntoIterator<Item = B>) -> Self;
}

impl TypeTraitObjectBuilder for TypeTraitObject {
    fn new<B: IntoTypeParamBound>(bounds: impl IntoIterator<Item = B>) -> Self {
        type_trait_object(bounds)
    }
}

pub fn type_tuple<E: IntoType>(elems: impl IntoIterator<Item = E>) -> TypeTuple {
    TypeTuple {
        paren_token: Default::default(),
        elems: FromIterator::from_iter(elems.into_iter().map(IntoType::into_type)),
    }
}

pub trait TypeTupleBuilder {
    fn new<E: IntoType>(elems: impl IntoIterator<Item = E>) -> Self;
}

impl TypeTupleBuilder for TypeTuple {
    fn new<E: IntoType>(elems: impl IntoIterator<Item = E>) -> Self {
        type_tuple(elems)
    }
}

pub fn abi(name: impl Into<LitStr>) -> Abi {
    Abi {
        extern_token: Default::default(),
        name: Some(name.into()),
    }
}

pub trait AbiBuilder {
    fn new(name: impl Into<LitStr>) -> Self;
}

impl AbiBuilder for Abi {
    fn new(name: impl Into<LitStr>) -> Self {
        abi(name)
    }
}

pub fn bare_fn_arg(ty: impl IntoType) -> BareFnArg {
    BareFnArg {
        attrs: Default::default(),
        name: None,
        ty: ty.into_type(),
    }
}

attrs_builder!(BareFnArg);

pub trait BareFnArgBuilder: AttrsPropsBuilder {
    fn new(ty: impl IntoType) -> Self;
    fn name(self, name: impl IntoIdent) -> Self;
}

impl BareFnArgBuilder for BareFnArg {
    fn new(ty: impl IntoType) -> Self {
        bare_fn_arg(ty)
    }

    fn name(self, name: impl IntoIdent) -> Self {
        Self {
            name: Some((name.into_ident(), Default::default())),
            ..self
        }
    }
}

pub fn bare_variadic() -> BareVariadic {
    BareVariadic {
        attrs: Default::default(),
        name: None,
        dots: Default::default(),
        comma: None,
    }
}

attrs_builder!(BareVariadic);

pub trait BareVariadicBuilder: AttrsPropsBuilder {
    fn new() -> Self;
    fn name(self, name: impl IntoIdent) -> Self;
}

impl BareVariadicBuilder for BareVariadic {
    fn new() -> Self {
        bare_variadic()
    }

    fn name(self, name: impl IntoIdent) -> Self {
        Self {
            name: Some((name.into_ident(), Default::default())),
            ..self
        }
    }
}
