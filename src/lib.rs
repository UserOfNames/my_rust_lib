//! Collection of small, miscellaneous features I often find useful.
//!
//! When I'm writing Rust projects, I sometimes find myself writing the same piece of functionality
//! multiple times across projects. If this functionality meets a few criteria, I put it here:
//! - It's likely to be useful in more than one or two situations.
//! - It's relatively small and simple to understand.
//! - It's relatively obvious what it does based only on where and how it's used.
//! - It isn't (to my knowledge) implemented in the standard library or a popular crate.

/// This module contains helper macros for **A**ssignment, **I**nitialization, and **D**eclaration
/// of multiple variables at once.
///
/// These are rarely useful in practical Rust code. Most of the time, it is better to:
/// - Manually declare each variable, even if that's slightly more boilerplate.
/// - Store the values in a collection instead of several different variables.
///
/// Be mindful of whether macros like these are actually applicable, or whether your design is
/// flawed.
pub mod chain_aid;

/// This module contains anything related to helping control flow, such as a macro to check a
/// `Result` and `continue` if it's an `Err`.
pub mod control_flow;

/// This module contains anything related to I/O, such as helper functions to prompt input from a
/// user, similarly to Python's `input()` function.
pub mod io;
