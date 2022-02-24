//! Macros for Python-like list comprehension creation of iterators.
//!
//! Another Crate that tries to bring the simplicty of Python's syntax to Rust iterators.
//!
//! The main macro is [`iter`](macro@iter), and other macros are extensions of the latter.
//!
//! # Examples
//!
//! Below, small examples of how the macros work:
//! ```rust
//! use comptools::*;
//!
//! let vec: Vec<u64> = vect![x*x; for x in 0..=10; if x % 2 == 0];
//! assert_eq!(vec, vec![0, 4, 16, 36, 64, 100]);
//!
//! let sum: u64 = sum![x*x; for x in 1..; while x*x*x < 1234];
//! assert_eq!(sum, 385);
//! ```

/// Create an iterator using Python's list-comprehension style.
///
/// # Basic usage
///
/// ```rust
/// # #[macro_use] extern crate comptools;
/// // iter![f(x); for x in iter];
/// // Create an iterator
/// let iter = iter![x*x; for x in 1..10];
/// assert_eq!(iter.collect::<Vec<_>>(), vec![1, 4, 9, 16, 25, 36, 49, 64, 81]);
/// ```
///
/// # Variants
///
/// Below are all the possible variants you can use with the [`iter`](macro@iter) macro.
///
/// ## Filter by value
///
/// **Warning:** filter condition uses references
///
/// ```rust
/// # #[macro_use] extern crate comptools;
/// // iter![f(x); for x in iter; if cond(x)];
/// let iter = iter![x*x; for x in 1..10; if x < &5];
/// assert_eq!(iter.collect::<Vec<_>>(), vec![1, 4, 9, 16]);
/// // or:
/// // iter![f(x); if cond(x); for x in iter];
/// let iter = iter![x*x; if x < &5; for x in 1..10];
/// assert_eq!(iter.collect::<Vec<_>>(), vec![1, 4, 9, 16]);
/// // Same as filter_map
/// ```
///
/// ## Conditional mapping
/// ```rust
/// # #[macro_use] extern crate comptools;
/// // iter![f(x); for x in iter; if cond(x); else g(x)];
/// let iter = iter![x*x; for x in 1..10; if x < 5; else 0];
/// assert_eq!(iter.collect::<Vec<_>>(), vec![1, 4, 9, 16, 0, 0, 0, 0, 0]);
/// // or
/// // iter![f(x); if cond(x); else g(x); for x in iter];
/// let iter = iter![x*x; if x < 5; else 0; for x in 1..10];
/// assert_eq!(iter.collect::<Vec<_>>(), vec![1, 4, 9, 16, 0, 0, 0, 0, 0]);
/// ```
///
/// ## Map while
///
/// **Warning:** while condition uses references
///
/// ```rust
/// # #[macro_use] extern crate comptools;
/// // iter![f(x); for x in iter; while cond(x)];
/// let iter = iter![x*x; for x in 1..10; while x < &5];
/// assert_eq!(iter.collect::<Vec<_>>(), vec![1, 4, 9, 16]);
/// // or
/// // iter![f(x); while cond(x); for x in iter];
/// let iter = iter![x*x; while x < &5; for x in 1..10];
/// assert_eq!(iter.collect::<Vec<_>>(), vec![1, 4, 9, 16]);
/// ```
#[macro_export]
macro_rules! iter {
    // [f(x); for x in iter]
    ($exp:expr; for $item:ident in $iter:expr) => {
        $iter.map(|$item| $exp)
    };
    // [f(x); for x in iter; if cond(x)]
    ($exp:expr; for $item:ident in $iter:expr; if $ifexp:expr) => {
        $iter.filter(|$item| $ifexp).map(|$item| $exp)
    };
    // [f(x); for x in iter; if cond(x); else g(x)]
    ($exp:expr; for $item:ident in $iter:expr; if $ifexp:expr; else $elsexp:expr) => {
        $iter.map(|$item| if $ifexp {$exp} else {$elsexp})
    };
    // [f(x); for x in iter; while cond(x)]
    ($exp:expr; for $item:ident in $iter:expr; while $whilexp:expr) => {
        $iter.take_while(|$item| $whilexp).map(|$item| $exp)
    };
    // Below are alternative ways for calling this macro
    //
    ($exp:expr; if $ifexp:expr; for $item:ident in $iter:expr) => {
        iter![$exp; for $item in $iter; if $ifexp]
    };
    ($exp:expr; if $ifexp:expr; else $elsexp:expr; for $item:ident in $iter:expr) => {
        iter![$exp; for $item in $iter; if $ifexp; else $elsexp]
    };
    ($exp:expr; while $whilexp:expr; for $item:ident in $iter:expr) => {
        iter![$exp; for $item in $iter; while $whilexp]
    };
}

