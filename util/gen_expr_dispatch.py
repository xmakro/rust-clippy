#!/usr/bin/env python3
"""Generate the ExprKind dispatch table for CombinedLateLintPass::check_expr.

For each pass in the combined list (parsed from lib.rs), locate its LateLintPass
impl block and its check_expr, then decide:
  - provably kind-gated       -> dispatch under the listed ExprKind variants
  - anything else             -> always-call (including passes where no check_expr
    is found: their default no-op impl inlines to nothing, so this is free and safe)

"Provably kind-gated" is deliberately strict: the entire body must be a single
`if` expression with no else and nothing after it, whose `&&` chain contains a
`let <pat> = <expr>.kind` conjunct where <pat> only matches ExprKind variants,
and every conjunct before that let must be a pure span test (from_expansion or
in_external_macro forms); or the body must start with `let <ExprKind pat> =
<expr>.kind else { return; }`.
"""
import re, os, sys, json

ROOT = os.path.expanduser("~/workspace/wt-nw/clippy_lints/src")
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
        elif c == "'" and j+2 < n and s[j+1] != '\\' and s[j+2] == "'":
            j += 2
        elif c == "'" and j+3 < n and s[j+1] == '\\':
            j += 1
            while j < n and s[j] != "'": j += 1
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
    for im in re.finditer(rf'impl(?:<[^>]*>)?\s*LateLintPass<[^>]*>\s*for\s+{ty}\b[^{{]*\{{', s):
        end = matching_brace(s, im.end()-1)
        block = s[im.end():end-1]
        fm = re.search(r'fn check_expr\s*\(\s*&mut self,\s*\w+:\s*&\w*\s*LateContext[^,]*,\s*(\w+):\s*&', block)
        if fm:
            bstart = block.index('{', fm.end())
            bend = matching_brace(block, bstart)
            return strip_comments(block[bstart+1:bend-1]).strip(), fm.group(1)
    return None, None

PURE_CONJ = re.compile(
    r'^!?\s*(?:\w+\.)*span(?:\(\))?\.from_expansion\(\)$'
    r'|^!?\s*is_in_external_macro\(.*\)$'
    r'|^!?\s*(?:\w+\.)*span\.in_external_macro\(.*\)$'
)

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

def top_level_statements(b):
    """split body into top-level statements (string-of-text chunks)."""
    stmts, depth, start = [], 0, 0
    i, n = 0, len(b)
    while i < n:
        c = b[i]
        if c in '([{': depth += 1
        elif c in ')]}':
            depth -= 1
            if depth == 0 and c == '}':
                # end of a block statement (if/match/...) unless followed by `else`
                j = i + 1
                while j < n and b[j] in ' \n\t': j += 1
                if b[j:j+4] =='else' or b[j:j+1] in (';', ',', '.', '?'):
                    i = j; continue
                stmts.append(b[start:i+1].strip()); start = i+1
        elif c == ';' and depth == 0:
            stmts.append(b[start:i+1].strip()); start = i+1
        elif c == '"':
            i += 1
            while i < n and b[i] != '"':
                if b[i] == '\\': i += 1
                i += 1
        elif c == "'" and i+2 < n and b[i+1] != '\\' and b[i+2] == "'":
            i += 2
        elif c == "'" and i+3 < n and b[i+1] == '\\':
            i += 1
            while i < n and b[i] != "'": i += 1
        i += 1
    tail = b[start:].strip()
    if tail: stmts.append(tail)
    return stmts

PURE_TXT = re.compile(r'^[!\s]*[\w\.\(\)::&, ]*$')
def is_pure_guard_cond(cond):
    for cj in split_conjuncts(cond):
        if not (PURE_CONJ.match(cj)
                or re.match(r'^!?\s*is_direct_expn_of\([^;{}]*\)\.is_(none|some)\(\)$', cj)):
            return False
    return True

