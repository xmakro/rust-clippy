#!/usr/bin/env python3
"""Generate the ExprKind dispatch table for CombinedLateLintPass::check_expr.

For each pass in the combined list (parsed from lib.rs), locate its LateLintPass
impl block and its check_expr, then decide:
  - no check_expr impl        -> omit (default no-op)
  - provably kind-gated       -> dispatch under the listed ExprKind variants
  - anything else             -> always-call

"Provably kind-gated" is deliberately strict: the entire body must be a single
`if` expression with no else and nothing after it, whose `&&` chain contains a
`let <pat> = <expr>.kind` conjunct where <pat> only matches ExprKind variants,
and every conjunct before that let must be a pure span test (from_expansion or
in_external_macro forms); or the body must start with `let <ExprKind pat> =
<expr>.kind else { return; }`.
"""
import re, os, sys, json

ROOT = os.path.expanduser("~/Documents/rust-clippy/clippy_lints/src")
LIB = os.path.join(ROOT, "..", "src", "lib.rs")
LIB = os.path.normpath(os.path.join(ROOT, "lib.rs"))

KINDS = ["ConstBlock","Array","Call","MethodCall","Use","Tup","Binary","Unary","Lit","Cast","Type","DropTemps","Let","If","Loop","Match","Closure","Block","Assign","AssignOp","Field","Index","Path","AddrOf","Break","Continue","Ret","Become","InlineAsm","OffsetOf","Struct","Repeat","Yield","UnsafeBinderCast","Err"]

def matching_brace(s, i):
    """index just past the brace matching s[i]=='{', skipping strings/comments crudely."""
    depth = 0
    j = i
    n = len(s)
    while j < n:
        c = s[j]
        if c == '{': depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0: return j + 1
        elif c == '"':
            j += 1
            while j < n and s[j] != '"':
                if s[j] == '\\': j += 1
                j += 1
        elif c == '/' and j+1 < n and s[j+1] == '/':
            while j < n and s[j] != '\n': j += 1
        elif c == '/' and j+1 < n and s[j+1] == '*':
            j += 2
            while j+1 < n and not (s[j] == '*' and s[j+1] == '/'): j += 1
            j += 1
        j += 1
    return n

def strip_comments(s):
    s = re.sub(r'//[^\n]*', '', s)
    s = re.sub(r'/\*.*?\*/', '', s, flags=re.S)
    return s

# ---- parse combined field list from lib.rs ----
lib = open(LIB).read()
m = re.search(r'rustc_lint::late_lint_methods!\(\s*crate::combined_late_lint_pass,\s*\[CombinedLateLintPass,[^\[]*\[(.*?)\n    \]\]\n\);', lib, re.S)
assert m, "combined late list not found"
entries = []
for em in re.finditer(r'^\s*(\w+):\s*([\w:]+)(?:<[^=]*?>)?\s*=', m.group(1), re.M):
    field, ty = em.group(1), em.group(2)
    entries.append((field, ty))
print(f"combined late passes: {len(entries)}", file=sys.stderr)

def module_file(typath):
    # e.g. operators::arithmetic_side_effects::ArithmeticSideEffects
    parts = typath.split("::")
    modparts, ty = parts[:-1], parts[-1]
    for cand in (os.path.join(ROOT, *modparts) + ".rs",
                 os.path.join(ROOT, *modparts, "mod.rs")):
        if os.path.exists(cand):
            return cand, ty
    return None, ty

def find_check_expr(path, ty):
    """return body text of check_expr in the LateLintPass impl for ty, and the expr var name."""
    s = open(path).read()
    for im in re.finditer(rf'impl<[^>]*>\s*LateLintPass<[^>]*>\s*for\s+{ty}\b[^{{]*\{{', s):
        end = matching_brace(s, im.end()-1)
        block = s[im.end():end-1]
        fm = re.search(r'fn check_expr\s*\(\s*&mut self,\s*\w+:\s*&\w*\s*LateContext[^,]*,\s*(\w+):\s*&', block)
        if fm:
            bstart = block.index('{', fm.end())
            bend = matching_brace(block, bstart)
            return strip_comments(block[bstart+1:bend-1]).strip(), fm.group(1)
    return None, None

PURE_CONJ = re.compile(r'^!?\s*(?:\w+\.)*\w+\.span\.from_expansion\(\)$|^!?\s*(?:\w+\.)*span\.from_expansion\(\)$|^!?\s*is_in_external_macro\([^()]*\)$|^!?\s*\w+\.span\.in_external_macro\([^()]*(?:\(\))?[^()]*\)$')

def split_conjuncts(cond):
    """split on top-level &&"""
    out, depth, cur = [], 0, ''
    i = 0
    while i < len(cond):
        c = cond[i]
        if c in '([{': depth += 1
        elif c in ')]}': depth -= 1
        if depth == 0 and cond[i:i+2] == '&&':
            out.append(cur.strip()); cur = ''; i += 2; continue
        cur += c; i += 1
    out.append(cur.strip())
    return out