/// Return sum of values of an iterator using Python's list-comprehension style.
///
/// # Basic usage
///
/// ```rust
/// # #[macro_use] extern crate comptools;
/// // sum![f(x); for x in iter];
/// // Create an iterator and sum its values
/// let sum: u64 = sum![x*x; for x in 1..10];
/// assert_eq!(sum, 285);
/// // Same as iter![...].sum()
/// ```
///
/// For more details, refer to the documentation of [`iter`](macro@iter).
#[macro_export]
macro_rules! sum {
    ($($body:tt)*) => {{
    (iter![$($body)*]).sum()
    }};
}

/// Return product of values of an iterator using Python's list-comprehension style.
///
/// # Basic usage
///
/// ```rust
/// # #[macro_use] extern crate comptools;
/// // product![f(x); for x in iter];
/// // Create an iterator and multiply its values
/// let product: u64 = product![x*x; for x in 1..10];
/// assert_eq!(product, 131681894400);
/// // Same as iter![...].product()
/// ```
///
/// For more details, refer to the documentation of [`iter`](macro@iter).
#[macro_export]
macro_rules! product {
    ($($body:tt)*) => {{
    (iter![$($body)*]).product()
    }};
}

/// Create a collection using Python's list-comprehension style.
///
/// # Basic usage
///
/// ```rust
/// # #[macro_use] extern crate comptools;
/// // vect![f(x); for x in iter];
/// // Create a collection
/// let vec: Vec<u64> = vect![x*x; for x in 1..10];
/// assert_eq!(vec, vec![1, 4, 9, 16, 25, 36, 49, 64, 81]);
/// // Same as iter![...].collect()
/// ```
///
/// For more details, refer to the documentation of [`iter`](macro@iter).
#[macro_export]
macro_rules! vect {
    ($($body:tt)*) => {{
    (iter![$($body)*]).collect()
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vect() {
        let expected: Vec<u64> = (1..10).map(|x| x * x).collect();
        let got: Vec<u64> = vect![x*x; for x in 1..10];
        assert_eq!(expected, got);
    }
    #[test]
    fn test_vect_if() {
        let expected: Vec<u64> = (1..10).filter(|x| x < &5).map(|x| x * x).collect();
        let got: Vec<u64> = vect![x*x; for x in 1..10; if x < &5];
        assert_eq!(expected, got);
    }
    #[test]
    fn test_vect_if_else() {
        let expected: Vec<u64> = (1..10).map(|x| if x < 5 { x * x } else { 0 }).collect();
        let got: Vec<u64> = vect![x*x; for x in 1..10; if x < 5; else 0];
        assert_eq!(expected, got);
    }
    #[test]
    fn test_vect_while() {
        let expected: Vec<u64> = (1..).take_while(|x| x < &10).collect();
        let got: Vec<u64> = vect![x; for x in 1..; while x < &10];
        assert_eq!(expected, got);
    }
    #[test]
    fn test_sum() {
        let expected: u64 = (1..10).filter(|x| x < &5).map(|x| x * x).sum();
        let got = sum![x*x; for x in 1..10; if x < &5];
        assert_eq!(expected, got);
    }
    #[test]
    fn test_product() {
        let expected: u64 = (1..10).filter(|x| x < &5).map(|x| x * x).product();
        let got = product![x*x; for x in 1..10; if x < &5];
        assert_eq!(expected, got);
    }
}
