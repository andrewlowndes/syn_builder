use crate::{
    attrs_builder, label_builder, mutability_builder, output_builder, qself_builder, IntoBinOp,
    IntoIdent, IntoLit, IntoPat, IntoPath, IntoType, IntoUnOp,
};
use proc_macro2::TokenStream;
use syn::{
    token, AngleBracketedGenericArguments, Arm, Block, Expr, ExprArray, ExprAssign, ExprAsync,
    ExprAwait, ExprBinary, ExprBlock, ExprBreak, ExprCall, ExprCast, ExprClosure, ExprConst,
    ExprContinue, ExprField, ExprForLoop, ExprGroup, ExprIf, ExprIndex, ExprInfer, ExprLet,
    ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange,
    ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprUnary,
    ExprUnsafe, ExprWhile, ExprYield, FieldValue, Index, Label, Lifetime, Macro, Member,
    RangeLimits, ReturnType,
};

pub trait IntoExpr {
    fn into_expr(self) -> Expr;
}

impl IntoExpr for Expr {
    fn into_expr(self) -> Expr {
        self
    }
}

macro_rules! impl_into_expr {
    ($($target:ident($type:ty),)*) => {
        $(
            impl IntoExpr for $type {
                fn into_expr(self) -> Expr {
                    Expr::$target(self)
                }
            }
        )*
    };
}

impl_into_expr!(
    Array(ExprArray),
    Assign(ExprAssign),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Const(ExprConst),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Infer(ExprInfer),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    Verbatim(TokenStream),
    While(ExprWhile),
    Yield(ExprYield),
);

pub fn expr_array<I: IntoExpr>(elems: impl IntoIterator<Item = I>) -> ExprArray {
    ExprArray {
        attrs: Default::default(),
        bracket_token: Default::default(),
        elems: FromIterator::from_iter(elems.into_iter().map(IntoExpr::into_expr)),
    }
}

attrs_builder!(ExprArray);

pub fn expr_assign(left: impl IntoExpr, right: impl IntoExpr) -> ExprAssign {
    ExprAssign {
        attrs: Default::default(),
        left: left.into_expr().into(),
        eq_token: Default::default(),
        right: right.into_expr().into(),
    }
}

attrs_builder!(ExprAssign);

pub fn expr_async(block: impl Into<Block>) -> ExprAsync {
    ExprAsync {
        attrs: Default::default(),
        async_token: Default::default(),
        capture: None,
        block: block.into(),
    }
}

attrs_builder!(ExprAsync);

pub trait ExprAsyncBuilder {
    fn capture(self, capture: bool) -> Self;
}

impl ExprAsyncBuilder for ExprAsync {
    fn capture(self, capture: bool) -> Self {
        Self {
            capture: capture.then(Default::default),
            ..self
        }
    }
}

pub fn expr_await(base: impl IntoExpr) -> ExprAwait {
    ExprAwait {
        attrs: Default::default(),
        base: base.into_expr().into(),
        dot_token: Default::default(),
        await_token: Default::default(),
    }
}

attrs_builder!(ExprAwait);

pub fn expr_binary(left: impl IntoExpr, op: impl IntoBinOp, right: impl IntoExpr) -> ExprBinary {
    ExprBinary {
        attrs: Default::default(),
        left: left.into_expr().into(),
        op: op.into_bin_op(),
        right: right.into_expr().into(),
    }
}

attrs_builder!(ExprBinary);

pub fn expr_block(block: impl Into<Block>) -> ExprBlock {
    ExprBlock {
        attrs: Default::default(),
        label: None,
        block: block.into(),
    }
}

attrs_builder!(ExprBlock);
label_builder!(ExprBlock);

pub fn expr_break() -> ExprBreak {
    ExprBreak {
        attrs: Default::default(),
        break_token: Default::default(),
        label: None,
        expr: None,
    }
}

attrs_builder!(ExprBreak);

pub trait ExprBreakBuilder {
    fn label(self, label: impl Into<Lifetime>) -> Self;
    fn expr(self, expr: impl IntoExpr) -> Self;
}

