//! A host pass for lints that fire on call expressions.
//!
//! Groups late lints whose `check_expr` reacts only to `ExprKind::Call` or
//! `ExprKind::MethodCall` behind one shared kind test, following the same
//! submodule convention as `methods` and `operators`. Each lint keeps its own
//! module and guards; this pass only routes the matching expressions to them.

use clippy_config::Conf;
use clippy_utils::msrvs::Msrv;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;

pub struct Calls {
    msrv: Msrv,
}

impl Calls {
    pub fn new(conf: &'static Conf) -> Self {
        Self { msrv: conf.msrv.into() }
    }
}

impl_lint_pass!(Calls => [
    crate::mem_replace::MEM_REPLACE_OPTION_WITH_NONE,
    crate::mem_replace::MEM_REPLACE_OPTION_WITH_SOME,
    crate::mem_replace::MEM_REPLACE_WITH_DEFAULT,
    crate::mem_replace::MEM_REPLACE_WITH_UNINIT,
    crate::drop_forget_ref::DROP_NON_DROP,
    crate::drop_forget_ref::FORGET_NON_DROP,
    crate::drop_forget_ref::MEM_FORGET,
    crate::create_dir::CREATE_DIR,
    crate::exit::EXIT,
    crate::from_str_radix_10::FROM_STR_RADIX_10,
    crate::strlen_on_c_strings::STRLEN_ON_C_STRINGS,
    crate::swap_ptr_to_ref::SWAP_PTR_TO_REF,
    crate::default_instead_of_iter_empty::DEFAULT_INSTEAD_OF_ITER_EMPTY,
    crate::box_default::BOX_DEFAULT,
    crate::from_raw_with_void_ptr::FROM_RAW_WITH_VOID_PTR,
    crate::size_of_ref::SIZE_OF_REF,
    crate::same_length_and_capacity::SAME_LENGTH_AND_CAPACITY,
    crate::duration_suboptimal_units::DURATION_SUBOPTIMAL_UNITS,
    crate::with_capacity_zero::WITH_CAPACITY_ZERO,
    crate::non_octal_unix_permissions::NON_OCTAL_UNIX_PERMISSIONS,
    crate::zombie_processes::ZOMBIE_PROCESSES,
    crate::volatile_composites::VOLATILE_COMPOSITES,
    crate::unnecessary_mut_passed::UNNECESSARY_MUT_PASSED,
]);

impl<'tcx> LateLintPass<'tcx> for Calls {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        match expr.kind {
            ExprKind::Call(..) => {
                crate::mem_replace::check(cx, expr, self.msrv);
                crate::drop_forget_ref::check(cx, expr);
                crate::create_dir::check(cx, expr);
                crate::exit::check(cx, expr);
                crate::from_str_radix_10::check(cx, expr);
                crate::strlen_on_c_strings::check(cx, expr, self.msrv);
                crate::swap_ptr_to_ref::check(cx, expr);
                crate::default_instead_of_iter_empty::check(cx, expr);
                crate::box_default::check(cx, expr);
                crate::from_raw_with_void_ptr::check(cx, expr);
                crate::size_of_ref::check(cx, expr);
                crate::same_length_and_capacity::check(cx, expr);
                crate::duration_suboptimal_units::check(cx, expr, self.msrv);
                crate::with_capacity_zero::check(cx, expr);
                crate::non_octal_unix_permissions::check(cx, expr);
                crate::zombie_processes::check(cx, expr);
                crate::volatile_composites::check(cx, expr);
                crate::unnecessary_mut_passed::check(cx, expr);
            },
            ExprKind::MethodCall(..) => {
                crate::non_octal_unix_permissions::check(cx, expr);
                crate::zombie_processes::check(cx, expr);
                crate::volatile_composites::check(cx, expr);
                crate::unnecessary_mut_passed::check(cx, expr);
            },
            _ => {},
        }
    }
}
