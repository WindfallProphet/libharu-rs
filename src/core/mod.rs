pub mod destination;
pub mod document;
pub mod encoder;
pub mod error;
pub mod font;
pub mod image;
pub mod outline;
pub mod page;

pub use destination::{
    Destination
};

pub use document::{
    Document,
    PageLayout,
    PageMode,
    ColorSpace,
    InfoType,
    EncryptMode,
    CompressionMode,
};

pub use encoder::{
    Encoder,
};

pub use error::{
    Error,
    Result,
};

pub use font::{
    Font
};
pub use image::{
    Image
};
pub use outline::{
    Outline
};
pub use page::{
    LineCap,
    LineJoin,
    TextRenderingMode,
    PageSize,
    PageDirection,
    TextAlignment,
    Page,
    PageDescTextCommonFunction,
    PageDescTextCommonFunctionCStr,
    PageDescPathCommonFunction,
    PageDescriptionMode,
};
pub type Real = libharu_sys::HPDF_REAL;

/// RGB color type.
#[derive(Debug, Clone)]
pub struct Color {
    /// red (0.0 ~ 1.0)
    pub red: Real,

    /// green (0.0 ~ 1.0)
    pub green: Real,

    /// blue (0.0 ~ 1.0)
    pub blue: Real,
}

impl Copy for Color {}

impl From<(Real, Real, Real)> for Color {
    fn from(v: (Real, Real, Real)) -> Self {
        Self { red: v.0, green: v.1, blue: v.2 }
    }
}

/// CMYK color type
#[derive(Debug, Clone)]
pub struct CmykColor {
    /// cyan (0.0 ~ 1.0)
    pub cyan: Real,

    /// magenta (0.0 ~ 1.0)
    pub magenta: Real,
    
    /// yellow (0.0 ~ 1.0)
    pub yellow: Real,
    
    /// keyplate (0.0 ~ 1.0)
    pub keyplate: Real,
}

impl Copy for CmykColor {}

impl From<(Real, Real, Real, Real)> for CmykColor {
    fn from(v: (Real, Real, Real, Real)) -> Self {
        Self { cyan: v.0, magenta: v.1, yellow: v.2, keyplate: v.3 }
    }
}
/// Point
#[derive(Debug, Clone)]
pub struct Point {
    /// x
    pub x: Real,

    /// y
    pub y: Real,
}

impl Copy for Point {}

impl From<(Real, Real)> for Point {
    fn from(v: (Real, Real)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

/// Rect
#[derive(Debug, Clone)]
pub struct Rect {
    /// Left position
    pub left: Real,

    /// Top position
    pub top: Real,
    
    /// Right position
    pub right: Real,

    /// Bottom position
    pub bottom: Real,
}

impl Copy for Rect {}

impl From<(Real, Real, Real, Real)> for Rect {
    fn from(v: (Real, Real, Real, Real)) -> Self {
        Self { left: v.0, top: v.1, right: v.2, bottom: v.3 }
    }
}
