use crate::{
    attrs_builder,
    macros::{AttrsPropsBuilder, OutputPropsBuilder, VisPropsBuilder},
    output_builder, type_path, vis_builder, IntoExpr, IntoFields, IntoIdent, IntoPat, IntoPath,
    IntoType, IntoTypeParamBound,
};
use proc_macro2::TokenStream;
use syn::{
    Abi, Block, FieldsNamed, FnArg, ForeignItem, ForeignItemFn, ForeignItemMacro,
    ForeignItemStatic, ForeignItemType, Generics, ImplItem, ImplItemConst, ImplItemFn,
    ImplItemMacro, ImplItemType, Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn,
    ItemForeignMod, ItemImpl, ItemMacro, ItemMod, ItemStatic, ItemStruct, ItemTrait,
    ItemTraitAlias, ItemType, ItemUnion, ItemUse, Lifetime, Macro, PatType, Receiver, ReturnType,
    Signature, StaticMutability, TraitItem, TraitItemConst, TraitItemFn, TraitItemMacro,
    TraitItemType, UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree, Variadic, Variant,
    Visibility,
};

pub trait ItemGenericsPropsBuilder {
    fn generics(self, args: impl Into<Generics>) -> Self;
}

macro_rules! generics_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::item::ItemGenericsPropsBuilder for $name {
                fn generics(self, generics: impl Into<syn::Generics>) -> Self {
                    Self {
                        generics: generics.into(),
                        ..self
                    }
                }
            }
        )*
    }
}

pub trait UnsafetyPropsBuilder {
    fn unsafety(self, unsafety: bool) -> Self;
}

macro_rules! unsafety_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::item::UnsafetyPropsBuilder for $name {
                fn unsafety(self, unsafety: bool) -> Self {
                    Self {
                        unsafety: unsafety.then(|| Default::default()),
                        ..self
                    }
                }
            }
        )*
    }
}

pub trait DefaultnessPropsBuilder {
    fn defaultness(self, defaultness: bool) -> Self;
}

macro_rules! defaultness_builder {
    ($($name:ident),+) => {
        $(
            impl $crate::item::DefaultnessPropsBuilder for $name {
                fn defaultness(self, defaultness: bool) -> Self {
                    Self {
                        defaultness: defaultness.then(|| Default::default()),
                        ..self
                    }
                }
            }
        )*
    }
}

pub trait IntoItem {
    fn into_item(self) -> Item;
}

impl IntoItem for Item {
    fn into_item(self) -> Item {
        self
    }
}

macro_rules! impl_into_item {
    ($($target:ident($type:ty),)*) => {
        $(
            impl IntoItem for $type {
                fn into_item(self) -> Item {
                    Item::$target(self)
                }
            }
        )*
    };
}

impl_into_item!(
    Const(ItemConst),
    Enum(ItemEnum),
    ExternCrate(ItemExternCrate),
    Fn(ItemFn),
    ForeignMod(ItemForeignMod),
    Impl(ItemImpl),
    Macro(ItemMacro),
    Mod(ItemMod),
    Static(ItemStatic),
    Struct(ItemStruct),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Use(ItemUse),
    Verbatim(TokenStream),
);

pub fn item_const(ident: impl IntoIdent, ty: impl IntoType, expr: impl IntoExpr) -> ItemConst {
    ItemConst {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        const_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        colon_token: Default::default(),
        ty: ty.into_type().into(),
        eq_token: Default::default(),
        expr: expr.into_expr().into(),
        semi_token: Default::default(),
    }
}

attrs_builder!(ItemConst);
vis_builder!(ItemConst);
generics_builder!(ItemConst);

pub trait ItemConstBuilder: AttrsPropsBuilder + VisPropsBuilder + ItemGenericsPropsBuilder {
    fn new(ident: impl IntoIdent, ty: impl IntoType, expr: impl IntoExpr) -> Self;
}

impl ItemConstBuilder for ItemConst {
    fn new(ident: impl IntoIdent, ty: impl IntoType, expr: impl IntoExpr) -> Self {
        item_const(ident, ty, expr)
    }
}

pub fn item_enum(ident: impl IntoIdent) -> ItemEnum {
    ItemEnum {
        attrs: Default::default(),
        vis: Visibility::Public(Default::default()),
        enum_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        brace_token: Default::default(),
        variants: Default::default(),
    }
}

attrs_builder!(ItemEnum);
vis_builder!(ItemEnum);
generics_builder!(ItemEnum);

pub trait ItemEnumBuilder: AttrsPropsBuilder + VisPropsBuilder + ItemGenericsPropsBuilder {
    fn new(ident: impl IntoIdent) -> Self;
    fn variant(self, variant: impl Into<Variant>) -> Self;
    fn variants<V: Into<Variant>>(self, variants: impl IntoIterator<Item = V>) -> Self;
}

