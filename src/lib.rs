//! This crate provides raw FFI bindings to Rofi's C plugin interface,
//! manually written from the headers in `/usr/include/rofi`.
//!
//! See [the examples folder] for examples on how to use this crate.
//!
//! [the examples folder]: https://github.com/SabrinaJewson/rofi-plugin-sys.rs/tree/main/examples

pub use {cairo_sys, glib_sys};

mod types;
pub use types::*;

mod mode_private;
pub use mode_private::*;

mod mode;
pub use mode::*;

pub mod icon_fetcher;

pub mod helper;
