//! # Python-style collection comprehensions
//!
//! The structure is consistent with Python's [list and dictionary comprehensions](https://docs.python.org/3/reference/expressions.html#displays-for-lists-sets-and-dictionaries).
//!
//! - If the loop body (everything before the first `for`) contains two
//! expressions separated by `:`, it will be treated as a hash map comprehension.
//!
//! - If the loop body ends with a `;`, it will be a statement comprehension
//! (it won't evaluate to a collection).
//!
//! - Otherwise it's a vector comprehension.
//!
//! Comprehensions consist of a body followed by a `for ... in ...` expression,
//! followed by any combination of `for ... in ...` or `if ...` expressions.
//!
//! The `for ... in ...` and `if ...` expressions are nested left-to-right, so
//! ```
//! # extern crate comprende;
//! # use comprende::c;
//! # fn do_something(x: i32, y: char) {}
//! c!(do_something(x, y); for x in 0..10 if x % 2 == 1 for y in 'a'..='z');
//! ```
//!
//! is equivalent to:
//!
//! ```no_run
//! # fn do_something(x: i32, y: char) {}
//! for x in 0..10 {
//!     if x % 2 == 1 {
//!         for y in 'a'..='z' {
//!             do_something(x, y);
//!         }
//!     }
//! }
//! ```
//!
//! # Examples
//!
//! ## Vectors
//!
//! - A simple vector comprehension:
//! ```
//! # extern crate comprende;
//! # use comprende::c;
//! let v = c![x * (1 + x) for x in 1..=10];
//! println!("{:?}", v);
//! ```
//!
//! ```text
//! [2, 6, 12, 20, 30, 42, 56, 72, 90, 110]
//! ```
//!
//! - Multiple iterators and conditionals:
//! ```
//! # extern crate comprende;
//! # use comprende::c;
//! let v = c![x * y for x in 1..=10 if x % 2 != 0 for y in -2..=2 if x > y];
//! println!("{:?}", v);
//! ```
//! ```text
//! [-2, -1, 0, -6, -3, 0, 3, 6, -10, -5, 0, 5, 10, -14, -7, 0, 7, 14, -18, -9, 0, 9, 18]
//! ```
//!
//! ## Hash Maps
//!
//! - A simple hash map comprehension:
//! ```
//! # extern crate comprende;
//! # use comprende::c;
//! let m = c!{x: x * (1 + x) for x in 1..=10};
//! println!("{:?}", m);
//! ```
//! ```text
//! {2: 6, 5: 30, 8: 72, 4: 20, 3: 12, 6: 42, 1: 2, 7: 56, 9: 90, 10: 110}
//! ```
//!
//! - Complex hash map comprehension:
//! ```
//! # extern crate comprende;
//! # use comprende::c;
//! let m = c!{format!("[{}|{}]", x, y): y.to_string().repeat(x) for x in 1..=3 for y in 'A'..='C'};
//! println!("{:?}", m);
//! ```
//! ```text
//! {"[3|C]": "CCC", "[3|B]": "BBB", "[3|A]": "AAA", "[2|B]": "BB", "[1|C]": "C", "[1|A]": "A", "[1|B]": "B", "[2|C]": "CC", "[2|A]": "AA"}
//! ```
//!
//! ## Statements
//!
//! - A simple statement comprehension:
//! ```
//! # extern crate comprende;
//! # use comprende::c;
//! let mut n = 1;
//! c!(n *= x; for x in 1..=10);
//! println!("{}", n);
//! ```
//! ```text
//! 3628800
//! ```

extern crate clean_macro_docs;
use clean_macro_docs::clean_docs;

