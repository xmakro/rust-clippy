//! A statically-combined early lint pass.
//!
//! The early-pass analogue of [`combined_late_pass`]. Folds clippy's early
//! passes into one concrete struct, one field per pass, with a single
//! `EarlyLintPass` impl that forwards each `check_*` to every field. Same
//! static-dispatch / DCE win as the late version: because the field types are
//! concrete and the forwards are `#[inline(always)]`, a pass that doesn't
//! override a `check_*` contributes only the empty default body, which is
//! DCE'd away. So the per-node, per-pass indirect (vtable) call into an empty
//! method disappears entirely, and the passes that do override become direct,
//! inlined calls. No vtable, no per-node dynamic dispatch.
//!
//! Like the late combine, each field carries an `active` flag so that passes
//! whose lints provably cannot emit anywhere in the crate are skipped. rustc
//! computes this for late passes (`lints_that_dont_need_to_run`) but has no
//! equivalent for early passes (there is no query infrastructure at that
//! stage), so [`EarlyPassFilter`] mirrors that query's predicate on the AST:
//! a pass still needs to run if any of its lints is `eval_always`, has
//! future-breakage reporting, is above `Allow` at the crate root (defaults,
//! CLI flags and crate attributes included), or is raised above `Allow` by a
//! lint attribute somewhere in the crate. Attribute mentions are resolved
//! through [`LintStore::find_lints`], the same maps rustc's lint level
//! machinery resolves the attribute with when it applies (groups expanded,
//! renames and deprecated aliases followed). The flags are computed once, in
//! `check_crate`, the first callback that sees the whole expanded crate.
//!
//! [`combined_late_pass`]: crate::combined_late_pass

use rustc_ast::visit::{self as ast_visit, Visitor};
use rustc_ast::{self as ast, Attribute};
use rustc_data_structures::fx::FxHashSet;
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_lint::{EarlyContext, Lint, LintContext, LintId, LintStore, unerased_lint_store};
use rustc_span::sym;

/// A pass paired with its "still needs to run" flag.
///
/// The flag starts out `true` (only crate-root `check_attributes` runs before
/// it is computed) and is set from [`EarlyPassFilter`] in `check_crate`.
pub struct Gated<P> {
    pub(crate) active: bool,
    pub(crate) pass: P,
}

impl<P> Gated<P> {
    pub fn new(pass: P) -> Self {
        Gated { active: true, pass }
    }
}

/// Decides which passes still need to run, mirroring the predicate of
/// rustc's `lints_that_dont_need_to_run` used to gate the late passes.
pub(crate) struct EarlyPassFilter<'a, 'ecx> {
    cx: &'a EarlyContext<'ecx>,
    krate: &'a ast::Crate,
    /// Lints raised above `Allow` by a lint attribute somewhere in the crate.
    /// Computed lazily: crates where every lint is already decided by the
    /// crate root level never pay for the scan.
    mentioned: Option<FxHashSet<LintId>>,
}

impl<'a, 'ecx> EarlyPassFilter<'a, 'ecx> {
    pub(crate) fn new(cx: &'a EarlyContext<'ecx>, krate: &'a ast::Crate) -> Self {
        EarlyPassFilter {
            cx,
            krate,
            mentioned: None,
        }
    }

    /// Whether any of a pass's lints may still emit somewhere in the crate.
    /// Passes declaring no lints are collectors which must always run.
    pub(crate) fn pass_is_active(&mut self, lints: &[&'static Lint]) -> bool {
        lints.is_empty() || lints.iter().any(|lint| self.lint_may_emit(lint))
    }

    fn lint_may_emit(&mut self, lint: &'static Lint) -> bool {
        lint.eval_always
            || lint.future_incompatible.is_some_and(|fut| fut.report_in_deps)
            || !self.cx.get_lint_level_spec(lint).is_allow()
            || self.mentioned().contains(&LintId::of(lint))
    }

    fn mentioned(&mut self) -> &FxHashSet<LintId> {
        self.mentioned
            .get_or_insert_with(|| mentioned_lints(unerased_lint_store(self.cx.sess()), self.krate))
    }
}

/// Collects every lint that a lint level attribute anywhere in the crate
/// mentions with a level other than `allow`. Such a mention means the lint
/// may emit somewhere even though it is allowed at the crate root.
fn mentioned_lints(store: &LintStore, krate: &ast::Crate) -> FxHashSet<LintId> {
    struct Scanner<'a> {
        store: &'a LintStore,
        mentioned: FxHashSet<LintId>,
    }

    impl Scanner<'_> {
        fn scan_attr(&mut self, attr: &Attribute) {
            // `allow` can never make a lint emit, every other level attribute can.
            if !(attr.has_name(sym::warn)
                || attr.has_name(sym::deny)
                || attr.has_name(sym::forbid)
                || attr.has_name(sym::expect))
            {
                return;
            }
            for item in attr.meta_item_list().iter().flatten() {
                if let Some(meta_item) = item.meta_item() {
                    // The `by_name`/`lint_groups` keys are `lint_name` and
                    // `tool::lint_name`; anything else (`reason = ".."`,
                    // unknown or removed lints) resolves to nothing.
                    let ids = match &*meta_item.path.segments {
                        [name] => self.store.find_lints(name.ident.name.as_str()),
                        [tool, name] => self
                            .store
                            .find_lints(&format!("{}::{}", tool.ident.name, name.ident.name)),
                        _ => None,
                    };
                    self.mentioned.extend(ids.into_iter().flatten().copied());
                }
            }
        }
    }

    impl<'ast> Visitor<'ast> for Scanner<'_> {
        fn visit_attribute(&mut self, attr: &'ast Attribute) {
            self.scan_attr(attr);
        }

        fn visit_expr(&mut self, e: &'ast ast::Expr) {
            ensure_sufficient_stack(|| ast_visit::walk_expr(self, e));
        }
    }

    let mut scanner = Scanner {
        store,
        mentioned: FxHashSet::default(),
    };
    ast_visit::walk_crate(&mut scanner, krate);
    scanner.mentioned
}