PURE_LET = re.compile(r'^let\s+\w+\s*=\s*[\w\.]+\.span\.from_expansion\(\)\s*;$|^let\s+\w+\s*=\s*[\w\.]+\.span\(\)\.from_expansion\(\)\s*;$')

def kinds_of_if_stmt(stmt, var):
    """form-A single if statement -> kinds or None"""
    if not stmt.startswith('if '):
        return None
    depth = 0; obr = None; i = 2
    while i < len(stmt):
        c = stmt[i]
        if c in '([': depth += 1
        elif c in ')]': depth -= 1
        elif c == '{' and depth == 0:
            obr = i; break
        i += 1
    if obr is None: return None
    cond = stmt[3:obr].strip()
    body_end = matching_brace(stmt, obr)
    if stmt[body_end:].strip():
        return None  # else or trailing junk inside this statement chunk
    conjs = split_conjuncts(cond)
    for idx0, cj in enumerate(conjs):
        lm = re.match(rf'let\s+(.+?)\s*=\s*&?{var}\.kind$', cj, re.S)
        if lm:
            pat = lm.group(1)
            ks = sorted(set(re.findall(r'ExprKind::(\w+)', pat)))
            alts = [a.strip() for a in re.split(r'\|(?![\|])', pat)] if pat.count('|') else [pat.strip()]
            if not ks or any(k not in KINDS for k in ks):
                return None
            if not all(a.startswith('ExprKind::') or a.startswith('hir::ExprKind::') for a in alts):
                return None
            for prev in conjs[:idx0]:
                if not (PURE_CONJ.match(prev) or re.match(r'^!?\s*\w+$', prev)):
                    return None
            return ks
    return None

def kinds_of_match_stmt(stmt, var):
    """match var.kind { arms } with empty-or-absent wildcard -> kinds or None"""
    m0 = re.match(rf'match\s+&?{var}\.kind\s*\{{', stmt)
    if not m0:
        return None
    obr = stmt.index('{', m0.start())
    end = matching_brace(stmt, obr)
    if stmt[end:].strip():
        return None
    arms_text = stmt[obr+1:end-1]
    # split arms at top level on '=>' boundaries: simpler heuristic, split arm heads by scanning
    kinds = set()
    i, n, depth = 0, len(arms_text), 0
    head_start = 0
    while i < n:
        c = arms_text[i]
        if c in '([{': depth += 1
        elif c in ')]}': depth -= 1
        elif depth == 0 and arms_text[i:i+2] == '=>':
            head = arms_text[head_start:i].strip()
            # strip guard
            head_nog = head.split(' if ')[0].strip()
            body_start = i + 2
            # arm body: block or expression to top-level comma
            j = body_start
            while j < n and arms_text[j] in ' \n\t': j += 1
            if j < n and arms_text[j] == '{':
                bend = matching_brace(arms_text, j)
                body = arms_text[j+1:bend-1].strip()
                k = bend
                while k < n and arms_text[k] in ' ,\n\t': k += 1
            else:
                d2 = 0; k = j
                while k < n:
                    ch = arms_text[k]
                    if ch in '([{': d2 += 1
                    elif ch in ')]}': d2 -= 1
                    elif ch == ',' and d2 == 0: break
                    k += 1
                body = arms_text[j:k].strip()
                k += 1
            if head_nog == '_' or not head_nog.startswith(('ExprKind::', 'hir::ExprKind::')):
                # wildcard/binding arm must be empty
                if body not in ('', '()', '{}'):
                    return None
            else:
                ks = set(re.findall(r'ExprKind::(\w+)', head_nog))
                if not ks or any(kk not in KINDS for kk in ks):
                    return None
                kinds |= ks
            head_start = k
            i = k
            continue
        i += 1
    return sorted(kinds) if kinds else None