impl ItemEnumBuilder for ItemEnum {
    fn new(ident: impl IntoIdent) -> Self {
        item_enum(ident)
    }

    fn variant(mut self, variant: impl Into<Variant>) -> Self {
        self.variants.push(variant.into());
        self
    }

    fn variants<V: Into<Variant>>(self, variants: impl IntoIterator<Item = V>) -> Self {
        Self {
            variants: FromIterator::from_iter(variants.into_iter().map(Into::into)),
            ..self
        }
    }
}

pub fn item_extern_crate(ident: impl IntoIdent) -> ItemExternCrate {
    ItemExternCrate {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        extern_token: Default::default(),
        crate_token: Default::default(),
        ident: ident.into_ident(),
        rename: None,
        semi_token: Default::default(),
    }
}

attrs_builder!(ItemExternCrate);
vis_builder!(ItemExternCrate);

pub trait ItemExternCrateBuilder: AttrsPropsBuilder + VisPropsBuilder {
    fn new(ident: impl IntoIdent) -> Self;
    fn rename(self, rename: impl IntoIdent) -> Self;
}

impl ItemExternCrateBuilder for ItemExternCrate {
    fn new(ident: impl IntoIdent) -> Self {
        item_extern_crate(ident)
    }

    fn rename(self, rename: impl IntoIdent) -> Self {
        Self {
            rename: Some((Default::default(), rename.into_ident())),
            ..self
        }
    }
}

pub fn item_fn(sig: impl Into<Signature>, block: impl Into<Block>) -> ItemFn {
    ItemFn {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        sig: sig.into(),
        block: block.into().into(),
    }
}

attrs_builder!(ItemFn);
vis_builder!(ItemFn);

pub trait ItemFnBuilder: AttrsPropsBuilder + VisPropsBuilder {
    fn new(sig: impl Into<Signature>, block: impl Into<Block>) -> Self;
}

impl ItemFnBuilder for ItemFn {
    fn new(sig: impl Into<Signature>, block: impl Into<Block>) -> Self {
        item_fn(sig, block)
    }
}

pub fn item_foreign_mod(abi: impl Into<Abi>) -> ItemForeignMod {
    ItemForeignMod {
        attrs: Default::default(),
        unsafety: None,
        abi: abi.into(),
        brace_token: Default::default(),
        items: Default::default(),
    }
}

attrs_builder!(ItemForeignMod);
unsafety_builder!(ItemForeignMod);

pub trait ItemForeignModBuilder: AttrsPropsBuilder + UnsafetyPropsBuilder {
    fn new(abi: impl Into<Abi>) -> Self;
    fn items<I: IntoForeignItem>(self, items: impl IntoIterator<Item = I>) -> Self;
    fn item(self, item: impl IntoForeignItem) -> Self;
}

impl ItemForeignModBuilder for ItemForeignMod {
    fn new(abi: impl Into<Abi>) -> Self {
        item_foreign_mod(abi)
    }

    fn items<I: IntoForeignItem>(self, items: impl IntoIterator<Item = I>) -> Self {
        Self {
            items: items
                .into_iter()
                .map(IntoForeignItem::into_foreign_item)
                .collect(),
            ..self
        }
    }

    fn item(mut self, item: impl IntoForeignItem) -> Self {
        self.items.push(item.into_foreign_item());
        self
    }
}