#[clean_docs]
#[macro_export]
macro_rules! c {
    // Preprocess the loop body expression.

    // Replace `:` with `=>` and proceed to @preprocess[1]
    (@preprocess[0] {: $($ts:tt)*} {$($procd_ts:tt)*}) =>
        { c!(@preprocess[1] {$($ts)*} {$($procd_ts)* =>}) };

    // Reached end of the loop body expression, proceed to @preprocess[1]
    (@preprocess[0] {for $($ts:tt)*} {$($procd_ts:tt)*}) =>
        { c!(@preprocess[1] {for $($ts)*} {$($procd_ts)*}) };

    // Continue to next token
    (@preprocess[0] {$t:tt $($ts:tt)*} {$($procd_ts:tt)*}) =>
        { c!(@preprocess[0] {$($ts)*} {$($procd_ts)* $t}) };

    // ERROR: No `for`
    (@preprocess[0] {} {$($procd_ts:tt)*}) =>
        { compile_error!("Comprehension must contain at least one `for ... in ...` expression") };


    // Preprocess the loop and conditional components.
    // Replaces instances of `for` with `, for` and `if` with `, if`.
    // This allows us to match with more specific fragments, such as
    // expr and stmt in the @construct phases.

    // ERROR: No loop body
    (@preprocess[1] {$($ts:tt)*} {, $($procd_ts:tt)*}) =>
        { compile_error!("Missing loop body") };
    // Replace `for` with `, for` and continue to next token
    (@preprocess[1] {for $($ts:tt)*} {$($procd_ts:tt)*}) =>
        { c!(@preprocess[1] {$($ts)*} {$($procd_ts)* , for}) };
    // Replace `if` with `, if` and continue to next token
    (@preprocess[1] {if $($ts:tt)*} {$($procd_ts:tt)*}) =>
        { c!(@preprocess[1] {$($ts)*} {$($procd_ts)* , if}) };

    // Continue to next token
    (@preprocess[1] {$t:tt $($ts:tt)*} {$($procd_ts:tt)*}) =>
        { c!(@preprocess[1] {$($ts)*} {$($procd_ts)* $t}) };

    // Done with preprocessing, continue to @construct[0]
    (@preprocess[1] {} {$($procd_ts:tt)*}) =>
        { c!(@construct[0] $($procd_ts)*) };


    // Start constructing the result.
    // If the loop body is an expression, create the appropriate collection.
    (@construct[0] $k:expr => $v:expr, for $($rest:tt)*) => {{
        let mut m = std::collections::HashMap::new();
        c![@construct[1] {m.insert($k, $v);}, for $($rest)*];
        m
    }};
    (@construct[0] $e:expr, for $($rest:tt)*) => {{
        let mut v = Vec::new();
        c![@construct[1] v.push($e), for $($rest)*];
        v
    }};
    (@construct[0] $s:stmt;, for $($rest:tt)*) => {{
        c![@construct[1] $s, for $($rest)*];
    }};

    // Construct the for-loops and if-expressions.
    (@construct[1] $s:stmt, for $el:ident in $iter:expr $(, $($rest:tt)*)?) => {{
        for $el in $iter {
            c![@construct[1] $s $(, $($rest)*)?]
        }
    }};
    (@construct[1] $s:stmt, for $p:pat in $iter:expr $(, $($rest:tt)*)?) => {{
        for $p in $iter {
            c![@construct[1] $s $(, $($rest)*)?]
        }
    }};
    (@construct[1] $s:stmt, for $($rest:tt)*) => {{
        compile_error!("Invalid for-loop")
    }};
    (@construct[1] $s:stmt, if $cond:expr $(, $($rest:tt)*)?) => {{
        if $cond {
            c![@construct[1] $s $(, $($rest)*)?]
        }
    }};
    (@construct[1] $s:stmt, if $($rest:tt)*) => {{
        compile_error!("Invalid if-expression")
    }};
    (@construct[1] $s:stmt) => {{
        $s
    }};

    // Public entry point
    ($($comp:tt)*) => {{
        c!(@preprocess[0] {$($comp)*} {})
    }};
}

