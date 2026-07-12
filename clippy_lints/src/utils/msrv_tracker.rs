use clippy_utils::msrvs::{MsrvStack, SharedMsrvStack};
use rustc_ast::Attribute;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::impl_lint_pass;

/// The single early pass responsible for parsing `#[clippy::msrv]` attributes.
///
/// It owns the authoritative [`MsrvStack`] and, after every push/pop, publishes the current version
/// into the shared [`SharedMsrvStack`] that every other MSRV-aware early pass reads. This replaces
/// the old scheme where each of those passes kept its own stack and re-parsed the same attribute
/// slice, so a node visited by `N` passes was scanned `2N` times; now it is scanned once on entry
/// and once on exit regardless of how many passes consult the MSRV.
pub struct MsrvTracker {
    stack: MsrvStack,
    shared: SharedMsrvStack,
}

impl MsrvTracker {
    pub fn new(shared: SharedMsrvStack, stack: MsrvStack) -> Self {
        Self { stack, shared }
    }
}

impl_lint_pass!(MsrvTracker => []);

impl EarlyLintPass for MsrvTracker {
    fn check_attributes(&mut self, _cx: &EarlyContext<'_>, attrs: &[Attribute]) {
        // The overwhelming majority of nodes carry no attributes and can't change the MSRV. Decide
        // that here — this method is inlined into the combined pass, so the empty-slice case costs
        // only a length check, and the cross-crate parse and the republish happen just for the rare
        // node that actually has attributes.
        if !attrs.is_empty() {
            self.stack.check_attributes(attrs);
            self.shared.sync_to(&self.stack);
        }
    }

    fn check_attributes_post(&mut self, _cx: &EarlyContext<'_>, attrs: &[Attribute]) {
        if !attrs.is_empty() {
            self.stack.check_attributes_post(attrs);
            self.shared.sync_to(&self.stack);
        }
    }
}