/// Run one field's `check_*`, if that field is active.
///
/// Fully qualified through [`rustc_lint::EarlyLintPass`] since some passes impl
/// both `EarlyLintPass` and `LateLintPass` with like-named methods, which would
/// be ambiguous on the concrete field type.
#[macro_export]
macro_rules! run_combined_early_lint_pass_field {
    ($self:ident, $field:ident, $name:ident, ($($arg:expr),* $(,)?)) => {
        if $self.$field.active {
            rustc_lint::EarlyLintPass::$name(&mut $self.$field.pass, $($arg),*);
        }
    };
}

/// Forward one `check_*` method to every field of the combined pass.
#[macro_export]
macro_rules! expand_combined_early_lint_pass_method {
    ([$($field:ident),*], $self:ident, $name:ident, $args:tt) => ({
        $($crate::run_combined_early_lint_pass_field!($self, $field, $name, $args);)*
    })
}

/// Hook expanded at the top of every combined `check_*` method body: a
/// compile-time match on the method name. Only the `check_crate` arm expands
/// to code — `check_crate` is the first callback to see the whole expanded
/// crate, so the per-pass `active` flags are computed there, once, before any
/// field runs. For every other method the fallback arm expands to nothing, so
/// the per-node callbacks carry no runtime cost from this hook.
///
/// Should `check_crate`'s parameter list in `early_lint_methods!` ever change
/// shape, the first arm stops matching and `compute_active` goes uncalled,
/// which the `dead_code` lint turns into a build failure.
#[macro_export]
macro_rules! combined_early_lint_pass_gate {
    (check_crate, $self:ident, $cx:ident, ($krate:ident)) => {
        $self.compute_active($cx, $krate);
    };
    ($name:ident, $self:ident, $cx:ident, $params:tt) => {};
}

/// Generate the combined `EarlyLintPass` impl's `check_*` methods, one per method
/// in rustc's early-pass method list.
#[macro_export]
macro_rules! expand_combined_early_lint_pass_methods {
    ($fields:tt, [$($(#[$attr:meta])* fn $name:ident($($param:ident: $arg:ty),*);)*]) => (
        $(#[inline(always)] fn $name(&mut self, cx: &rustc_lint::EarlyContext<'_>, $($param: $arg),*) {
            // Expands to `self.compute_active(..)` in `check_crate` and to
            // nothing in every other method (see the gate macro's docs).
            $crate::combined_early_lint_pass_gate!($name, self, cx, ($($param),*));
            $crate::expand_combined_early_lint_pass_method!($fields, self, $name, (cx, $($param),*));
        })*
    )
}

/// Declare the combined struct (one [`Gated`] field per pass) plus its
/// `LintPass`/`EarlyLintPass` impls. The method list comes from
/// `rustc_lint::early_lint_methods!` so it can't drift from rustc's.
///
/// Each entry is `Field: Type = constructor`; `new`'s params (`conf`, ...) come
/// from the caller so ctor exprs can name them without hygiene trouble.
#[macro_export]
macro_rules! combined_early_lint_pass {
    (
        [$name:ident, ($($pname:ident: $pty:ty),* $(,)?), [$($field:ident: $fty:ty = $ctor:expr,)*]],
        $methods:tt
    ) => {
        #[allow(non_snake_case)]
        pub struct $name {
            $($field: $crate::combined_early_pass::Gated<$fty>,)*
        }

        impl $name {
            pub fn new($($pname: $pty,)*) -> Self {
                Self {
                    $($field: $crate::combined_early_pass::Gated::new($ctor),)*
                }
            }

            /// Called once per crate, from `check_crate` (see
            /// `combined_early_lint_pass_gate!`).
            fn compute_active(&mut self, cx: &rustc_lint::EarlyContext<'_>, krate: &rustc_ast::Crate) {
                let mut filter = $crate::combined_early_pass::EarlyPassFilter::new(cx, krate);
                $(self.$field.active = filter.pass_is_active(&rustc_lint::LintPass::get_lints(&self.$field.pass));)*
            }
        }

        #[allow(rustc::lint_pass_impl_without_macro)]
        impl rustc_lint::LintPass for $name {
            fn name(&self) -> &'static str {
                stringify!($name)
            }
            fn get_lints(&self) -> rustc_lint::LintVec {
                // Reserve at least one slot per pass up front to skip the early reallocations.
                let mut lints = Vec::with_capacity([$(stringify!($field)),*].len());
                $(lints.extend(self.$field.pass.get_lints());)*
                lints
            }
        }

        impl rustc_lint::EarlyLintPass for $name {
            $crate::expand_combined_early_lint_pass_methods!([$($field),*], $methods);
        }
    };
}