#[cfg(test)]
mod tests {
    // Vector
    #[test]
    fn simple_vec() {
        let v = c![x * x for x in 1..=10];
        assert_eq!(v, vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
    }

    #[test]
    fn simple_cond_vec() {
        let v = c![x * x for x in 1..=10 if x % 2 == 0];
        assert_eq!(v, vec![4, 16, 36, 64, 100]);
    }

    #[test]
    fn for_for_vec() {
        let v = c![(x, y) for x in 1..=3 for y in 'a'..='c'];
        assert_eq!(
            v,
            vec![
                (1, 'a'),
                (1, 'b'),
                (1, 'c'),
                (2, 'a'),
                (2, 'b'),
                (2, 'c'),
                (3, 'a'),
                (3, 'b'),
                (3, 'c')
            ]
        );
    }

    #[test]
    fn for_for_if_vec() {
        let v = c![(x, y) for x in 1..=3 for y in 'a'..='c' if x % 2 != 0];
        assert_eq!(
            v,
            vec![(1, 'a'), (1, 'b'), (1, 'c'), (3, 'a'), (3, 'b'), (3, 'c')]
        );
    }

    #[test]
    fn for_if_for_vec() {
        let v = c![(x, y) for x in 1..=3 if x % 2 != 0 for y in 'a'..='c'];
        assert_eq!(
            v,
            vec![(1, 'a'), (1, 'b'), (1, 'c'), (3, 'a'), (3, 'b'), (3, 'c')]
        );
    }

    // HashMap
    #[test]
    fn simple_map() {
        let m = c! {x: x * x for x in 1..=10};
        assert_eq!(
            m,
            [
                (1, 1),
                (2, 4),
                (3, 9),
                (4, 16),
                (5, 25),
                (6, 36),
                (7, 49),
                (8, 64),
                (9, 81),
                (10, 100),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn simple_cond_map() {
        let m = c! {x: x * x for x in 1..=10 if x % 2 == 0};
        assert_eq!(
            m,
            [(2, 4), (4, 16), (6, 36), (8, 64), (10, 100)]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn for_for_map() {
        let m = c! {format!("{}|{}", x, y): (x, y) for x in 1..=3 for y in 'a'..='c'};
        assert_eq!(
            m,
            [
                ("1|a".to_string(), (1, 'a')),
                ("1|b".to_string(), (1, 'b')),
                ("1|c".to_string(), (1, 'c')),
                ("2|a".to_string(), (2, 'a')),
                ("2|b".to_string(), (2, 'b')),
                ("2|c".to_string(), (2, 'c')),
                ("3|a".to_string(), (3, 'a')),
                ("3|b".to_string(), (3, 'b')),
                ("3|c".to_string(), (3, 'c')),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn for_for_if_map() {
        let m = c! {format!("{}|{}", x, y): (x, y) for x in 1..=3 for y in 'a'..='c' if x % 2 != 0};
        assert_eq!(
            m,
            [
                ("1|a".to_string(), (1, 'a')),
                ("1|b".to_string(), (1, 'b')),
                ("1|c".to_string(), (1, 'c')),
                ("3|a".to_string(), (3, 'a')),
                ("3|b".to_string(), (3, 'b')),
                ("3|c".to_string(), (3, 'c')),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn for_if_for_map() {
        let m = c! {format!("{}|{}", x, y): (x, y) for x in 1..=3 if x % 2 != 0 for y in 'a'..='c'};
        assert_eq!(
            m,
            [
                ("1|a".to_string(), (1, 'a')),
                ("1|b".to_string(), (1, 'b')),
                ("1|c".to_string(), (1, 'c')),
                ("3|a".to_string(), (3, 'a')),
                ("3|b".to_string(), (3, 'b')),
                ("3|c".to_string(), (3, 'c')),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    // Statement
    #[test]
    fn simple_stmt() {
        let mut n = 0;
        c!(n += x * x; for x in 1..=10);
        assert_eq!(n, 385);
    }

    #[test]
    fn simple_cond_stmt() {
        let mut n = 0;
        c!(n += x * x; for x in 1..=10 if x % 2 == 0);
        assert_eq!(n, 220);
    }

    #[test]
    fn for_for_stmt() {
        let mut s = String::new();
        c!(s += &format!("[{}|{}]", x, y); for x in 1..=3 for y in 'a'..='c');
        assert_eq!(s, "[1|a][1|b][1|c][2|a][2|b][2|c][3|a][3|b][3|c]");
    }

    #[test]
    fn for_for_if_stmt() {
        let mut s = String::new();
        c!(s += &format!("[{}|{}]", x, y); for x in 1..=3 for y in 'a'..='c' if x % 2 != 0);
        assert_eq!(s, "[1|a][1|b][1|c][3|a][3|b][3|c]");
    }

    #[test]
    fn for_if_for_stmt() {
        let mut s = String::new();
        c!(s += &format!("[{}|{}]", x, y); for x in 1..=3 if x % 2 != 0 for y in 'a'..='c');
        assert_eq!(s, "[1|a][1|b][1|c][3|a][3|b][3|c]");
    }
}