impl ExprBreakBuilder for ExprBreak {
    fn label(self, label: impl Into<Lifetime>) -> Self {
        Self {
            label: Some(label.into()),
            ..self
        }
    }

    fn expr(self, expr: impl IntoExpr) -> Self {
        Self {
            expr: Some(expr.into_expr().into()),
            ..self
        }
    }
}

pub fn expr_call<I: IntoExpr>(func: impl IntoExpr, args: impl IntoIterator<Item = I>) -> ExprCall {
    ExprCall {
        attrs: Default::default(),
        func: func.into_expr().into(),
        paren_token: Default::default(),
        args: FromIterator::from_iter(args.into_iter().map(IntoExpr::into_expr)),
    }
}

attrs_builder!(ExprCall);

pub fn expr_cast(expr: impl IntoExpr, ty: impl IntoType) -> ExprCast {
    ExprCast {
        attrs: Default::default(),
        expr: expr.into_expr().into(),
        as_token: Default::default(),
        ty: ty.into_type().into(),
    }
}

attrs_builder!(ExprCast);

pub fn expr_closure<I: IntoPat>(
    inputs: impl IntoIterator<Item = I>,
    body: impl IntoExpr,
) -> ExprClosure {
    ExprClosure {
        attrs: Default::default(),
        lifetimes: None,
        constness: None,
        movability: None,
        asyncness: None,
        capture: None,
        or1_token: Default::default(),
        inputs: FromIterator::from_iter(inputs.into_iter().map(IntoPat::into_pat)),
        or2_token: Default::default(),
        output: ReturnType::Default,
        body: body.into_expr().into(),
    }
}

attrs_builder!(ExprClosure);
output_builder!(ExprClosure);

pub trait ExprClosureBuilder {
    fn lifetimes(self, lifetimes: bool) -> Self;
    fn constness(self, constness: bool) -> Self;
    fn movability(self, movability: bool) -> Self;
    fn asyncness(self, asyncness: bool) -> Self;
    fn capture(self, capture: bool) -> Self;
}

impl ExprClosureBuilder for ExprClosure {
    fn lifetimes(self, lifetimes: bool) -> Self {
        Self {
            lifetimes: lifetimes.then(Default::default),
            ..self
        }
    }

    fn constness(self, constness: bool) -> Self {
        Self {
            constness: constness.then(Default::default),
            ..self
        }
    }

    fn movability(self, movability: bool) -> Self {
        Self {
            movability: movability.then(Default::default),
            ..self
        }
    }

    fn asyncness(self, asyncness: bool) -> Self {
        Self {
            asyncness: asyncness.then(Default::default),
            ..self
        }
    }

    fn capture(self, capture: bool) -> Self {
        Self {
            capture: capture.then(Default::default),
            ..self
        }
    }
}

pub fn expr_const(block: impl Into<Block>) -> ExprConst {
    ExprConst {
        attrs: Default::default(),
        const_token: Default::default(),
        block: block.into(),
    }
}

attrs_builder!(ExprConst);

pub fn expr_continue() -> ExprContinue {
    ExprContinue {
        attrs: Default::default(),
        continue_token: Default::default(),
        label: None,
    }
}

attrs_builder!(ExprContinue);

pub trait ExprContinueBuilder {
    fn label(self, label: impl Into<Lifetime>) -> Self;
}

impl ExprContinueBuilder for ExprContinue {
    fn label(self, label: impl Into<Lifetime>) -> Self {
        Self {
            label: Some(label.into()),
            ..self
        }
    }
}

pub fn expr_field(base: impl IntoExpr, member: impl Into<Member>) -> ExprField {
    ExprField {
        attrs: Default::default(),
        base: base.into_expr().into(),
        dot_token: Default::default(),
        member: member.into(),
    }
}

attrs_builder!(ExprField);

