//! Macros for container literals with specific type.
//!
//! ```
//! use crate::map;
//!
//! # fn main() {
//! let map = map!{
//!     "a" => 1,
//!     "b" => 2,
//! };
//! # }
//! ```
//!
//! The macros uses `=>` syntax to separate the key and value for the
//! mapping macros. (It was not possible to use `:` as separator due to syntactic
//! restrictions in regular `macro_rules!` macros.)
//!
//! Note that rust macros are flexible in which brackets you use for the invocation.
//! You can use them as `map!{}` or `map![]` or `map!()`.
//!
//! Generic container macros already exist elsewhere, so those are not provided
//! here at the moment.

// Disable warnings
#[allow(unused_macros)]

/// Create a **LinkedHashMap** from a list of key-value pairs
///
/// ## Example
///
/// ```
/// use crate::map;
/// # fn main() {
///
/// let map = map!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! map {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(map!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { map!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = map!(@count $($key),*);
            let mut _map = ::linked_hash_map::LinkedHashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

/// Create a **LinkedHashSet** from a list of elements.
///
/// ## Example
///
/// ```
/// use crate::map;
/// # fn main() {
///
/// let set = set!{"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! set {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(set!(@single $rest)),*]));

    ($($key:expr,)+) => { set!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = set!(@count $($key),*);
            let mut _set = ::linked_hash_set::LinkedHashSet::with_capacity(_cap);
            $(
                let _ = _set.insert($key);
            )*
            _set
        }
    };
}