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
    crate::box_default::BOX_DEFAULT,
    crate::create_dir::CREATE_DIR,
    crate::default_instead_of_iter_empty::DEFAULT_INSTEAD_OF_ITER_EMPTY,
    crate::drop_forget_ref::DROP_NON_DROP,
    crate::drop_forget_ref::FORGET_NON_DROP,
    crate::drop_forget_ref::MEM_FORGET,
    crate::duration_suboptimal_units::DURATION_SUBOPTIMAL_UNITS,
    crate::exit::EXIT,
    crate::from_raw_with_void_ptr::FROM_RAW_WITH_VOID_PTR,
    crate::from_str_radix_10::FROM_STR_RADIX_10,
    crate::mem_replace::MEM_REPLACE_OPTION_WITH_NONE,
    crate::mem_replace::MEM_REPLACE_OPTION_WITH_SOME,
    crate::mem_replace::MEM_REPLACE_WITH_DEFAULT,
    crate::mem_replace::MEM_REPLACE_WITH_UNINIT,
    crate::non_octal_unix_permissions::NON_OCTAL_UNIX_PERMISSIONS,
    crate::same_length_and_capacity::SAME_LENGTH_AND_CAPACITY,
    crate::size_of_ref::SIZE_OF_REF,
    crate::strlen_on_c_strings::STRLEN_ON_C_STRINGS,
    crate::swap_ptr_to_ref::SWAP_PTR_TO_REF,
    crate::unnecessary_mut_passed::UNNECESSARY_MUT_PASSED,
    crate::volatile_composites::VOLATILE_COMPOSITES,
    crate::with_capacity_zero::WITH_CAPACITY_ZERO,
    crate::zombie_processes::ZOMBIE_PROCESSES,
]);

impl<'tcx> LateLintPass<'tcx> for Calls {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        match expr.kind {
            ExprKind::Call(func, args) => {
                // Callee resolution shared by the lints that match a path callee against a
                // `DefId`. Lints that inspect the callee syntactically or resolve it another
                // way (`basic_res`, `fn_def_id`) keep their own logic.
                let callee_id = if let ExprKind::Path(ref qpath) = func.kind {
                    cx.qpath_res(qpath, func.hir_id).opt_def_id()
                } else {
                    None
                };
                crate::mem_replace::check(cx, expr, args, callee_id, self.msrv);
                crate::drop_forget_ref::check(cx, expr, args, callee_id);
                crate::create_dir::check(cx, expr, func, args, callee_id);
                crate::exit::check(cx, expr, args, callee_id);
                crate::from_str_radix_10::check(cx, expr, func, args);
                crate::strlen_on_c_strings::check(cx, expr, args, callee_id, self.msrv);
                crate::swap_ptr_to_ref::check(cx, expr, func, args);
                crate::default_instead_of_iter_empty::check(cx, expr, func, args);
                crate::box_default::check(cx, expr, func, args);
                crate::from_raw_with_void_ptr::check(cx, expr, func, args);
                crate::size_of_ref::check(cx, expr, func, args);
                crate::same_length_and_capacity::check(cx, expr, func, args);
                crate::duration_suboptimal_units::check(cx, expr, func, args, self.msrv);
                crate::with_capacity_zero::check(cx, expr, func, args);
                crate::non_octal_unix_permissions::check_call(cx, expr, args, callee_id);
                crate::zombie_processes::check(cx, expr);
                crate::volatile_composites::check_call(cx, expr, args, callee_id);
                crate::unnecessary_mut_passed::check_call(cx, expr, func, args);
            },
            ExprKind::MethodCall(seg, recv, args, _) => {
                crate::non_octal_unix_permissions::check_method_call(cx, expr, seg, recv, args);
                crate::zombie_processes::check(cx, expr);
                crate::volatile_composites::check_method_call(cx, expr, seg, recv);
                crate::unnecessary_mut_passed::check_method_call(cx, expr, seg, recv, args);
            },
            _ => {},
        }
    }
}
