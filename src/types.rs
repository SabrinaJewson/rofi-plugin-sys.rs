//! Bindings to rofi-types.h

use ::{
    bitflags::bitflags,
    std::{
        ffi::c_void,
        mem,
        os::raw::{c_char, c_double, c_int, c_uint},
    },
};

/// The type of a property.
#[allow(clippy::manual_non_exhaustive)] // count variant is not for that
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum PropertyType {
    Integer,
    Double,
    String,
    Char,
    Boolean,
    Color,
    Image,
    Padding,
    Link,
    Position,
    Highlight,
    List,
    Orientation,
    Cursor,
    Inherit,
    #[doc(hidden)]
    #[non_exhaustive]
    __Count,
}

extern "C" {
    /// This array maps [`PropertyType`] to a user-readable name.
    pub static ProtypeTypeName: [*const c_char; PropertyType::__Count as usize];
}

bitflags! {
    /// Style of text highlight.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct RofiHighlightStyle: c_uint {
        const BOLD = 1;
        const UNDERLINE = 2;
        const STRIKETHROUGH = 16;
        const ITALIC = 4;
        const COLOR = 8;
        const UPPERCASE = 32;
        const LOWERCASE = 64;
        const CAPITALIZE = 128;
    }
}

// Make sure the enum's sizes match
const _: [(); mem::size_of::<RofiHighlightStyle>()] = [(); mem::size_of::<PropertyType>()];

/// Style of line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum RofiLineStyle {
    /// Solid line
    Solid,
    /// Dashed line
    Dash,
}

/// Distance unit type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum RofiPixelUnit {
    /// PixelWidth in pixels.
    Px,
    /// PixelWidth in millimetres.
    Mm,
    /// PixelWidth in EM.
    Em,
    /// PixelWidth in percentage.
    Percent,
    /// PixelWidth in CH.
    Ch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum RofiDistanceModifier {
    None,
    Add,
    Subtract,
    Divide,
    Multiply,
    Modulo,
    Group,
    Min,
    Max,
    Round,
    Floor,
    Ceil,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct RofiDistanceUnit {
    /// Distance
    pub distance: c_double,
    /// Unit type of the distance
    pub unit_type: RofiPixelUnit,

    /// Type
    pub mod_type: RofiDistanceModifier,

    /// Modifier
    pub left: *mut RofiDistanceUnit,

    /// Modifier
    pub right: *mut RofiDistanceUnit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct RofiDistance {
    /// Base
    pub base: RofiDistanceUnit,
    /// Style of the line (optional)
    pub style: RofiLineStyle,
}

/// Type of orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum RofiOrientation {
    Vertical,
    Horizontal,
}

/// Cursor type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum RofiCursorType {
    Default,
    Pointer,
    Text,
}

impl Default for RofiCursorType {
    fn default() -> Self {
        Self::Default
    }
}

/// Represents the color in a theme.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ThemeColor {
    /// Red channel
    pub red: f64,
    /// Green channel
    pub green: f64,
    /// Blue channel
    pub blue: f64,
    /// Alpha channel
    pub alpha: f64,
}

/// Theme image
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum RofiImageType {
    Url,
    LinearGradient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum RofiDirection {
    Left,
    Right,
    Top,
    Bottom,
    Angle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum RofiScaleType {
    None,
    Both,
    Height,
    Width,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct RofiImage {
    pub image_type: RofiImageType,
    pub url: *const c_char,
    pub scaling: RofiScaleType,
    pub wsize: c_int,
    pub hsize: c_int,

    pub dir: RofiDirection,
    pub angle: f64,
    /// Colors
    pub colors: *const glib_sys::GList,

    /// Cached image
    pub surface_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct RofiPadding {
    pub top: RofiDistance,
    pub right: RofiDistance,
    pub bottom: RofiDistance,
    pub left: RofiDistance,
}

/// Theme highlight.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct RofiHighlightColorStyle {
    /// Style to display
    pub style: RofiHighlightStyle,
    pub color: ThemeColor,
}

bitflags! {
    /// Bitflags indicating location or gravity of window.
    ///
    /// ```text
    /// NORTH_WEST      NORTH      NORTH_EAST
    /// EAST            CENTER     EAST
    /// SOUTH_WEST      SOUTH      SOUTH_EAST
    /// ```
    #[repr(transparent)]
    pub struct WindowLocation: c_uint {
        const CENTER = 0;
        /// Top middle
        const NORTH = 1;
        /// Middle right
        const EAST = 2;
        /// Bottom middle
        const SOUTH = 4;
        /// Middle left
        const WEST = 8;
        /// Top left corner.
        const NORTH_WEST = Self::NORTH.bits() | Self::WEST.bits();
        /// Top right corner.
        const NORTH_EAST = Self::NORTH.bits() | Self::EAST.bits();
        /// Bottom right.
        const SOUTH_EAST = Self::SOUTH.bits() | Self::EAST.bits();
        /// Bottom left.
        const SOUTH_WEST = Self::SOUTH.bits() | Self::WEST.bits();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct PropertyLink {
    /// Name
    pub name: *mut c_char,
    /// Cached looked up ref
    pub property_ref: *mut Property,
    /// Property default
    pub def_value: *mut Property,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union PropertyValue {
    /// [`PropertyType::Integer`]
    pub i: i32,
    /// [`PropertyType::Double`]
    pub f: f64,
    /// [`PropertyType::String`]
    pub s: *mut c_char,
    /// [`PropertyType::Char`]
    pub c: c_char,
    /// [`PropertyType::Boolean`]
    pub b: glib_sys::gboolean,
    /// [`PropertyType::Color`]
    pub color: ThemeColor,
    /// [`PropertyType::Padding`]
    pub padding: RofiPadding,
    /// Reference - [`PropertyType::Link`]
    pub link: PropertyLink,
    /// Highlight style - [`PropertyType::Highlight`]
    pub highlight: RofiHighlightColorStyle,
    /// [`PropertyType::Image`]
    pub image: RofiImage,
    /// [`PropertyType::List`]
    pub list: *mut glib_sys::GList,
}

/// Property structure.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Property {
    /// Name of property
    pub name: *const c_char,
    /// Type of property
    pub ty: PropertyType,
    /// Value
    pub value: PropertyValue,
}

/// Structure to hold a range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RofiRangePair {
    pub start: c_int,
    pub stop: c_int,
}

/// Internal structure for matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RofiIntMatcher {
    pub regex: *mut glib_sys::GRegex,
    pub invert: glib_sys::gboolean,
}

/// Structure with data to process by each worker thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ThreadState {
    pub callback: Option<unsafe extern "C" fn(t: *mut ThreadState, data: *mut c_void)>,
}

extern "C" {
    pub static mut tpool: *mut glib_sys::GThreadPool;
}