def classify(body, var):
    if body is None:
        return ("none", [])
    b = body.strip()
    # form B: let <pat> = var.kind else { return };  followed by anything
    mb = re.match(rf'let\s+(.+?)\s*=\s*{var}\.kind\s+else\s*\{{', b, re.S)
    if mb:
        pat = mb.group(1)
        kinds = sorted(set(re.findall(r'ExprKind::(\w+)', pat)))
        else_end = matching_brace(b, b.index('{', mb.start()))
        else_body = b[b.index('{', mb.start())+1:else_end-1].strip()
        if kinds and all(k in KINDS for k in kinds) and re.fullmatch(r'return\s*;?', else_body):
            return ("kinds", kinds)
        return ("always", [])
    # form C: [pure guard returns / pure lets]* then [kind-gated ifs / kind matches]+, nothing else
    stmts = top_level_statements(b)
    if not stmts:
        return ("always", [])
    kinds = set()
    seen_kind_block = False
    for st in stmts:
        gm = re.match(r'if\s+(.+?)\s*\{\s*return\s*;?\s*\}$', st, re.S)
        if gm and not seen_kind_block and is_pure_guard_cond(gm.group(1).strip()):
            continue
        if PURE_LET.match(st) and not seen_kind_block:
            continue
        if re.match(r'^use [\w:{}, ]+;$', st) and not seen_kind_block:
            continue
        ks = kinds_of_if_stmt(st, var)
        if ks is None:
            ks = kinds_of_match_stmt(st, var)
        if ks is None:
            return ("always", [])
        kinds |= set(ks)
        seen_kind_block = True
    if seen_kind_block and kinds:
        return ("kinds", sorted(kinds))
    return ("always", [])

# Hand-audited kind sets for passes whose check_expr the scanner cannot prove.
# Every entry documents the audit; re-audit when the pass's check_expr changes.
MANUAL_KINDS = {
    # if-let ExprKind::Match branch, else-if higher::IfLet::hir (ExprKind::If with a Let
    # condition), then higher::WhileLet::hir twice (ExprKind::Loop while-let desugar); the
    # leading guard and span binding are pure.
    "Matches": ["Match", "If", "Loop"],
    # higher::ForLoop::hir matches ExprKind::DropTemps; the explicit branches match
    # ExprKind::Loop; while_let_on_iterator and higher::While/WhileLet resolve to
    # ExprKind::Loop; the final branch matches ExprKind::MethodCall.
    "Loops": ["DropTemps", "Loop", "MethodCall"],
    # if-let ExprKind::Cast branch; borrow_as_ptr::check_implicit_cast matches AddrOf;
    # cast_slice_from_raw_parts::check_implicit_cast and cast_slice_different_sizes use
    # expr.peel_blocks() and so also react to Block and Call; cast_ptr_alignment and
    # ptr_cast_constness method checks match MethodCall.
    "Casts": ["Cast", "Call", "MethodCall", "AddrOf", "Block"],
    # misc: check_used_underscore's leading match binds only Path, MethodCall, Struct and
    # Field expressions; everything after it only returns early.
    "LintPass": ["Path", "Field", "MethodCall", "Struct"],
}

verdicts = {}
for field, ty in entries:
    path, tyname = module_file(ty)
    if path is None:
        verdicts[field] = ("always", [], ty)  # be safe
        continue
    body, var = find_check_expr(path, tyname)
    kind, ks = classify(body, var) if body is not None else ("none", [])
    if field in MANUAL_KINDS:
        if kind == "kinds":
            print(f"note: {field} is provable automatically now; drop its manual entry", file=sys.stderr)
        kind, ks = "kinds", MANUAL_KINDS[field]
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
    else:
        # No provable kind gate, or no check_expr found: run unconditionally. A pass
        # without a check_expr impl inlines to nothing, so this is free and safe.
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
          open("/tmp/dispatch-verdicts.json", "w"), indent=1)
print("wrote expr_dispatch.rs", file=sys.stderr)
for f,(k,ks,ty) in sorted(verdicts.items()):
    if k == "kinds":
        print(f"  {f:35} {','.join(ks)}")