pub fn item_impl(self_ty: impl IntoType) -> ItemImpl {
    ItemImpl {
        attrs: Default::default(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: Default::default(),
        trait_: None,
        self_ty: self_ty.into_type().into(),
        brace_token: Default::default(),
        items: Default::default(),
    }
}

attrs_builder!(ItemImpl);
defaultness_builder!(ItemImpl);
unsafety_builder!(ItemImpl);
generics_builder!(ItemImpl);

pub trait ItemImplBuilder:
    AttrsPropsBuilder + DefaultnessPropsBuilder + UnsafetyPropsBuilder + ItemGenericsPropsBuilder
{
    fn new(self_ty: impl IntoType) -> Self;
    fn trait_(self, bang: bool, path: impl IntoPath) -> Self;
    fn items<I: IntoImplItem>(self, items: impl IntoIterator<Item = I>) -> Self;
    fn item(self, item: impl IntoImplItem) -> Self;
}

impl ItemImplBuilder for ItemImpl {
    fn new(self_ty: impl IntoType) -> Self {
        item_impl(self_ty)
    }

    fn trait_(self, bang: bool, path: impl IntoPath) -> Self {
        Self {
            trait_: Some((
                bang.then(Default::default),
                path.into_path(),
                Default::default(),
            )),
            ..self
        }
    }

    fn items<I: IntoImplItem>(self, items: impl IntoIterator<Item = I>) -> Self {
        Self {
            items: items
                .into_iter()
                .map(IntoImplItem::into_impl_item)
                .collect(),
            ..self
        }
    }

    fn item(mut self, item: impl IntoImplItem) -> Self {
        self.items.push(item.into_impl_item());
        self
    }
}

pub fn item_macro(ident: impl IntoIdent, mac: impl Into<Macro>) -> ItemMacro {
    ItemMacro {
        attrs: Default::default(),
        ident: Some(ident.into_ident()),
        mac: mac.into(),
        semi_token: None,
    }
}

attrs_builder!(ItemMacro);

pub trait ItemMacroBuilder: AttrsPropsBuilder {
    fn new(ident: impl IntoIdent, mac: impl Into<Macro>) -> Self;
}

impl ItemMacroBuilder for ItemMacro {
    fn new(ident: impl IntoIdent, mac: impl Into<Macro>) -> Self {
        item_macro(ident, mac)
    }
}

pub fn item_mod(ident: impl IntoIdent) -> ItemMod {
    ItemMod {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        unsafety: None,
        mod_token: Default::default(),
        ident: ident.into_ident(),
        content: None,
        semi: None,
    }
}

attrs_builder!(ItemMod);
vis_builder!(ItemMod);
unsafety_builder!(ItemMod);

pub trait ItemModBuilder: AttrsPropsBuilder + VisPropsBuilder + UnsafetyPropsBuilder {
    fn new(ident: impl IntoIdent) -> Self;
    fn content<I: IntoItem>(self, items: impl IntoIterator<Item = I>) -> Self;
}

impl ItemModBuilder for ItemMod {
    fn new(ident: impl IntoIdent) -> Self {
        item_mod(ident)
    }

    fn content<I: IntoItem>(self, items: impl IntoIterator<Item = I>) -> Self {
        Self {
            content: Some((
                Default::default(),
                items.into_iter().map(IntoItem::into_item).collect(),
            )),
            ..self
        }
    }
}

pub fn item_static(ident: impl IntoIdent, ty: impl IntoType, expr: impl IntoExpr) -> ItemStatic {
    ItemStatic {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        static_token: Default::default(),
        mutability: StaticMutability::None,
        ident: ident.into_ident(),
        colon_token: Default::default(),
        ty: ty.into_type().into(),
        eq_token: Default::default(),
        expr: expr.into_expr().into(),
        semi_token: Default::default(),
    }
}

attrs_builder!(ItemStatic);
vis_builder!(ItemStatic);

pub trait ItemStaticBuilder: AttrsPropsBuilder + VisPropsBuilder {
    fn new(ident: impl IntoIdent, ty: impl IntoType, expr: impl IntoExpr) -> Self;
    fn mutability(self, mutability: bool) -> Self;
}

impl ItemStaticBuilder for ItemStatic {
    fn new(ident: impl IntoIdent, ty: impl IntoType, expr: impl IntoExpr) -> Self {
        item_static(ident, ty, expr)
    }

    fn mutability(self, mutability: bool) -> Self {
        Self {
            mutability: if mutability {
                StaticMutability::Mut(Default::default())
            } else {
                StaticMutability::None
            },
            ..self
        }
    }
}

pub fn item_struct(ident: impl IntoIdent, fields: impl IntoFields) -> ItemStruct {
    ItemStruct {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        struct_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        fields: fields.into_fields(),
        semi_token: None,
    }
}

attrs_builder!(ItemStruct);
vis_builder!(ItemStruct);
generics_builder!(ItemStruct);

pub trait ItemStructBuilder:
    AttrsPropsBuilder + VisPropsBuilder + ItemGenericsPropsBuilder
{
    fn new(ident: impl IntoIdent, fields: impl IntoFields) -> Self;
}

impl ItemStructBuilder for ItemStruct {
    fn new(ident: impl IntoIdent, fields: impl IntoFields) -> Self {
        item_struct(ident, fields)
    }
}

pub fn item_trait(ident: impl IntoIdent) -> ItemTrait {
    ItemTrait {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        unsafety: None,
        auto_token: None,
        restriction: None,
        trait_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        colon_token: None,
        supertraits: Default::default(),
        brace_token: Default::default(),
        items: Default::default(),
    }
}

attrs_builder!(ItemTrait);
vis_builder!(ItemTrait);
unsafety_builder!(ItemTrait);
generics_builder!(ItemTrait);

pub trait ItemTraitBuilder:
    AttrsPropsBuilder + VisPropsBuilder + UnsafetyPropsBuilder + ItemGenericsPropsBuilder
{
    fn new(ident: impl IntoIdent) -> Self;
    fn auto(self, auto: bool) -> Self;
    fn colon_token(self, colon_token: bool) -> Self;
    fn supertraits<S: IntoTypeParamBound>(self, supertraits: impl IntoIterator<Item = S>) -> Self;
    fn items<I: IntoTraitItem>(self, items: impl IntoIterator<Item = I>) -> Self;
    fn item(self, item: impl IntoTraitItem) -> Self;
}

impl ItemTraitBuilder for ItemTrait {
    fn new(ident: impl IntoIdent) -> Self {
        item_trait(ident)
    }

    fn auto(self, auto: bool) -> Self {
        Self {
            auto_token: auto.then(Default::default),
            ..self
        }
    }

    fn colon_token(self, colon_token: bool) -> Self {
        Self {
            colon_token: colon_token.then(Default::default),
            ..self
        }
    }

    fn supertraits<S: IntoTypeParamBound>(self, supertraits: impl IntoIterator<Item = S>) -> Self {
        Self {
            supertraits: FromIterator::from_iter(
                supertraits
                    .into_iter()
                    .map(IntoTypeParamBound::into_type_param_bound),
            ),
            ..self
        }
    }

    fn items<I: IntoTraitItem>(self, items: impl IntoIterator<Item = I>) -> Self {
        Self {
            items: items
                .into_iter()
                .map(IntoTraitItem::into_trait_item)
                .collect(),
            ..self
        }
    }

    fn item(mut self, item: impl IntoTraitItem) -> Self {
        self.items.push(item.into_trait_item());
        self
    }
}

pub fn item_trait_alias<B: IntoTypeParamBound>(
    ident: impl IntoIdent,
    bounds: impl IntoIterator<Item = B>,
) -> ItemTraitAlias {
    ItemTraitAlias {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        trait_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        eq_token: Default::default(),
        bounds: FromIterator::from_iter(
            bounds
                .into_iter()
                .map(IntoTypeParamBound::into_type_param_bound),
        ),
        semi_token: Default::default(),
    }
}

attrs_builder!(ItemTraitAlias);
vis_builder!(ItemTraitAlias);
generics_builder!(ItemTraitAlias);

pub trait ItemTraitAliasBuilder:
    AttrsPropsBuilder + VisPropsBuilder + ItemGenericsPropsBuilder
{
    fn new<B: IntoTypeParamBound>(
        ident: impl IntoIdent,
        bounds: impl IntoIterator<Item = B>,
    ) -> Self;
}

impl ItemTraitAliasBuilder for ItemTraitAlias {
    fn new<B: IntoTypeParamBound>(
        ident: impl IntoIdent,
        bounds: impl IntoIterator<Item = B>,
    ) -> Self {
        item_trait_alias(ident, bounds)
    }
}

pub fn item_type(ident: impl IntoIdent, ty: impl IntoType) -> ItemType {
    ItemType {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        type_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        eq_token: Default::default(),
        ty: ty.into_type().into(),
        semi_token: Default::default(),
    }
}

attrs_builder!(ItemType);
vis_builder!(ItemType);
generics_builder!(ItemType);

pub trait ItemTypeBuilder: AttrsPropsBuilder + VisPropsBuilder + ItemGenericsPropsBuilder {
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self;
}

impl ItemTypeBuilder for ItemType {
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self {
        item_type(ident, ty)
    }
}

pub fn item_union(ident: impl IntoIdent, fields: impl Into<FieldsNamed>) -> ItemUnion {
    ItemUnion {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        union_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        fields: fields.into(),
    }
}

attrs_builder!(ItemUnion);
vis_builder!(ItemUnion);
generics_builder!(ItemUnion);

pub trait ItemUnionBuilder: AttrsPropsBuilder + VisPropsBuilder + ItemGenericsPropsBuilder {
    fn new(ident: impl IntoIdent, fields: impl Into<FieldsNamed>) -> Self;
}

impl ItemUnionBuilder for ItemUnion {
    fn new(ident: impl IntoIdent, fields: impl Into<FieldsNamed>) -> Self {
        item_union(ident, fields)
    }
}

pub fn item_use(tree: impl IntoUseTree) -> ItemUse {
    ItemUse {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree: tree.into_use_tree(),
        semi_token: Default::default(),
    }
}

attrs_builder!(ItemUse);
vis_builder!(ItemUse);

pub trait ItemUseBuilder: AttrsPropsBuilder + VisPropsBuilder {
    fn new(tree: impl IntoUseTree) -> Self;
    fn leading(self, leading: bool) -> Self;
}

impl ItemUseBuilder for ItemUse {
    fn new(tree: impl IntoUseTree) -> Self {
        item_use(tree)
    }

    fn leading(self, leading: bool) -> Self {
        Self {
            leading_colon: leading.then(Default::default),
            ..self
        }
    }
}

pub trait IntoUseTree {
    fn into_use_tree(self) -> UseTree;
}

impl IntoUseTree for UseTree {
    fn into_use_tree(self) -> UseTree {
        self
    }
}

macro_rules! impl_into_use_tree {
    ($($target:ident($type:ty),)*) => {
        $(
            impl IntoUseTree for $type {
                fn into_use_tree(self) -> UseTree {
                    UseTree::$target(self)
                }
            }
        )*
    };
}

impl_into_use_tree!(
    Path(UsePath),
    Name(UseName),
    Rename(UseRename),
    Glob(UseGlob),
    Group(UseGroup),
);

pub fn use_path(ident: impl IntoIdent, tree: impl IntoUseTree) -> UsePath {
    UsePath {
        ident: ident.into_ident(),
        colon2_token: Default::default(),
        tree: tree.into_use_tree().into(),
    }
}

pub trait UsePathBuilder {
    fn new(ident: impl IntoIdent, tree: impl IntoUseTree) -> Self;
}

impl UsePathBuilder for UsePath {
    fn new(ident: impl IntoIdent, tree: impl IntoUseTree) -> Self {
        use_path(ident, tree)
    }
}

pub fn use_name(ident: impl IntoIdent) -> UseName {
    UseName {
        ident: ident.into_ident(),
    }
}

pub trait UseNameBuilder {
    fn new(ident: impl IntoIdent) -> Self;
}

impl UseNameBuilder for UseName {
    fn new(ident: impl IntoIdent) -> Self {
        use_name(ident)
    }
}

pub fn use_rename(name: impl IntoIdent, rename: impl IntoIdent) -> UseRename {
    UseRename {
        ident: name.into_ident(),
        as_token: Default::default(),
        rename: rename.into_ident(),
    }
}

pub trait UseRenamBuilder {
    fn new(name: impl IntoIdent, rename: impl IntoIdent) -> Self;
}

impl UseRenamBuilder for UseRename {
    fn new(name: impl IntoIdent, rename: impl IntoIdent) -> Self {
        use_rename(name, rename)
    }
}

pub fn use_glob() -> UseGlob {
    UseGlob {
        star_token: Default::default(),
    }
}

pub trait UseGlobBuilder {
    fn new() -> Self;
}

impl UseGlobBuilder for UseGlob {
    fn new() -> Self {
        use_glob()
    }
}

pub fn use_group<I: IntoUseTree>(items: impl IntoIterator<Item = I>) -> UseGroup {
    UseGroup {
        brace_token: Default::default(),
        items: FromIterator::from_iter(items.into_iter().map(IntoUseTree::into_use_tree)),
    }
}

pub trait UseGroupBuilder {
    fn new<I: IntoUseTree>(items: impl IntoIterator<Item = I>) -> Self;
}

impl UseGroupBuilder for UseGroup {
    fn new<I: IntoUseTree>(items: impl IntoIterator<Item = I>) -> Self {
        use_group(items)
    }
}

pub trait IntoForeignItem {
    fn into_foreign_item(self) -> ForeignItem;
}

impl IntoForeignItem for ForeignItem {
    fn into_foreign_item(self) -> ForeignItem {
        self
    }
}

macro_rules! impl_into_foreign_item {
    ($($target:ident($type:ty),)*) => {
        $(
            impl IntoForeignItem for $type {
                fn into_foreign_item(self) -> ForeignItem {
                    ForeignItem::$target(self)
                }
            }
        )*
    };
}

impl_into_foreign_item!(
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
    Verbatim(TokenStream),
);

pub fn foreign_item_fn(sig: impl Into<Signature>) -> ForeignItemFn {
    ForeignItemFn {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        sig: sig.into(),
        semi_token: Default::default(),
    }
}

attrs_builder!(ForeignItemFn);
vis_builder!(ForeignItemFn);

pub trait ForeignItemFnBuilder: AttrsPropsBuilder + VisPropsBuilder {
    fn new(sig: impl Into<Signature>) -> Self;
}

impl ForeignItemFnBuilder for ForeignItemFn {
    fn new(sig: impl Into<Signature>) -> Self {
        foreign_item_fn(sig)
    }
}

pub fn foreign_item_static(ident: impl IntoIdent, ty: impl IntoType) -> ForeignItemStatic {
    ForeignItemStatic {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        static_token: Default::default(),
        mutability: StaticMutability::None,
        ident: ident.into_ident(),
        colon_token: Default::default(),
        ty: ty.into_type().into(),
        semi_token: Default::default(),
    }
}

attrs_builder!(ForeignItemStatic);
vis_builder!(ForeignItemStatic);

pub trait ForeignItemStaticBuilder: AttrsPropsBuilder + VisPropsBuilder {
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self;
    fn mutability(self, mutability: bool) -> Self;
}

impl ForeignItemStaticBuilder for ForeignItemStatic {
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self {
        foreign_item_static(ident, ty)
    }

    fn mutability(self, mutability: bool) -> Self {
        Self {
            mutability: if mutability {
                StaticMutability::Mut(Default::default())
            } else {
                StaticMutability::None
            },
            ..self
        }
    }
}

pub fn foreign_item_type(ident: impl IntoIdent) -> ForeignItemType {
    ForeignItemType {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        type_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        semi_token: Default::default(),
    }
}

attrs_builder!(ForeignItemType);
vis_builder!(ForeignItemType);
generics_builder!(ForeignItemType);

pub trait ForeignItemTypeBuilder:
    AttrsPropsBuilder + VisPropsBuilder + ItemGenericsPropsBuilder
{
    fn new(ident: impl IntoIdent) -> Self;
}

impl ForeignItemTypeBuilder for ForeignItemType {
    fn new(ident: impl IntoIdent) -> Self {
        foreign_item_type(ident)
    }
}

pub fn foreign_item_macro(mac: impl Into<Macro>) -> ForeignItemMacro {
    ForeignItemMacro {
        attrs: Default::default(),
        mac: mac.into(),
        semi_token: None,
    }
}

attrs_builder!(ForeignItemMacro);

pub trait ForeignItemMacroBuilder: AttrsPropsBuilder {
    fn new(mac: impl Into<Macro>) -> Self;
}

impl ForeignItemMacroBuilder for ForeignItemMacro {
    fn new(mac: impl Into<Macro>) -> Self {
        foreign_item_macro(mac)
    }
}

pub trait IntoTraitItem {
    fn into_trait_item(self) -> TraitItem;
}

impl IntoTraitItem for TraitItem {
    fn into_trait_item(self) -> TraitItem {
        self
    }
}

macro_rules! impl_into_trait_item {
    ($($target:ident($type:ty),)*) => {
        $(
            impl IntoTraitItem for $type {
                fn into_trait_item(self) -> TraitItem {
                    TraitItem::$target(self)
                }
            }
        )*
    };
}

impl_into_trait_item!(
    Const(TraitItemConst),
    Fn(TraitItemFn),
    Type(TraitItemType),
    Macro(TraitItemMacro),
    Verbatim(TokenStream),
);

pub fn trait_item_const(ident: impl IntoIdent, ty: impl IntoType) -> TraitItemConst {
    TraitItemConst {
        attrs: Default::default(),
        const_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        colon_token: Default::default(),
        ty: ty.into_type(),
        default: None,
        semi_token: Default::default(),
    }
}

attrs_builder!(TraitItemConst);
generics_builder!(TraitItemConst);

pub trait TraitItemConstBuilder: AttrsPropsBuilder + ItemGenericsPropsBuilder {
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self;
    fn default(self, expr: impl IntoExpr) -> Self;
}

impl TraitItemConstBuilder for TraitItemConst {
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self {
        trait_item_const(ident, ty)
    }

    fn default(self, expr: impl IntoExpr) -> Self {
        Self {
            default: Some((Default::default(), expr.into_expr())),
            ..self
        }
    }
}

pub fn trait_item_fn(sig: impl Into<Signature>) -> TraitItemFn {
    TraitItemFn {
        attrs: Default::default(),
        sig: sig.into(),
        default: None,
        semi_token: Default::default(),
    }
}

attrs_builder!(TraitItemFn);

pub trait TraitItemFnBuilder: AttrsPropsBuilder {
    fn new(sig: impl Into<Signature>) -> Self;
    fn default(self, block: impl Into<Block>) -> Self;
}

impl TraitItemFnBuilder for TraitItemFn {
    fn new(sig: impl Into<Signature>) -> Self {
        trait_item_fn(sig)
    }

    fn default(self, block: impl Into<Block>) -> Self {
        Self {
            default: Some(block.into()),
            ..self
        }
    }
}

pub fn trait_item_type(ident: impl IntoIdent) -> TraitItemType {
    TraitItemType {
        attrs: Default::default(),
        type_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        colon_token: None,
        bounds: Default::default(),
        default: None,
        semi_token: Default::default(),
    }
}

attrs_builder!(TraitItemType);

pub trait TraitItemTypeBuilder: AttrsPropsBuilder {
    fn new(ident: impl IntoIdent) -> Self;
    fn bounds<B: IntoTypeParamBound>(self, bounds: impl IntoIterator<Item = B>) -> Self;
    fn default(self, ty: impl IntoType) -> Self;
}

impl TraitItemTypeBuilder for TraitItemType {
    fn new(ident: impl IntoIdent) -> Self {
        trait_item_type(ident)
    }

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

    fn default(self, ty: impl IntoType) -> Self {
        Self {
            default: Some((Default::default(), ty.into_type())),
            ..self
        }
    }
}

pub fn trait_item_macro(mac: impl Into<Macro>) -> TraitItemMacro {
    TraitItemMacro {
        attrs: Default::default(),
        mac: mac.into(),
        semi_token: None,
    }
}

attrs_builder!(TraitItemMacro);

pub trait TraitItemMacroBuilder: AttrsPropsBuilder {
    fn new(mac: impl Into<Macro>) -> Self;
}

impl TraitItemMacroBuilder for TraitItemMacro {
    fn new(mac: impl Into<Macro>) -> Self {
        trait_item_macro(mac)
    }
}

pub trait IntoImplItem {
    fn into_impl_item(self) -> ImplItem;
}

impl IntoImplItem for ImplItem {
    fn into_impl_item(self) -> ImplItem {
        self
    }
}

macro_rules! impl_into_impl_item {
    ($($target:ident($type:ty),)*) => {
        $(
            impl IntoImplItem for $type {
                fn into_impl_item(self) -> ImplItem {
                    ImplItem::$target(self)
                }
            }
        )*
    };
}

impl_into_impl_item!(
    Const(ImplItemConst),
    Fn(ImplItemFn),
    Type(ImplItemType),
    Macro(ImplItemMacro),
    Verbatim(TokenStream),
);

pub fn impl_item_const(
    ident: impl IntoIdent,
    ty: impl IntoType,
    expr: impl IntoExpr,
) -> ImplItemConst {
    ImplItemConst {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        defaultness: None,
        const_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        colon_token: Default::default(),
        ty: ty.into_type(),
        eq_token: Default::default(),
        expr: expr.into_expr(),
        semi_token: Default::default(),
    }
}

attrs_builder!(ImplItemConst);
vis_builder!(ImplItemConst);
defaultness_builder!(ImplItemConst);

pub trait ImplItemConstBuilder:
    AttrsPropsBuilder + VisPropsBuilder + DefaultnessPropsBuilder
{
    fn new(ident: impl IntoIdent, ty: impl IntoType, expr: impl IntoExpr) -> Self;
}

impl ImplItemConstBuilder for ImplItemConst {
    fn new(ident: impl IntoIdent, ty: impl IntoType, expr: impl IntoExpr) -> Self {
        impl_item_const(ident, ty, expr)
    }
}

pub fn impl_item_fn(sig: impl Into<Signature>, block: impl Into<Block>) -> ImplItemFn {
    ImplItemFn {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: sig.into(),
        block: block.into(),
    }
}

attrs_builder!(ImplItemFn);
vis_builder!(ImplItemFn);
defaultness_builder!(ImplItemFn);

pub trait ImplItemFnBuilder: AttrsPropsBuilder + VisPropsBuilder + DefaultnessPropsBuilder {
    fn new(sig: impl Into<Signature>, block: impl Into<Block>) -> Self;
}

impl ImplItemFnBuilder for ImplItemFn {
    fn new(sig: impl Into<Signature>, block: impl Into<Block>) -> Self {
        impl_item_fn(sig, block)
    }
}

pub fn impl_item_type(ident: impl IntoIdent, ty: impl IntoType) -> ImplItemType {
    ImplItemType {
        attrs: Default::default(),
        vis: Visibility::Inherited,
        defaultness: None,
        type_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        eq_token: Default::default(),
        ty: ty.into_type(),
        semi_token: Default::default(),
    }
}

attrs_builder!(ImplItemType);
vis_builder!(ImplItemType);
defaultness_builder!(ImplItemType);
generics_builder!(ImplItemType);

pub trait ImplItemTypeBuilder:
    AttrsPropsBuilder + VisPropsBuilder + DefaultnessPropsBuilder + ItemGenericsPropsBuilder
{
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self;
}

impl ImplItemTypeBuilder for ImplItemType {
    fn new(ident: impl IntoIdent, ty: impl IntoType) -> Self {
        impl_item_type(ident, ty)
    }
}

pub fn impl_item_macro(mac: impl Into<Macro>) -> ImplItemMacro {
    ImplItemMacro {
        attrs: Default::default(),
        mac: mac.into(),
        semi_token: None,
    }
}

attrs_builder!(ImplItemMacro);

pub trait ImplItemMacroBuilder: AttrsPropsBuilder {
    fn new(mac: impl Into<Macro>) -> Self;
}

impl ImplItemMacroBuilder for ImplItemMacro {
    fn new(mac: impl Into<Macro>) -> Self {
        impl_item_macro(mac)
    }
}

pub fn signature<I: IntoFnArg>(
    ident: impl IntoIdent,
    inputs: impl IntoIterator<Item = I>,
) -> Signature {
    Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: ident.into_ident(),
        generics: Default::default(),
        paren_token: Default::default(),
        inputs: FromIterator::from_iter(inputs.into_iter().map(IntoFnArg::into_fn_arg)),
        variadic: None,
        output: ReturnType::Default,
    }
}

