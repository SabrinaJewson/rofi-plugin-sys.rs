//! Functions related to Rofi’s menu view.
//!
//! This corresponds to `view.h`.
//!
//! WARNING: This is an undocumented API!
//! See <https://github.com/davatorium/rofi/blob/next/include/view.h> for the header file itself.
//! As such, the only functions bound here are functions with explicit conformation from the author
//! that they are public.

extern "C" {
    /// Indicate the current view needs to reload its data.
    /// This can only be done when *more* information is available.
    ///
    /// The reloading happens 'lazy', multiple calls might be handled at once.
    ///
    /// See [this
    /// comment](https://github.com/davatorium/rofi/discussions/1654#discussioncomment-3120858)
    /// that confirms this function can be used.
    #[link_name = "rofi_view_reload"]
    pub fn reload();
}
