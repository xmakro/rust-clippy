//! Check the pass gating of `combined_early_pass.rs`: a pass whose lints are
//! all allow-by-default is skipped unless something raises one of them above
//! `allow`. The cases the crate root level can't decide are lint attributes
//! below the crate root, which the gating finds with a crate-wide scan; each
//! case here would lose its warning (or report an unfulfilled expectation) if
//! that scan missed the attribute.
//@no-rustfix: `else_if_without_else` has no machine-applicable suggestion

#![allow(clippy::needless_return)]

// A lint level attribute directly on a function.
#[warn(clippy::else_if_without_else)]
fn attr_on_fn(a: bool, b: bool) {
    if a {
        println!("a");
    } else if b {
        //~^ else_if_without_else
        println!("b");
    }
}

// An inner attribute inside a module.
mod module_inner_attr {
    #![warn(clippy::else_if_without_else)]

    pub fn f(a: bool, b: bool) {
        if a {
            println!("a");
        } else if b {
            //~^ else_if_without_else
            println!("b");
        }
    }
}

// An attribute on an item nested inside an expression: the scan must descend
// into expressions to find it.
fn attr_in_nested_item() {
    let _ = {
        #[warn(clippy::else_if_without_else)]
        fn nested(a: bool, b: bool) {
            if a {
                println!("a");
            } else if b {
                //~^ else_if_without_else
                println!("b");
            }
        }
        nested(true, false);
        0
    };
}

// `expect` must keep the pass running, otherwise the fulfilled expectation
// would be reported as unfulfilled.
#[expect(clippy::redundant_else)]
fn expectation_is_fulfilled(b: bool) {
    if b {
        println!("early");
        return;
    } else {
        println!("rest");
    }
}

// A group mention must keep the passes of every lint in the group running.
#[warn(clippy::pedantic)]
fn group_mention(b: bool) {
    if b {
        println!("early");
        return;
    } else {
        //~^ redundant_else
        println!("rest");
    }
}

fn main() {
    attr_on_fn(true, false);
    module_inner_attr::f(true, false);
    attr_in_nested_item();
    expectation_is_fulfilled(true);
    group_mention(false);
}