def classify(body, var):
    if body is None:
        return ("none", [])
    b = body.strip()
    # form B: let <pat> = var.kind else { <diverge> }; rest...
    mb = re.match(rf'let\s+(.+?)\s*=\s*{var}\.kind\s+else\s*\{{', b, re.S)
    if mb:
        pat = mb.group(1)
        kinds = sorted(set(re.findall(r'ExprKind::(\w+)', pat)))
        else_end = matching_brace(b, b.index('{', mb.start()))
        else_body = b[b.index('{', mb.start())+1:else_end-1].strip()
        if kinds and all(k in KINDS for k in kinds) and re.fullmatch(r'return\s*;?', else_body):
            return ("kinds", kinds)
        return ("always", [])
    # form A: entire body is one if-expression, no else, nothing after
    if not b.startswith('if '):
        return ("always", [])
    # find the opening brace of the if body at depth 0 of parens
    depth = 0
    obr = None
    i = 2
    while i < len(b):
        c = b[i]
        if c in '([': depth += 1
        elif c in ')]': depth -= 1
        elif c == '{' and depth == 0:
            obr = i; break
        elif c == '|' and b[i:i+2] == '|' and depth == 0:
            pass
        i += 1
    if obr is None: return ("always", [])
    cond = b[3:obr].strip()
    body_end = matching_brace(b, obr)
    tail = b[body_end:].strip()
    if tail:  # else clause or trailing statements
        return ("always", [])
    conjs = split_conjuncts(cond)
    kinds = None
    for idx, cj in enumerate(conjs):
        lm = re.match(rf'let\s+(.+?)\s*=\s*{var}\.kind$', cj, re.S) or \
             re.match(rf'let\s+(.+?)\s*=\s*&?{var}\.kind$', cj, re.S)
        if lm:
            pat = lm.group(1)
            ks = sorted(set(re.findall(r'ExprKind::(\w+)', pat)))
            # pattern must be pure ExprKind alternatives (all top-level alternatives start with ExprKind::)
            alts = [a.strip() for a in re.split(r'\|(?![\|])', pat)] if pat.count('|') else [pat.strip()]
            if not ks or any(k not in KINDS for k in ks):
                return ("always", [])
            if not all(a.startswith('ExprKind::') or a.startswith('hir::ExprKind::') for a in alts):
                return ("always", [])
            # all conjuncts BEFORE must be pure span tests
            for prev in conjs[:idx]:
                if not PURE_CONJ.match(prev):
                    return ("always", [])
            kinds = ks
            break
    if kinds:
        return ("kinds", kinds)
    return ("always", [])

verdicts = {}
for field, ty in entries:
    path, tyname = module_file(ty)
    if path is None:
        verdicts[field] = ("always", [], ty)  # be safe
        continue
    body, var = find_check_expr(path, tyname)
    kind, ks = classify(body, var) if body is not None else ("none", [])
    verdicts[field] = (kind, ks, ty)

n_none = sum(1 for v in verdicts.values() if v[0]=="none")
n_kind = sum(1 for v in verdicts.values() if v[0]=="kinds")
n_alw  = sum(1 for v in verdicts.values() if v[0]=="always")
print(f"none: {n_none}, kind-dispatch: {n_kind}, always: {n_alw}", file=sys.stderr)

# ---- emit the dispatch macro ----
by_kind = {k: [] for k in KINDS}
always = []
for field, (kind, ks, ty) in verdicts.items():
    if kind == "kinds":
        for k in ks: by_kind[k].append(field)
    elif kind == "always":
        always.append(field)

out = []
out.append("//! Generated ExprKind dispatch for `CombinedLateLintPass::check_expr`.")
out.append("//!")
out.append("//! Regenerate with `util/gen_expr_dispatch.py` after adding or changing a late pass;")
out.append("//! passes whose `check_expr` is not provably gated on `expr.kind` are called for every")
out.append("//! expression, so a stale table can only miss a speedup for new passes, never skip one,")
out.append("//! as long as new passes are added to the always-run list or the table is regenerated.")
out.append("")
out.append("/// Run one pass's `check_expr`, if active.")
out.append("#[macro_export]")
out.append("macro_rules! run_check_expr_field {")
out.append("    ($self:ident, $field:ident, $cx:expr, $e:expr) => {")
out.append("        if $self.$field.active {")
out.append("            rustc_lint::LateLintPass::check_expr(&mut $self.$field.pass, $cx, $e);")
out.append("        }")
out.append("    };")
out.append("}")
out.append("")
out.append("/// The generated `check_expr` body: always-run passes first, then the passes whose")
out.append("/// `check_expr` provably reacts only to specific `ExprKind`s, grouped per kind.")
out.append("#[macro_export]")
out.append("macro_rules! combined_check_expr_dispatch {")
out.append("    ($self:ident, $cx:expr, $e:expr) => {{")
for f in always:
    out.append(f"        $crate::run_check_expr_field!($self, {f}, $cx, $e);")
out.append("        match $e.kind {")
for k in KINDS:
    fields = by_kind[k]
    if not fields:
        continue
    out.append(f"            rustc_hir::ExprKind::{k}(..) => {{")
    for f in fields:
        out.append(f"                $crate::run_check_expr_field!($self, {f}, $cx, $e);")
    out.append("            },")
out.append("            _ => {},")
out.append("        }")
out.append("    }};")
out.append("}")

open(os.path.join(ROOT, "expr_dispatch.rs"), "w").write("\n".join(out) + "\n")
json.dump({f: {"verdict": v[0], "kinds": v[1], "type": v[2]} for f, v in verdicts.items()},
          open("/tmp/claude-1000/-home-makro-Documents/28a8f459-fed0-4010-932c-d8f8ae3a0b21/scratchpad/dispatch-verdicts.json", "w"), indent=1)
print("wrote expr_dispatch.rs", file=sys.stderr)
for f,(k,ks,ty) in sorted(verdicts.items()):
    if k == "kinds":
        print(f"  {f:35} {','.join(ks)}")
