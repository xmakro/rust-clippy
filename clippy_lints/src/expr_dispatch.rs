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
        $crate::run_check_expr_field!($self, SerdeApi, $cx, $e);
        $crate::run_check_expr_field!($self, Types, $cx, $e);
        $crate::run_check_expr_field!($self, NonminimalBool, $cx, $e);
        $crate::run_check_expr_field!($self, UnportableVariant, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessBool, $cx, $e);
        $crate::run_check_expr_field!($self, BoolComparison, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessForEach, $cx, $e);
        $crate::run_check_expr_field!($self, LintPass, $cx, $e);
        $crate::run_check_expr_field!($self, EtaReduction, $cx, $e);
        $crate::run_check_expr_field!($self, UnnecessaryMutPassed, $cx, $e);
        $crate::run_check_expr_field!($self, SignificantDropTightening, $cx, $e);
        $crate::run_check_expr_field!($self, LenZero, $cx, $e);
        $crate::run_check_expr_field!($self, LenWithoutIsEmpty, $cx, $e);
        $crate::run_check_expr_field!($self, Attributes, $cx, $e);
        $crate::run_check_expr_field!($self, BlocksInConditions, $cx, $e);
        $crate::run_check_expr_field!($self, UninitVec, $cx, $e);
        $crate::run_check_expr_field!($self, StringAdd, $cx, $e);
        $crate::run_check_expr_field!($self, ImplicitReturn, $cx, $e);
        $crate::run_check_expr_field!($self, ImplicitSaturatingSub, $cx, $e);
        $crate::run_check_expr_field!($self, DefaultNumericFallback, $cx, $e);
        $crate::run_check_expr_field!($self, NonOctalUnixPermissions, $cx, $e);
        $crate::run_check_expr_field!($self, ApproxConstant, $cx, $e);
        $crate::run_check_expr_field!($self, Matches, $cx, $e);
        $crate::run_check_expr_field!($self, ManualStrip, $cx, $e);
        $crate::run_check_expr_field!($self, CheckedConversions, $cx, $e);
        $crate::run_check_expr_field!($self, Ranges, $cx, $e);
        $crate::run_check_expr_field!($self, FromOverInto, $cx, $e);
        $crate::run_check_expr_field!($self, UseSelf, $cx, $e);
        $crate::run_check_expr_field!($self, MissingConstForFn, $cx, $e);
        $crate::run_check_expr_field!($self, Casts, $cx, $e);
        $crate::run_check_expr_field!($self, SizeOfInElementCount, $cx, $e);
        $crate::run_check_expr_field!($self, SameNameMethod, $cx, $e);
        $crate::run_check_expr_field!($self, IndexRefutableSlice, $cx, $e);
        $crate::run_check_expr_field!($self, Shadow, $cx, $e);
        $crate::run_check_expr_field!($self, Methods, $cx, $e);
        $crate::run_check_expr_field!($self, UnitTypes, $cx, $e);
        $crate::run_check_expr_field!($self, Loops, $cx, $e);
        $crate::run_check_expr_field!($self, MainRecursion, $cx, $e);
        $crate::run_check_expr_field!($self, Lifetimes, $cx, $e);
        $crate::run_check_expr_field!($self, HashMapPass, $cx, $e);
        $crate::run_check_expr_field!($self, MinMaxPass, $cx, $e);
        $crate::run_check_expr_field!($self, Mutex, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessBorrowedRef, $cx, $e);
        $crate::run_check_expr_field!($self, NoEffect, $cx, $e);
        $crate::run_check_expr_field!($self, CognitiveComplexity, $cx, $e);
        $crate::run_check_expr_field!($self, BoxedLocal, $cx, $e);
        $crate::run_check_expr_field!($self, UselessVec, $cx, $e);
        $crate::run_check_expr_field!($self, PanicUnimplemented, $cx, $e);
        $crate::run_check_expr_field!($self, StringLitAsBytes, $cx, $e);
        $crate::run_check_expr_field!($self, Derive, $cx, $e);
        $crate::run_check_expr_field!($self, DerivableImpls, $cx, $e);
        $crate::run_check_expr_field!($self, EmptyEnums, $cx, $e);
        $crate::run_check_expr_field!($self, Regex, $cx, $e);
        $crate::run_check_expr_field!($self, CopyAndPaste, $cx, $e);
        $crate::run_check_expr_field!($self, CopyIterator, $cx, $e);
        $crate::run_check_expr_field!($self, UselessFormat, $cx, $e);
        $crate::run_check_expr_field!($self, Swap, $cx, $e);
        $crate::run_check_expr_field!($self, PanickingOverflowChecks, $cx, $e);
        $crate::run_check_expr_field!($self, NewWithoutDefault, $cx, $e);
        $crate::run_check_expr_field!($self, DisallowedNames, $cx, $e);
        $crate::run_check_expr_field!($self, Functions, $cx, $e);
        $crate::run_check_expr_field!($self, Documentation, $cx, $e);
        $crate::run_check_expr_field!($self, LetIfSeq, $cx, $e);
        $crate::run_check_expr_field!($self, EvalOrderDependence, $cx, $e);
        $crate::run_check_expr_field!($self, MissingDoc, $cx, $e);
        $crate::run_check_expr_field!($self, MissingInline, $cx, $e);
        $crate::run_check_expr_field!($self, ExhaustiveItems, $cx, $e);
        $crate::run_check_expr_field!($self, UnusedResultOk, $cx, $e);
        $crate::run_check_expr_field!($self, MatchResultOk, $cx, $e);
        $crate::run_check_expr_field!($self, PartialEqNeImpl, $cx, $e);
        $crate::run_check_expr_field!($self, UnusedIoAmount, $cx, $e);
        $crate::run_check_expr_field!($self, LargeEnumVariant, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessPassByValue, $cx, $e);
        $crate::run_check_expr_field!($self, PassByRefOrValue, $cx, $e);
        $crate::run_check_expr_field!($self, RefOptionRef, $cx, $e);
        $crate::run_check_expr_field!($self, InfiniteIter, $cx, $e);
        $crate::run_check_expr_field!($self, InlineFnWithoutBody, $cx, $e);
        $crate::run_check_expr_field!($self, UselessConversion, $cx, $e);
        $crate::run_check_expr_field!($self, ImplicitHasher, $cx, $e);
        $crate::run_check_expr_field!($self, FallibleImplFrom, $cx, $e);
        $crate::run_check_expr_field!($self, QuestionMark, $cx, $e);
        $crate::run_check_expr_field!($self, SuspiciousImpl, $cx, $e);
        $crate::run_check_expr_field!($self, MapUnit, $cx, $e);
        $crate::run_check_expr_field!($self, MultipleInherentImpl, $cx, $e);
        $crate::run_check_expr_field!($self, Unwrap, $cx, $e);
        $crate::run_check_expr_field!($self, NonCopyConst, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantClone, $cx, $e);
        $crate::run_check_expr_field!($self, UnnecessaryWraps, $cx, $e);
        $crate::run_check_expr_field!($self, AssertionsOnConstants, $cx, $e);
        $crate::run_check_expr_field!($self, AssertionsOnResultStates, $cx, $e);
        $crate::run_check_expr_field!($self, InherentToString, $cx, $e);
        $crate::run_check_expr_field!($self, TraitBounds, $cx, $e);
        $crate::run_check_expr_field!($self, ComparisonChain, $cx, $e);
        $crate::run_check_expr_field!($self, MutableKeyType, $cx, $e);
        $crate::run_check_expr_field!($self, FormatImpl, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantClosureCall, $cx, $e);
        $crate::run_check_expr_field!($self, Return, $cx, $e);
        $crate::run_check_expr_field!($self, ItemsAfterStatements, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessParensOnRangeLiterals, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessContinue, $cx, $e);
        $crate::run_check_expr_field!($self, ItemNameRepetitions, $cx, $e);
        $crate::run_check_expr_field!($self, UpperCaseAcronyms, $cx, $e);
        $crate::run_check_expr_field!($self, Default, $cx, $e);
        $crate::run_check_expr_field!($self, UnusedSelf, $cx, $e);
        $crate::run_check_expr_field!($self, DebugAssertWithMutCall, $cx, $e);
        $crate::run_check_expr_field!($self, LargeStackArrays, $cx, $e);
        $crate::run_check_expr_field!($self, LargeConstArrays, $cx, $e);
        $crate::run_check_expr_field!($self, FloatingPointArithmetic, $cx, $e);
        $crate::run_check_expr_field!($self, LetUnderscore, $cx, $e);
        $crate::run_check_expr_field!($self, ExcessiveBools, $cx, $e);
        $crate::run_check_expr_field!($self, WildcardImports, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantPubCrate, $cx, $e);
        $crate::run_check_expr_field!($self, Dereferencing, $cx, $e);
        $crate::run_check_expr_field!($self, OptionIfLetElse, $cx, $e);
        $crate::run_check_expr_field!($self, FutureNotSend, $cx, $e);
        $crate::run_check_expr_field!($self, IfLetMutex, $cx, $e);
        $crate::run_check_expr_field!($self, ManualAsyncFn, $cx, $e);
        $crate::run_check_expr_field!($self, PanicInResultFn, $cx, $e);
        $crate::run_check_expr_field!($self, MacroUseImports, $cx, $e);
        $crate::run_check_expr_field!($self, PatternTypeMismatch, $cx, $e);
        $crate::run_check_expr_field!($self, UnwrapInResult, $cx, $e);
        $crate::run_check_expr_field!($self, SemicolonIfNothingReturned, $cx, $e);
        $crate::run_check_expr_field!($self, AsyncYieldsAsync, $cx, $e);
        $crate::run_check_expr_field!($self, DisallowedMacros, $cx, $e);
        $crate::run_check_expr_field!($self, DisallowedMethods, $cx, $e);
        $crate::run_check_expr_field!($self, EmptyDrop, $cx, $e);
        $crate::run_check_expr_field!($self, StrToString, $cx, $e);
        $crate::run_check_expr_field!($self, ZeroSizedMapValues, $cx, $e);
        $crate::run_check_expr_field!($self, VecInitThenPush, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantSlicing, $cx, $e);
        $crate::run_check_expr_field!($self, IfThenSomeElseNone, $cx, $e);
        $crate::run_check_expr_field!($self, BoolAssertComparison, $cx, $e);
        $crate::run_check_expr_field!($self, UnusedAsync, $cx, $e);
        $crate::run_check_expr_field!($self, DisallowedTypes, $cx, $e);
        $crate::run_check_expr_field!($self, ImportRename, $cx, $e);
        $crate::run_check_expr_field!($self, SelfNamedConstructors, $cx, $e);
        $crate::run_check_expr_field!($self, IterNotReturningIterator, $cx, $e);
        $crate::run_check_expr_field!($self, ManualAssert, $cx, $e);
        $crate::run_check_expr_field!($self, NonSendFieldInSendTy, $cx, $e);
        $crate::run_check_expr_field!($self, UndocumentedUnsafeBlocks, $cx, $e);
        $crate::run_check_expr_field!($self, FormatArgs, $cx, $e);
        $crate::run_check_expr_field!($self, TrailingEmptyArray, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessLateInit, $cx, $e);
        $crate::run_check_expr_field!($self, ReturnSelfNotMustUse, $cx, $e);
        $crate::run_check_expr_field!($self, ManualBitWidth, $cx, $e);
        $crate::run_check_expr_field!($self, DefaultUnionRepresentation, $cx, $e);
        $crate::run_check_expr_field!($self, OnlyUsedInRecursion, $cx, $e);
        $crate::run_check_expr_field!($self, DbgMacro, $cx, $e);
        $crate::run_check_expr_field!($self, Write, $cx, $e);
        $crate::run_check_expr_field!($self, Cargo, $cx, $e);
        $crate::run_check_expr_field!($self, EmptyWithBrackets, $cx, $e);
        $crate::run_check_expr_field!($self, FormatPushString, $cx, $e);
        $crate::run_check_expr_field!($self, LargeIncludeFile, $cx, $e);
        $crate::run_check_expr_field!($self, TrimSplitWhitespace, $cx, $e);
        $crate::run_check_expr_field!($self, RcCloneInVecInit, $cx, $e);
        $crate::run_check_expr_field!($self, TypeParamMismatch, $cx, $e);
        $crate::run_check_expr_field!($self, ReadZeroByteVec, $cx, $e);
        $crate::run_check_expr_field!($self, ManualRetain, $cx, $e);
        $crate::run_check_expr_field!($self, ManualRotate, $cx, $e);
        $crate::run_check_expr_field!($self, Operators, $cx, $e);
        $crate::run_check_expr_field!($self, StdReexports, $cx, $e);
        $crate::run_check_expr_field!($self, UncheckedTimeSubtraction, $cx, $e);
        $crate::run_check_expr_field!($self, PartialeqToNone, $cx, $e);
        $crate::run_check_expr_field!($self, ManualAbsDiff, $cx, $e);
        $crate::run_check_expr_field!($self, ManualClamp, $cx, $e);
        $crate::run_check_expr_field!($self, ManualStringNew, $cx, $e);
        $crate::run_check_expr_field!($self, UnusedPeekable, $cx, $e);
        $crate::run_check_expr_field!($self, BoolToIntWithIf, $cx, $e);
        $crate::run_check_expr_field!($self, ImplicitSaturatingAdd, $cx, $e);
        $crate::run_check_expr_field!($self, MissingTraitMethods, $cx, $e);
        $crate::run_check_expr_field!($self, ConfusingXorAndPow, $cx, $e);
        $crate::run_check_expr_field!($self, ManualIsAsciiCheck, $cx, $e);
        $crate::run_check_expr_field!($self, SemicolonBlock, $cx, $e);
        $crate::run_check_expr_field!($self, MultipleUnsafeOpsPerBlock, $cx, $e);
        $crate::run_check_expr_field!($self, ExtraUnusedTypeParameters, $cx, $e);
        $crate::run_check_expr_field!($self, NoMangleWithRustAbi, $cx, $e);
        $crate::run_check_expr_field!($self, CollectionIsNeverRead, $cx, $e);
        $crate::run_check_expr_field!($self, MissingAssertMessage, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessMaybeSized, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantAsyncBlock, $cx, $e);
        $crate::run_check_expr_field!($self, ManualMainSeparatorStr, $cx, $e);
        $crate::run_check_expr_field!($self, UnnecessaryBoxReturns, $cx, $e);
        $crate::run_check_expr_field!($self, TestsOutsideTestModule, $cx, $e);
        $crate::run_check_expr_field!($self, ItemsAfterTestModule, $cx, $e);
        $crate::run_check_expr_field!($self, DefaultConstructedUnitStructs, $cx, $e);
        $crate::run_check_expr_field!($self, MissingFieldsInDebug, $cx, $e);
        $crate::run_check_expr_field!($self, EndianBytes, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantTypeAnnotations, $cx, $e);
        $crate::run_check_expr_field!($self, ArcWithNonSendSync, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessIfs, $cx, $e);
        $crate::run_check_expr_field!($self, MinIdentChars, $cx, $e);
        $crate::run_check_expr_field!($self, LargeStackFrames, $cx, $e);
        $crate::run_check_expr_field!($self, SingleRangeInVecInit, $cx, $e);
        $crate::run_check_expr_field!($self, NonCanonicalImpls, $cx, $e);
        $crate::run_check_expr_field!($self, LegacyNumericConstants, $cx, $e);
        $crate::run_check_expr_field!($self, ManualRangePatterns, $cx, $e);
        $crate::run_check_expr_field!($self, TupleArrayConversions, $cx, $e);
        $crate::run_check_expr_field!($self, FourForwardSlashes, $cx, $e);
        $crate::run_check_expr_field!($self, ErrorImplError, $cx, $e);
        $crate::run_check_expr_field!($self, AbsolutePaths, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantLocals, $cx, $e);
        $crate::run_check_expr_field!($self, IgnoredUnitPatterns, $cx, $e);
        $crate::run_check_expr_field!($self, ReserveAfterInitialization, $cx, $e);
        $crate::run_check_expr_field!($self, ImpliedBoundsInImpls, $cx, $e);
        $crate::run_check_expr_field!($self, MissingAssertsForIndexing, $cx, $e);
        $crate::run_check_expr_field!($self, NeedlessBorrowsForGenericArgs, $cx, $e);
        $crate::run_check_expr_field!($self, ManualHashOne, $cx, $e);
        $crate::run_check_expr_field!($self, IterWithoutIntoIter, $cx, $e);
        $crate::run_check_expr_field!($self, IterOverHashType, $cx, $e);
        $crate::run_check_expr_field!($self, ImplHashWithBorrowStrBytes, $cx, $e);
        $crate::run_check_expr_field!($self, RepeatVecWithCapacity, $cx, $e);
        $crate::run_check_expr_field!($self, UnconditionalRecursion, $cx, $e);
        $crate::run_check_expr_field!($self, PubUnderscoreFields, $cx, $e);
        $crate::run_check_expr_field!($self, MissingConstForThreadLocal, $cx, $e);
        $crate::run_check_expr_field!($self, IncompatibleMsrv, $cx, $e);
        $crate::run_check_expr_field!($self, ToStringTraitImpl, $cx, $e);
        $crate::run_check_expr_field!($self, AssigningClones, $cx, $e);
        $crate::run_check_expr_field!($self, ZeroRepeatSideEffects, $cx, $e);
        $crate::run_check_expr_field!($self, ExprMetavarsInUnsafe, $cx, $e);
        $crate::run_check_expr_field!($self, SetContainsOrInsert, $cx, $e);
        $crate::run_check_expr_field!($self, ManualIsPowerOfTwo, $cx, $e);
        $crate::run_check_expr_field!($self, NonZeroSuggestions, $cx, $e);
        $crate::run_check_expr_field!($self, LiteralStringWithFormattingArg, $cx, $e);
        $crate::run_check_expr_field!($self, UnusedTraitNames, $cx, $e);
        $crate::run_check_expr_field!($self, ManualIgnoreCaseCmp, $cx, $e);
        $crate::run_check_expr_field!($self, UnnecessaryLiteralBound, $cx, $e);
        $crate::run_check_expr_field!($self, ArbitrarySourceItemOrdering, $cx, $e);
        $crate::run_check_expr_field!($self, UselessConcat, $cx, $e);
        $crate::run_check_expr_field!($self, UnneededStructPattern, $cx, $e);
        $crate::run_check_expr_field!($self, UnnecessarySemicolon, $cx, $e);
        $crate::run_check_expr_field!($self, ManualOptionAsSlice, $cx, $e);
        $crate::run_check_expr_field!($self, SingleOptionMap, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantTestPrefix, $cx, $e);
        $crate::run_check_expr_field!($self, InfallibleTryFrom, $cx, $e);
        $crate::run_check_expr_field!($self, CoerceContainerToAny, $cx, $e);
        $crate::run_check_expr_field!($self, ToplevelRefArg, $cx, $e);
        $crate::run_check_expr_field!($self, VolatileComposites, $cx, $e);
        $crate::run_check_expr_field!($self, DisallowedFields, $cx, $e);
        $crate::run_check_expr_field!($self, ManualIlog2, $cx, $e);
        $crate::run_check_expr_field!($self, DurationSuboptimalUnits, $cx, $e);
        $crate::run_check_expr_field!($self, ManualNoopWaker, $cx, $e);
        $crate::run_check_expr_field!($self, ByteCharSlice, $cx, $e);
        $crate::run_check_expr_field!($self, ManualAssertEq, $cx, $e);
        $crate::run_check_expr_field!($self, RefPatterns, $cx, $e);
        $crate::run_check_expr_field!($self, RedundantElse, $cx, $e);
        match $e.kind {
            rustc_hir::ExprKind::Call(..) => {
                $crate::run_check_expr_field!($self, MemReplace, $cx, $e);
                $crate::run_check_expr_field!($self, Transmute, $cx, $e);
                $crate::run_check_expr_field!($self, DropForgetRef, $cx, $e);
                $crate::run_check_expr_field!($self, CreateDir, $cx, $e);
                $crate::run_check_expr_field!($self, Exit, $cx, $e);
                $crate::run_check_expr_field!($self, FromStrRadix10, $cx, $e);
                $crate::run_check_expr_field!($self, StrlenOnCStrings, $cx, $e);
                $crate::run_check_expr_field!($self, SwapPtrToRef, $cx, $e);
                $crate::run_check_expr_field!($self, DefaultIterEmpty, $cx, $e);
                $crate::run_check_expr_field!($self, BoxDefault, $cx, $e);
                $crate::run_check_expr_field!($self, FromRawWithVoidPtr, $cx, $e);
                $crate::run_check_expr_field!($self, SizeOfRef, $cx, $e);
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
                $crate::run_check_expr_field!($self, DerefAddrOf, $cx, $e);
                $crate::run_check_expr_field!($self, UninhabitedReferences, $cx, $e);
            },
            rustc_hir::ExprKind::Lit(..) => {
                $crate::run_check_expr_field!($self, FloatLiteral, $cx, $e);
                $crate::run_check_expr_field!($self, Unicode, $cx, $e);
            },
            rustc_hir::ExprKind::Cast(..) => {
                $crate::run_check_expr_field!($self, AsConversions, $cx, $e);
            },
            rustc_hir::ExprKind::Let(..) => {
                $crate::run_check_expr_field!($self, PatternEquality, $cx, $e);
            },
            rustc_hir::ExprKind::If(..) => {
                $crate::run_check_expr_field!($self, CollapsibleIf, $cx, $e);
                $crate::run_check_expr_field!($self, IfNotElse, $cx, $e);
                $crate::run_check_expr_field!($self, ManualTake, $cx, $e);
                $crate::run_check_expr_field!($self, ManualCheckedOps, $cx, $e);
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
                $crate::run_check_expr_field!($self, ReplaceBox, $cx, $e);
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
                $crate::run_check_expr_field!($self, NeedlessQuestionMark, $cx, $e);
                $crate::run_check_expr_field!($self, UnusedUnit, $cx, $e);
            },
            rustc_hir::ExprKind::InlineAsm(..) => {
                $crate::run_check_expr_field!($self, PointersInNomemAsmBlock, $cx, $e);
            },
            rustc_hir::ExprKind::Struct(..) => {
                $crate::run_check_expr_field!($self, InconsistentStructConstructor, $cx, $e);
                $crate::run_check_expr_field!($self, NeedlessUpdate, $cx, $e);
                $crate::run_check_expr_field!($self, NumberedFields, $cx, $e);
                $crate::run_check_expr_field!($self, UnnecessaryStruct, $cx, $e);
            },
            _ => {},
        }
    }};
}
