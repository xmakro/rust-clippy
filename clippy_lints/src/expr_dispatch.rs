//! Generated ExprKind dispatch for `CombinedLateLintPass::check_expr`.
//!
//! Regenerate with `util/gen_expr_dispatch.py` after adding or changing a late pass;
//! passes whose `check_expr` is not provably gated on `expr.kind` are called for every
//! expression, so a stale table can only miss a speedup for new passes, never skip one,
//! as long as new passes are added to the always-run list or the table is regenerated.

/// Run one pass's `check_expr`, if active.
#[macro_export]
macro_rules! run_check_expr_field {
    ($self:ident, $field:ident, $cx:expr, $e:expr) => {
        if $self.$field.active {
            rustc_lint::LateLintPass::check_expr(&mut $self.$field.pass, $cx, $e);
        }
    };
}

/// The generated `check_expr` body: always-run passes first, then the passes whose
/// `check_expr` provably reacts only to specific `ExprKind`s, grouped per kind.
#[macro_export]
macro_rules! combined_check_expr_dispatch {
    ($self:ident, $cx:expr, $e:expr) => {{
        $crate::run_check_expr_field!($self, ArithmeticSideEffects, $cx, $e);
        $crate::run_check_expr_field!($self, DumpHir, $cx, $e);
        $crate::run_check_expr_field!($self, Author, $cx, $e);
        $crate::run_check_expr_field!($self, NonminimalBool, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessBool, $cx, $e);
        $crate::run_check_expr_field!($self, BoolComparison, $cx, $e);
        $crate::run_check_expr_field!($self, LintPass, $cx, $e);
        $crate::run_check_expr_field!($self, EtaReduction, $cx, $e);
        $crate::run_check_expr_field!($self, UnnecessaryMutPassed, $cx, $e);
        $crate::run_check_expr_field!($self, LenZero, $cx, $e);
        $crate::run_check_expr_field!($self, BlocksInConditions, $cx, $e);
        $crate::run_check_expr_field!($self, StringAdd, $cx, $e);
        $crate::run_check_expr_field!($self, ImplicitSaturatingSub, $cx, $e);
        $crate::run_check_expr_field!($self, NonOctalUnixPermissions, $cx, $e);
        $crate::run_check_expr_field!($self, Matches, $cx, $e);
        $crate::run_check_expr_field!($self, ManualStrip, $cx, $e);
        $crate::run_check_expr_field!($self, Ranges, $cx, $e);
        $crate::run_check_expr_field!($self, UseSelf, $cx, $e);
        $crate::run_check_expr_field!($self, Casts, $cx, $e);
        $crate::run_check_expr_field!($self, SizeOfInElementCount, $cx, $e);
        $crate::run_check_expr_field!($self, IndexRefutableSlice, $cx, $e);
        $crate::run_check_expr_field!($self, Methods, $cx, $e);
        $crate::run_check_expr_field!($self, UnitTypes, $cx, $e);
        $crate::run_check_expr_field!($self, Loops, $cx, $e);
        $crate::run_check_expr_field!($self, HashMapPass, $cx, $e);
        $crate::run_check_expr_field!($self, MinMaxPass, $cx, $e);
        $crate::run_check_expr_field!($self, NoEffect, $cx, $e);
        $crate::run_check_expr_field!($self, UselessVec, $cx, $e);
        $crate::run_check_expr_field!($self, PanicUnimplemented, $cx, $e);
        $crate::run_check_expr_field!($self, StringLitAsBytes, $cx, $e);
        $crate::run_check_expr_field!($self, Regex, $cx, $e);
        $crate::run_check_expr_field!($self, CopyAndPaste, $cx, $e);
        $crate::run_check_expr_field!($self, UselessFormat, $cx, $e);
        $crate::run_check_expr_field!($self, PanickingOverflowChecks, $cx, $e);
        $crate::run_check_expr_field!($self, Functions, $cx, $e);
        $crate::run_check_expr_field!($self, EvalOrderDependence, $cx, $e);
        $crate::run_check_expr_field!($self, MatchResultOk, $cx, $e);
        $crate::run_check_expr_field!($self, InfiniteIter, $cx, $e);
        $crate::run_check_expr_field!($self, UselessConversion, $cx, $e);
        $crate::run_check_expr_field!($self, QuestionMark, $cx, $e);
        $crate::run_check_expr_field!($self, SuspiciousImpl, $cx, $e);
        $crate::run_check_expr_field!($self, NonCopyConst, $cx, $e);
        $crate::run_check_expr_field!($self, AssertionsOnConstants, $cx, $e);
        $crate::run_check_expr_field!($self, AssertionsOnResultStates, $cx, $e);
        $crate::run_check_expr_field!($self, ComparisonChain, $cx, $e);
        $crate::run_check_expr_field!($self, FormatImpl, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantClosureCall, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessParensOnRangeLiterals, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessContinue, $cx, $e);
        $crate::run_check_expr_field!($self, Default, $cx, $e);
        $crate::run_check_expr_field!($self, DebugAssertWithMutCall, $cx, $e);
        $crate::run_check_expr_field!($self, LargeStackArrays, $cx, $e);
        $crate::run_check_expr_field!($self, FloatingPointArithmetic, $cx, $e);
        $crate::run_check_expr_field!($self, Dereferencing, $cx, $e);
        $crate::run_check_expr_field!($self, OptionIfLetElse, $cx, $e);
        $crate::run_check_expr_field!($self, IfLetMutex, $cx, $e);
        $crate::run_check_expr_field!($self, PatternTypeMismatch, $cx, $e);
        $crate::run_check_expr_field!($self, UnwrapInResult, $cx, $e);
        $crate::run_check_expr_field!($self, AsyncYieldsAsync, $cx, $e);
        $crate::run_check_expr_field!($self, DisallowedMethods, $cx, $e);
        $crate::run_check_expr_field!($self, StrToString, $cx, $e);
        $crate::run_check_expr_field!($self, VecInitThenPush, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantSlicing, $cx, $e);
        $crate::run_check_expr_field!($self, IfThenSomeElseNone, $cx, $e);
        $crate::run_check_expr_field!($self, BoolAssertComparison, $cx, $e);
        $crate::run_check_expr_field!($self, ManualAssert, $cx, $e);
        $crate::run_check_expr_field!($self, FormatArgs, $cx, $e);
        $crate::run_check_expr_field!($self, OnlyUsedInRecursion, $cx, $e);
        $crate::run_check_expr_field!($self, Write, $cx, $e);
        $crate::run_check_expr_field!($self, FormatPushString, $cx, $e);
        $crate::run_check_expr_field!($self, TrimSplitWhitespace, $cx, $e);
        $crate::run_check_expr_field!($self, ManualRetain, $cx, $e);
        $crate::run_check_expr_field!($self, Operators, $cx, $e);
        $crate::run_check_expr_field!($self, PartialeqToNone, $cx, $e);
        $crate::run_check_expr_field!($self, ManualAbsDiff, $cx, $e);
        $crate::run_check_expr_field!($self, ManualClamp, $cx, $e);
        $crate::run_check_expr_field!($self, BoolToIntWithIf, $cx, $e);
        $crate::run_check_expr_field!($self, ImplicitSaturatingAdd, $cx, $e);
        $crate::run_check_expr_field!($self, ManualIsAsciiCheck, $cx, $e);
        $crate::run_check_expr_field!($self, MissingAssertMessage, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantAsyncBlock, $cx, $e);
        $crate::run_check_expr_field!($self, ArcWithNonSendSync, $cx, $e);
        $crate::run_check_expr_field!($self, LegacyNumericConstants, $cx, $e);
        $crate::run_check_expr_field!($self, ReserveAfterInitialization, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessBorrowsForGenericArgs, $cx, $e);
        $crate::run_check_expr_field!($self, IncompatibleMsrv, $cx, $e);
        $crate::run_check_expr_field!($self, AssigningClones, $cx, $e);
        $crate::run_check_expr_field!($self, SetContainsOrInsert, $cx, $e);
        $crate::run_check_expr_field!($self, ManualIsPowerOfTwo, $cx, $e);
        $crate::run_check_expr_field!($self, NonZeroSuggestions, $cx, $e);
        $crate::run_check_expr_field!($self, LiteralStringWithFormattingArg, $cx, $e);
        $crate::run_check_expr_field!($self, CoerceContainerToAny, $cx, $e);
        $crate::run_check_expr_field!($self, VolatileComposites, $cx, $e);
        $crate::run_check_expr_field!($self, DisallowedFields, $cx, $e);
        $crate::run_check_expr_field!($self, ByteCharSlice, $cx, $e);
        match $e.kind {
            rustc_hir::ExprKind::Call(..) => {
                $crate::run_check_expr_field!($self, MemReplace, $cx, $e);
                $crate::run_check_expr_field!($self, Transmute, $cx, $e);
                $crate::run_check_expr_field!($self, DropForgetRef, $cx, $e);
                $crate::run_check_expr_field!($self, Exit, $cx, $e);
                $crate::run_check_expr_field!($self, FromStrRadix10, $cx, $e);
                $crate::run_check_expr_field!($self, StrlenOnCStrings, $cx, $e);
                $crate::run_check_expr_field!($self, DefaultIterEmpty, $cx, $e);
                $crate::run_check_expr_field!($self, ZombieProcesses, $cx, $e);
                $crate::run_check_expr_field!($self, NonStdLazyStatic, $cx, $e);
                $crate::run_check_expr_field!($self, SameLengthAndCapacity, $cx, $e);
                $crate::run_check_expr_field!($self, WithCapacityZero, $cx, $e);
            },
            rustc_hir::ExprKind::MethodCall(..) => {
                $crate::run_check_expr_field!($self, UnitReturnExpectingOrd, $cx, $e);
                $crate::run_check_expr_field!($self, ExplicitWrite, $cx, $e);
                $crate::run_check_expr_field!($self, ToDigitIsSome, $cx, $e);
                $crate::run_check_expr_field!($self, PermissionsSetReadonlyFalse, $cx, $e);
                $crate::run_check_expr_field!($self, UnnecessaryMapOnConstructor, $cx, $e);
                $crate::run_check_expr_field!($self, IneffectiveOpenOptions, $cx, $e);
                $crate::run_check_expr_field!($self, StringPatterns, $cx, $e);
                $crate::run_check_expr_field!($self, ZombieProcesses, $cx, $e);
            },
            rustc_hir::ExprKind::Binary(..) => {
                $crate::run_check_expr_field!($self, Ptr, $cx, $e);
                $crate::run_check_expr_field!($self, ZeroDiv, $cx, $e);
                $crate::run_check_expr_field!($self, NegMultiply, $cx, $e);
                $crate::run_check_expr_field!($self, ManualBits, $cx, $e);
                $crate::run_check_expr_field!($self, ManualRemEuclid, $cx, $e);
                $crate::run_check_expr_field!($self, ManualSliceSizeCalculation, $cx, $e);
                $crate::run_check_expr_field!($self, ManualFloatMethods, $cx, $e);
            },
            rustc_hir::ExprKind::Unary(..) => {
                $crate::run_check_expr_field!($self, NoNegCompOpForPartialOrd, $cx, $e);
            },
            rustc_hir::ExprKind::Lit(..) => {
                $crate::run_check_expr_field!($self, FloatLiteral, $cx, $e);
            },
            rustc_hir::ExprKind::Cast(..) => {
                $crate::run_check_expr_field!($self, AsConversions, $cx, $e);
            },
            rustc_hir::ExprKind::Let(..) => {
                $crate::run_check_expr_field!($self, PatternEquality, $cx, $e);
            },
            rustc_hir::ExprKind::If(..) => {
                $crate::run_check_expr_field!($self, ManualPopIf, $cx, $e);
            },
            rustc_hir::ExprKind::Match(..) => {
                $crate::run_check_expr_field!($self, QuestionMarkUsed, $cx, $e);
                $crate::run_check_expr_field!($self, LargeFuture, $cx, $e);
            },
            rustc_hir::ExprKind::Closure(..) => {
                $crate::run_check_expr_field!($self, AwaitHolding, $cx, $e);
            },
            rustc_hir::ExprKind::Assign(..) => {
                $crate::run_check_expr_field!($self, TemporaryAssignment, $cx, $e);
                $crate::run_check_expr_field!($self, SlowVectorInit, $cx, $e);
                $crate::run_check_expr_field!($self, PathbufThenPush, $cx, $e);
            },
            rustc_hir::ExprKind::Index(..) => {
                $crate::run_check_expr_field!($self, IndexingSlicing, $cx, $e);
            },
            rustc_hir::ExprKind::Path(..) => {
                $crate::run_check_expr_field!($self, ManualNonExhaustive, $cx, $e);
                $crate::run_check_expr_field!($self, NeedlessPassByRefMut, $cx, $e);
                $crate::run_check_expr_field!($self, SingleCallFn, $cx, $e);
            },
            rustc_hir::ExprKind::AddrOf(..) => {
                $crate::run_check_expr_field!($self, MutMut, $cx, $e);
                $crate::run_check_expr_field!($self, BorrowDerefRef, $cx, $e);
                $crate::run_check_expr_field!($self, UnnecessaryOwnedEmptyStrings, $cx, $e);
                $crate::run_check_expr_field!($self, ClonedRefToSliceRefs, $cx, $e);
            },
            rustc_hir::ExprKind::Break(..) => {
                $crate::run_check_expr_field!($self, UnusedUnit, $cx, $e);
            },
            rustc_hir::ExprKind::Ret(..) => {
                $crate::run_check_expr_field!($self, UnusedUnit, $cx, $e);
            },
            rustc_hir::ExprKind::InlineAsm(..) => {
                $crate::run_check_expr_field!($self, PointersInNomemAsmBlock, $cx, $e);
            },
            rustc_hir::ExprKind::Struct(..) => {
                $crate::run_check_expr_field!($self, InconsistentStructConstructor, $cx, $e);
                $crate::run_check_expr_field!($self, NeedlessUpdate, $cx, $e);
                $crate::run_check_expr_field!($self, NumberedFields, $cx, $e);
            },
            _ => {},
        }
    }};
}
