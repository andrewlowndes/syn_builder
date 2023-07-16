use syn::{BinOp, UnOp};

pub fn bin_op_add_variant() -> BinOp {
    BinOp::Add(Default::default())
}

pub fn bin_op_sub_variant() -> BinOp {
    BinOp::Sub(Default::default())
}

pub fn bin_op_mul_variant() -> BinOp {
    BinOp::Mul(Default::default())
}

pub fn bin_op_div_variant() -> BinOp {
    BinOp::Div(Default::default())
}

pub fn bin_op_rem_variant() -> BinOp {
    BinOp::Rem(Default::default())
}

pub fn bin_op_and_variant() -> BinOp {
    BinOp::And(Default::default())
}

pub fn bin_op_or_variant() -> BinOp {
    BinOp::Or(Default::default())
}

pub fn bin_op_bit_xor_variant() -> BinOp {
    BinOp::BitXor(Default::default())
}

pub fn bin_op_bit_and_variant() -> BinOp {
    BinOp::BitAnd(Default::default())
}

pub fn bin_op_bit_or_variant() -> BinOp {
    BinOp::BitOr(Default::default())
}

pub fn bin_op_shl_variant() -> BinOp {
    BinOp::Shl(Default::default())
}

pub fn bin_op_shr_variant() -> BinOp {
    BinOp::Shr(Default::default())
}

pub fn bin_op_eq_variant() -> BinOp {
    BinOp::Eq(Default::default())
}

pub fn bin_op_lt_variant() -> BinOp {
    BinOp::Lt(Default::default())
}

pub fn bin_op_le_variant() -> BinOp {
    BinOp::Le(Default::default())
}

pub fn bin_op_ne_variant() -> BinOp {
    BinOp::Ne(Default::default())
}

pub fn bin_op_ge_variant() -> BinOp {
    BinOp::Ge(Default::default())
}

pub fn bin_op_gt_variant() -> BinOp {
    BinOp::Gt(Default::default())
}

pub fn bin_op_add_assign_variant() -> BinOp {
    BinOp::AddAssign(Default::default())
}

pub fn bin_op_sub_assign_variant() -> BinOp {
    BinOp::SubAssign(Default::default())
}

pub fn bin_op_mul_assign_variant() -> BinOp {
    BinOp::MulAssign(Default::default())
}

pub fn bin_op_div_assign_variant() -> BinOp {
    BinOp::DivAssign(Default::default())
}

pub fn bin_op_rem_assign_variant() -> BinOp {
    BinOp::RemAssign(Default::default())
}

pub fn bin_op_bit_xor_assign_variant() -> BinOp {
    BinOp::BitXorAssign(Default::default())
}

pub fn bin_op_bit_and_assign_variant() -> BinOp {
    BinOp::BitAndAssign(Default::default())
}

pub fn bin_op_bit_or_assign_variant() -> BinOp {
    BinOp::BitOrAssign(Default::default())
}

pub fn bin_op_shl_assign_variant() -> BinOp {
    BinOp::ShlAssign(Default::default())
}

pub fn bin_op_shr_assign_variant() -> BinOp {
    BinOp::ShrAssign(Default::default())
}

pub trait IntoBinOp {
    fn into_bin_op(self) -> BinOp;
}

impl IntoBinOp for BinOp {
    fn into_bin_op(self) -> BinOp {
        self
    }
}

macro_rules! impl_bin_op {
    ($($variant:ident => $token:path,)+) => {
        $(
            impl $crate::op::IntoBinOp for $token {
                fn into_bin_op(self) -> syn::BinOp {
                    syn::BinOp::$variant(self)
                }
            }
        )*
    };
}

impl_bin_op!(
    Add => syn::token::Plus,
    Sub => syn::token::Minus,
    Mul => syn::token::Star,
    Div => syn::token::Slash,
    Rem => syn::token::Percent,
    And => syn::token::AndAnd,
    Or => syn::token::OrOr,
    BitXor => syn::token::Caret,
    BitAnd => syn::token::And,
    BitOr => syn::token::Or,
    Shl => syn::token::Shl,
    Shr => syn::token::Shr,
    Eq => syn::token::EqEq,
    Lt => syn::token::Lt,
    Le => syn::token::Le,
    Ne => syn::token::Ne,
    Ge => syn::token::Ge,
    Gt => syn::token::Gt,
    AddAssign => syn::token::PlusEq,
    SubAssign => syn::token::MinusEq,
    MulAssign => syn::token::StarEq,
    DivAssign => syn::token::SlashEq,
    RemAssign => syn::token::PercentEq,
    BitXorAssign => syn::token::CaretEq,
    BitAndAssign => syn::token::AndEq,
    BitOrAssign => syn::token::OrEq,
    ShlAssign => syn::token::ShlEq,
    ShrAssign => syn::token::ShrEq,
);

pub fn un_op_deref() -> UnOp {
    UnOp::Deref(Default::default())
}

pub fn un_op_not() -> UnOp {
    UnOp::Not(Default::default())
}

pub fn un_op_neg() -> UnOp {
    UnOp::Neg(Default::default())
}

pub trait IntoUnOp {
    fn into_un_op(self) -> UnOp;
}

impl IntoUnOp for UnOp {
    fn into_un_op(self) -> UnOp {
        self
    }
}

macro_rules! impl_un_op {
    ($($variant:ident => $token:path,)+) => {
        $(
            impl $crate::op::IntoUnOp for $token {
                fn into_un_op(self) -> syn::UnOp {
                    syn::UnOp::$variant(self)
                }
            }
        )*
    };
}

impl_un_op!(
    Deref => syn::token::Star,
    Not => syn::token::Not,
    Neg => syn::token::Minus,
);