pub fn expr_for_loop(
    pat: impl IntoPat,
    expr: impl IntoExpr,
    body: impl Into<Block>,
) -> ExprForLoop {
    ExprForLoop {
        attrs: Default::default(),
        label: None,
        for_token: Default::default(),
        pat: pat.into_pat().into(),
        in_token: Default::default(),
        expr: expr.into_expr().into(),
        body: body.into(),
    }
}

attrs_builder!(ExprForLoop);
label_builder!(ExprForLoop);

pub fn expr_group(expr: impl IntoExpr) -> ExprGroup {
    ExprGroup {
        attrs: Default::default(),
        group_token: Default::default(),
        expr: expr.into_expr().into(),
    }
}

attrs_builder!(ExprGroup);

pub fn expr_if(cond: impl IntoExpr, then_branch: impl Into<Block>) -> ExprIf {
    ExprIf {
        attrs: Default::default(),
        if_token: Default::default(),
        cond: cond.into_expr().into(),
        then_branch: then_branch.into(),
        else_branch: None,
    }
}

attrs_builder!(ExprIf);

pub trait ExprIfBuilder {
    fn else_branch(self, expr: impl IntoExpr) -> Self;
}

impl ExprIfBuilder for ExprIf {
    fn else_branch(self, expr: impl IntoExpr) -> Self {
        Self {
            else_branch: Some((Default::default(), expr.into_expr().into())),
            ..self
        }
    }
}

pub fn expr_index(expr: impl IntoExpr, index: impl IntoExpr) -> ExprIndex {
    ExprIndex {
        attrs: Default::default(),
        expr: expr.into_expr().into(),
        bracket_token: Default::default(),
        index: index.into_expr().into(),
    }
}

attrs_builder!(ExprIndex);

pub fn expr_infer() -> ExprInfer {
    ExprInfer {
        attrs: Default::default(),
        underscore_token: Default::default(),
    }
}

attrs_builder!(ExprInfer);

pub fn expr_let(pat: impl IntoPat, expr: impl IntoExpr) -> ExprLet {
    ExprLet {
        attrs: Default::default(),
        let_token: Default::default(),
        pat: pat.into_pat().into(),
        eq_token: Default::default(),
        expr: expr.into_expr().into(),
    }
}

attrs_builder!(ExprLet);

pub fn expr_lit(lit: impl IntoLit) -> ExprLit {
    ExprLit {
        attrs: Default::default(),
        lit: lit.into_lit(),
    }
}

attrs_builder!(ExprLit);

pub fn expr_loop(body: impl Into<Block>) -> ExprLoop {
    ExprLoop {
        attrs: Default::default(),
        label: None,
        loop_token: Default::default(),
        body: body.into(),
    }
}

attrs_builder!(ExprLoop);
label_builder!(ExprLoop);

pub fn expr_macro(mac: impl Into<Macro>) -> ExprMacro {
    ExprMacro {
        attrs: Default::default(),
        mac: mac.into(),
    }
}

attrs_builder!(ExprMacro);

pub fn expr_match(expr: impl IntoExpr, arms: impl IntoIterator<Item = Arm>) -> ExprMatch {
    ExprMatch {
        attrs: Default::default(),
        match_token: Default::default(),
        expr: expr.into_expr().into(),
        brace_token: Default::default(),
        arms: arms.into_iter().collect(),
    }
}

attrs_builder!(ExprMatch);

pub fn expr_method_call<A: IntoExpr>(
    receiver: impl IntoExpr,
    method: impl IntoIdent,
    args: impl IntoIterator<Item = A>,
) -> ExprMethodCall {
    ExprMethodCall {
        attrs: Default::default(),
        receiver: receiver.into_expr().into(),
        dot_token: Default::default(),
        method: method.into_ident(),
        turbofish: None,
        paren_token: Default::default(),
        args: FromIterator::from_iter(args.into_iter().map(IntoExpr::into_expr)),
    }
}

attrs_builder!(ExprMethodCall);

pub trait ExprMethodCallBuilder {
    fn turbofish(self, args: impl Into<AngleBracketedGenericArguments>) -> Self;
}

