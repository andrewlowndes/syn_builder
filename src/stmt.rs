use crate::{attrs_builder, IntoExpr, IntoPat};
use syn::{Block, Expr, Item, Local, LocalInit, Macro, Stmt, StmtMacro};

pub fn block<S: IntoStmt>(stmts: impl IntoIterator<Item = S>) -> Block {
    Block {
        brace_token: Default::default(),
        stmts: stmts.into_iter().map(IntoStmt::into_stmt).collect(),
    }
}

pub trait IntoStmt {
    fn into_stmt(self) -> Stmt;
}

impl IntoStmt for Stmt {
    fn into_stmt(self) -> Stmt {
        self
    }
}

impl IntoStmt for Local {
    fn into_stmt(self) -> Stmt {
        Stmt::Local(self)
    }
}

impl IntoStmt for Item {
    fn into_stmt(self) -> Stmt {
        Stmt::Item(self)
    }
}

impl IntoStmt for Expr {
    fn into_stmt(self) -> Stmt {
        Stmt::Expr(self, None)
    }
}

impl IntoStmt for StmtMacro {
    fn into_stmt(self) -> Stmt {
        Stmt::Macro(self)
    }
}

pub fn local(pat: impl IntoPat) -> Local {
    Local {
        attrs: Default::default(),
        let_token: Default::default(),
        pat: pat.into_pat(),
        init: None,
        semi_token: Default::default(),
    }
}

attrs_builder!(Local);

pub trait LocalBuilder {
    fn init(self, init: LocalInit) -> Self;
}

impl LocalBuilder for Local {
    fn init(self, init: LocalInit) -> Self {
        Self {
            init: Some(init),
            ..self
        }
    }
}

pub fn local_init(expr: impl IntoExpr) -> LocalInit {
    LocalInit {
        eq_token: Default::default(),
        expr: expr.into_expr().into(),
        diverge: None,
    }
}

pub trait LocalInitBuilder {
    fn diverge(self, expr: impl IntoExpr) -> Self;
}

impl LocalInitBuilder for LocalInit {
    fn diverge(self, expr: impl IntoExpr) -> Self {
        Self {
            diverge: Some((Default::default(), expr.into_expr().into())),
            ..self
        }
    }
}

pub fn stmt_macro(mac: impl Into<Macro>) -> StmtMacro {
    StmtMacro {
        attrs: Default::default(),
        mac: mac.into(),
        semi_token: None,
    }
}

attrs_builder!(StmtMacro);
