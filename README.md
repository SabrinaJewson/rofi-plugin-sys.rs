# rofi-plugin-sys

This crate provides raw FFI bindings to Rofi's C plugin interface,
manually written from the headers in `/usr/include/rofi`.

See [the examples folder] for examples on how to use this crate.

These bindings are raw, `unsafe` and very low-level
so if you just want to write a Rofi plugin
you'll probably want an easier-to-use library instead,
such as [rofi-mode](https://docs.rs/rofi-mode).

[the examples folder]: https://github.com/SabrinaJewson/rofi-plugin-sys.rs/tree/main/examples

Set `RUSTFLAGS="--cfg rofi_next"` when building
to change the API to use unreleased Rofi features
(as of 2023-10-31).

License: MIT