impl ExprMethodCallBuilder for ExprMethodCall {
    fn turbofish(self, args: impl Into<AngleBracketedGenericArguments>) -> Self {
        Self {
            turbofish: Some(args.into()),
            ..self
        }
    }
}

pub fn expr_paren(expr: impl IntoExpr) -> ExprParen {
    ExprParen {
        attrs: Default::default(),
        paren_token: Default::default(),
        expr: expr.into_expr().into(),
    }
}

attrs_builder!(ExprParen);

pub fn expr_path(path: impl IntoPath) -> ExprPath {
    ExprPath {
        attrs: Default::default(),
        qself: None,
        path: path.into_path(),
    }
}

attrs_builder!(ExprPath);
qself_builder!(ExprPath);

pub fn expr_range(limits: impl IntoRangeLimits) -> ExprRange {
    ExprRange {
        attrs: Default::default(),
        start: None,
        limits: limits.into_range_limits(),
        end: None,
    }
}

attrs_builder!(ExprRange);

pub trait ExprRangeBuilder {
    fn start(self, expr: impl IntoExpr) -> Self;
    fn end(self, expr: impl IntoExpr) -> Self;
}

impl ExprRangeBuilder for ExprRange {
    fn start(self, expr: impl IntoExpr) -> Self {
        Self {
            end: Some(expr.into_expr().into()),
            ..self
        }
    }

    fn end(self, expr: impl IntoExpr) -> Self {
        Self {
            end: Some(expr.into_expr().into()),
            ..self
        }
    }
}

pub fn expr_reference(expr: impl IntoExpr) -> ExprReference {
    ExprReference {
        attrs: Default::default(),
        and_token: Default::default(),
        mutability: None,
        expr: expr.into_expr().into(),
    }
}

attrs_builder!(ExprReference);
mutability_builder!(ExprReference);

pub fn expr_repeat(expr: impl IntoExpr, len: impl IntoExpr) -> ExprRepeat {
    ExprRepeat {
        attrs: Default::default(),
        bracket_token: Default::default(),
        expr: expr.into_expr().into(),
        semi_token: Default::default(),
        len: len.into_expr().into(),
    }
}

attrs_builder!(ExprRepeat);

pub fn expr_return() -> ExprReturn {
    ExprReturn {
        attrs: Default::default(),
        return_token: Default::default(),
        expr: None,
    }
}

attrs_builder!(ExprReturn);

pub trait ExprReturnBuilder {
    fn expr(self, expr: impl IntoExpr) -> Self;
}

impl ExprReturnBuilder for ExprReturn {
    fn expr(self, expr: impl IntoExpr) -> Self {
        Self {
            expr: Some(expr.into_expr().into()),
            ..self
        }
    }
}

pub fn expr_struct<F: Into<FieldValue>>(
    path: impl IntoPath,
    fields: impl IntoIterator<Item = F>,
) -> ExprStruct {
    ExprStruct {
        attrs: Default::default(),
        qself: None,
        path: path.into_path(),
        brace_token: Default::default(),
        fields: FromIterator::from_iter(fields.into_iter().map(Into::into)),
        dot2_token: None,
        rest: None,
    }
}

attrs_builder!(ExprStruct);
qself_builder!(ExprStruct);

pub trait ExprStructBuilder {
    fn dot2_token(self, dot2_token: bool) -> Self;
    fn rest(self, rest: impl IntoExpr) -> Self;
}

impl ExprStructBuilder for ExprStruct {
    fn dot2_token(self, dot2_token: bool) -> Self {
        Self {
            dot2_token: dot2_token.then(Default::default),
            ..self
        }
    }

    fn rest(self, rest: impl IntoExpr) -> Self {
        Self {
            dot2_token: Some(Default::default()),
            rest: Some(rest.into_expr().into()),
            ..self
        }
    }
}

pub fn expr_try(expr: impl IntoExpr) -> ExprTry {
    ExprTry {
        attrs: Default::default(),
        expr: expr.into_expr().into(),
        question_token: Default::default(),
    }
}

attrs_builder!(ExprTry);