unsafety_builder!(Signature);
generics_builder!(Signature);
output_builder!(Signature);

pub trait SignatureBuilder:
    UnsafetyPropsBuilder + ItemGenericsPropsBuilder + OutputPropsBuilder
{
    fn new<I: IntoFnArg>(ident: impl IntoIdent, inputs: impl IntoIterator<Item = I>) -> Self;
    fn constness(self, constness: bool) -> Self;
    fn asyncness(self, asyncness: bool) -> Self;
    fn abi(self, abi: impl Into<Abi>) -> Self;
    fn variadic(self, variadic: impl Into<Variadic>) -> Self;
}

impl SignatureBuilder for Signature {
    fn new<I: IntoFnArg>(ident: impl IntoIdent, inputs: impl IntoIterator<Item = I>) -> Self {
        signature(ident, inputs)
    }
    fn constness(self, constness: bool) -> Self {
        Self {
            constness: constness.then(Default::default),
            ..self
        }
    }

    fn asyncness(self, asyncness: bool) -> Self {
        Self {
            asyncness: asyncness.then(Default::default),
            ..self
        }
    }

    fn abi(self, abi: impl Into<Abi>) -> Self {
        Self {
            abi: Some(abi.into()),
            ..self
        }
    }

    fn variadic(self, variadic: impl Into<Variadic>) -> Self {
        Self {
            variadic: Some(variadic.into()),
            ..self
        }
    }
}

