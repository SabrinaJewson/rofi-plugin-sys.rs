//! Small helper to fetch icons. This makes use of the 'view' threadpool.
//!
//! This corresponds to `rofi-icon-fetcher.h`.

use ::std::os::raw::{c_char, c_int};

extern "C" {
    /// Initialize the icon fetcher.
    #[link_name = "rofi_icon_fetcher_init"]
    pub fn init();

    /// Destroy and free the memory used by the icon fetcher.
    #[link_name = "rofi_icon_fetcher_destroy"]
    pub fn destroy();

    /// Query the icon-theme for icon with name and size.
    ///
    /// The returned icon will be the best match for the requested size,
    /// it should still be resized to the actual size.
    ///
    /// `name` can also be a full path, if prefixed with `file://`.
    ///
    /// Returns the UID identifying the request.
    #[link_name = "rofi_icon_fetcher_query"]
    pub fn query(name: *const c_char, size: c_int) -> u32;

    /// Query the icon-theme for icon with name and size.
    ///
    /// The returned icon will be the best match for the requested size,
    /// it should still be resized to the actual size.
    /// For icons it will take the min of wsize and hsize.
    ///
    /// `name` can also be a full path, if prefixed with `file://`.
    ///
    /// Returns the UID identifying the request.
    #[link_name = "rofi_icon_fetcher_query_advanced"]
    pub fn query_advanced(name: *const c_char, wsize: c_int, hsize: c_int) -> u32;

    /// Retrieves the surface with the icon, or `NULL` if it wasn't found.
    ///
    /// Accepts a request UID.
    #[link_name = "rofi_icon_fetcher_get"]
    pub fn get(uid: u32) -> *mut cairo_sys::cairo_surface_t;


    /// Retrieves the surface with the icon, writing the result into `surface`.
    /// Returns whether the query succeeded.
    ///
    /// Accepts a request UID and an out pointer.
    ///
    /// **Semver-exempt and only available with `cfg(rofi_next)`.**
    #[cfg(any(doc, rofi_next))]
    #[link_name = "rofi_icon_fetcher_get_ex"]
    pub fn get_ex(uid: u32, surface: *mut *mut cairo_sys::cairo_surface_t) -> glib_sys::gboolean;

    /// Checks if a file is a supported image (by looking at its extension).
    ///
    /// Returns true if it is an image, false otherwise.
    #[link_name = "rofi_icon_fetcher_file_is_image"]
    pub fn file_is_image(path: *const c_char) -> glib_sys::gboolean;
}