pub fn expr_try_block(block: impl Into<Block>) -> ExprTryBlock {
    ExprTryBlock {
        attrs: vec![],
        try_token: Default::default(),
        block: block.into(),
    }
}

attrs_builder!(ExprTryBlock);

pub fn expr_tuple<E: IntoExpr>(elems: impl IntoIterator<Item = E>) -> ExprTuple {
    ExprTuple {
        attrs: Default::default(),
        paren_token: Default::default(),
        elems: FromIterator::from_iter(elems.into_iter().map(IntoExpr::into_expr)),
    }
}

attrs_builder!(ExprTuple);

pub fn expr_unary(op: impl IntoUnOp, expr: impl IntoExpr) -> ExprUnary {
    ExprUnary {
        attrs: Default::default(),
        op: op.into_un_op(),
        expr: expr.into_expr().into(),
    }
}

attrs_builder!(ExprUnary);

pub fn expr_unsafe(block: impl Into<Block>) -> ExprUnsafe {
    ExprUnsafe {
        attrs: Default::default(),
        unsafe_token: Default::default(),
        block: block.into(),
    }
}

attrs_builder!(ExprUnsafe);

pub fn expr_while(cond: impl IntoExpr, body: impl Into<Block>) -> ExprWhile {
    ExprWhile {
        attrs: Default::default(),
        label: None,
        while_token: Default::default(),
        cond: cond.into_expr().into(),
        body: body.into(),
    }
}

attrs_builder!(ExprWhile);
label_builder!(ExprWhile);

pub fn expr_yield() -> ExprYield {
    ExprYield {
        attrs: Default::default(),
        yield_token: Default::default(),
        expr: None,
    }
}

attrs_builder!(ExprYield);

pub trait ExprYieldBuilder {
    fn expr(self, expr: impl IntoExpr) -> Self;
}

impl ExprYieldBuilder for ExprYield {
    fn expr(self, expr: impl IntoExpr) -> Self {
        Self {
            expr: Some(expr.into_expr().into()),
            ..self
        }
    }
}

pub fn index(index: impl Into<usize>) -> Index {
    Index::from(index.into())
}

pub fn field_value(member: impl Into<Member>, expr: impl IntoExpr) -> FieldValue {
    FieldValue {
        attrs: Default::default(),
        member: member.into(),
        colon_token: Some(Default::default()),
        expr: expr.into_expr(),
    }
}

attrs_builder!(FieldValue);

pub fn label(name: impl Into<Lifetime>) -> Label {
    Label {
        name: name.into(),
        colon_token: Default::default(),
    }
}

pub fn arm(pat: impl IntoPat, body: impl IntoExpr) -> Arm {
    Arm {
        attrs: Default::default(),
        pat: pat.into_pat(),
        guard: None,
        fat_arrow_token: Default::default(),
        body: body.into_expr().into(),
        comma: Some(Default::default()),
    }
}

attrs_builder!(Arm);

pub trait ArmBuilder {
    fn guard(self, expr: impl IntoExpr) -> Self;
}

impl ArmBuilder for Arm {
    fn guard(self, expr: impl IntoExpr) -> Self {
        Self {
            guard: Some((Default::default(), expr.into_expr().into())),
            ..self
        }
    }
}

pub fn range_limits_half_open_variant() -> RangeLimits {
    RangeLimits::HalfOpen(Default::default())
}

pub fn range_limits_closed_variant() -> RangeLimits {
    RangeLimits::Closed(Default::default())
}

pub trait IntoRangeLimits {
    fn into_range_limits(self) -> RangeLimits;
}

impl IntoRangeLimits for RangeLimits {
    fn into_range_limits(self) -> RangeLimits {
        self
    }
}

impl IntoRangeLimits for token::DotDot {
    fn into_range_limits(self) -> RangeLimits {
        RangeLimits::HalfOpen(self)
    }
}

impl IntoRangeLimits for token::DotDotEq {
    fn into_range_limits(self) -> RangeLimits {
        RangeLimits::Closed(self)
    }
}