pub trait IntoFnArg {
    fn into_fn_arg(self) -> FnArg;
}

impl IntoFnArg for FnArg {
    fn into_fn_arg(self) -> FnArg {
        self
    }
}

impl IntoFnArg for Receiver {
    fn into_fn_arg(self) -> FnArg {
        FnArg::Receiver(self)
    }
}

impl IntoFnArg for PatType {
    fn into_fn_arg(self) -> FnArg {
        FnArg::Typed(self)
    }
}

pub fn receiver() -> Receiver {
    Receiver {
        attrs: Default::default(),
        reference: None,
        mutability: None,
        self_token: Default::default(),
        colon_token: Some(Default::default()),
        ty: type_path("Self").into_type().into(),
    }
}

attrs_builder!(Receiver);

pub trait ReceiverBuilder: AttrsPropsBuilder {
    fn new() -> Self;
    fn reference(self, reference: bool) -> Self;
    fn lifetime(self, lifetime: impl Into<Lifetime>) -> Self;
    fn mutability(self, mutability: bool) -> Self;
    fn ty(self, ty: impl IntoType) -> Self;
}

impl ReceiverBuilder for Receiver {
    fn new() -> Self {
        receiver()
    }
    fn reference(self, reference: bool) -> Self {
        Self {
            reference: reference.then(|| (Default::default(), None)),
            ..self
        }
    }

    fn lifetime(self, lifetime: impl Into<Lifetime>) -> Self {
        Self {
            reference: Some((Default::default(), Some(lifetime.into()))),
            ..self
        }
    }

    fn mutability(self, mutability: bool) -> Self {
        Self {
            mutability: mutability.then(Default::default),
            ..self
        }
    }

    fn ty(self, ty: impl IntoType) -> Self {
        Self {
            colon_token: Some(Default::default()),
            ty: ty.into_type().into(),
            ..self
        }
    }
}

pub fn variadic() -> Variadic {
    Variadic {
        attrs: Default::default(),
        pat: None,
        dots: Default::default(),
        comma: None,
    }
}

attrs_builder!(Variadic);

pub trait VariadicBuilder: AttrsPropsBuilder {
    fn new() -> Self;
    fn pat(self, pat: impl IntoPat) -> Self;
}

impl VariadicBuilder for Variadic {
    fn new() -> Self {
        variadic()
    }
    fn pat(self, pat: impl IntoPat) -> Self {
        Self {
            pat: Some((pat.into_pat().into(), Default::default())),
            ..self
        }
    }
}

pub fn static_mutability_mut_variant() -> StaticMutability {
    StaticMutability::Mut(Default::default())
}
