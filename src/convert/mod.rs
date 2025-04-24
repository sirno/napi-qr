//! Converts a [`crate::QRCode`] to image or SVG you will need to activate associated feature flag

pub mod svg;

use svg::SvgError;

pub mod image;
use image::ImageError;

use crate::{Module, ModuleType};
use napi_derive::napi;

/// Converts a position to a module svg
/// # Example
///
/// For the square shape, the svg is `M{x},{y}h1v1h-1`
///
/// ```rust
/// # use fast_qr::Module;
/// fn square(y: usize, x: usize, _module: Module) -> String {
///     format!("M{x},{y}h1v1h-1")
/// }
/// ```
pub type ModuleFunction = fn(usize, usize, Module) -> String;

/// Different possible Shapes to represent modules in a [`crate::QRCode`]
#[napi(string_enum)]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Shape {
  /// Square Shape
  Square,
  /// Circle Shape
  Circle,
  /// RoundedSquare Shape
  RoundedSquare,
  /// Vertical Shape
  Vertical,
  /// Horizontal Shape
  Horizontal,
  /// Diamond Shape
  Diamond,
}

pub struct ModuleShape {
  shape: Shape,
  scale: f64,
  type: ModuleType,
}

impl From<Shape> for usize {
  fn from(shape: Shape) -> Self {
    match shape {
      Shape::Square => 0,
      Shape::Circle => 1,
      Shape::RoundedSquare => 2,
      Shape::Vertical => 3,
      Shape::Horizontal => 4,
      Shape::Diamond => 5,
    }
  }
}

impl From<String> for Shape {
  #[allow(clippy::match_same_arms)]
  fn from(shape: String) -> Self {
    match shape.to_lowercase().as_str() {
      "square" => Shape::Square,
      "circle" => Shape::Circle,
      "rounded_square" => Shape::RoundedSquare,
      "vertical" => Shape::Vertical,
      "horizontal" => Shape::Horizontal,
      "diamond" => Shape::Diamond,

      _ => Shape::Square,
    }
  }
}

impl From<Shape> for &str {
  fn from(shape: Shape) -> Self {
    match shape {
      Shape::Square => "square",
      Shape::Circle => "circle",
      Shape::RoundedSquare => "rounded_square",
      Shape::Vertical => "vertical",
      Shape::Horizontal => "horizontal",
      Shape::Diamond => "diamond",
    }
  }
}

impl Shape {
  pub fn module_fn(&self, y: usize, x: usize, _: Module) -> String {
    match self {
      Shape::Square => format!("M{x},{y}h1v1h-1"),
      Shape::Circle => format!("M{},{y}.5a.5,.5 0 1,1 0,-.1", x + 1),
      Shape::RoundedSquare => format!("M{x}.2,{y}.2 {x}.8,{y}.2 {x}.8,{y}.8 {x}.2,{y}.8z"),
      Shape::Vertical => format!("M{x}.1,{y}h.8v1h-.8"),
      Shape::Horizontal => format!("M{x},{y}.1h1v.8h-1"),
      Shape::Diamond => format!("M{x}.5,{y}l.5,.5l-.5,.5l-.5,-.5z"),
    }
  }
}

/// Different possible image background shapes
#[napi]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum ImageBackgroundShape {
  /// Square shape
  Square,
  /// Circle shape
  Circle,
  /// Rounded square shape
  RoundedSquare,
}

/// Contains possible errors for a conversion
#[derive(Debug)]
pub enum ConvertError {
  /// Contains error message for a SVG conversion
  Svg(String),
  /// Contains error message for an Image conversion
  Image(String),
  /// Contains error message if a file write failed
  Io(std::io::Error),
}

impl From<SvgError> for ConvertError {
  // TODO: revise this
  fn from(err: SvgError) -> Self {
    match err {
      SvgError::IoError(io_err) => Self::Io(io_err),
      SvgError::SvgError(svg_err) => Self::Svg(svg_err),
    }
  }
}

impl From<ImageError> for ConvertError {
  fn from(err: ImageError) -> Self {
    match err {
      ImageError::EncodingError(image_err) => Self::Image(image_err),
      ImageError::ImageError(image_err) => Self::Image(image_err),
      ImageError::IoError(io_err) => Self::Io(io_err),
    }
  }
}

/// Converts an array of pixel color to it's hexadecimal representation
/// # Example
/// ```rust
/// # use fast_qr::convert::rgba2hex;
/// let color = [0, 0, 0, 255];
/// assert_eq!(&rgba2hex(color), "#000000");
/// ```
#[must_use]
pub fn rgba2hex(color: [u8; 4]) -> String {
  let mut hex = String::with_capacity(9);

  hex.push('#');
  hex.push_str(&format!("{:02x}", color[0]));
  hex.push_str(&format!("{:02x}", color[1]));
  hex.push_str(&format!("{:02x}", color[2]));
  if color[3] != 255 {
    hex.push_str(&format!("{:02x}", color[3]));
  }

  hex
}

/// Allows to take String, string slices, arrays or slices of u8 (3 or 4) to create a [Color]
pub struct Color(pub String);

impl Color {
  /// Returns the contained color
  #[must_use]
  pub fn to_str(&self) -> &str {
    &self.0
  }
}

impl From<String> for Color {
  fn from(color: String) -> Self {
    Self(color)
  }
}

impl From<&str> for Color {
  fn from(color: &str) -> Self {
    Self(color.to_string())
  }
}

impl From<[u8; 4]> for Color {
  fn from(color: [u8; 4]) -> Self {
    Self(rgba2hex(color))
  }
}

impl From<[u8; 3]> for Color {
  fn from(color: [u8; 3]) -> Self {
    Self::from([color[0], color[1], color[2], 255])
  }
}

impl From<&[u8]> for Color {
  fn from(color: &[u8]) -> Self {
    if color.len() == 3 {
      Self::from([color[0], color[1], color[2]])
    } else if color.len() == 4 {
      Self::from([color[0], color[1], color[2], color[3]])
    } else {
      panic!("Invalid color length");
    }
  }
}

impl From<Vec<u8>> for Color {
  fn from(color: Vec<u8>) -> Self {
    Self::from(&color[..])
  }
}

/// Trait for `SvgBuilder` and `ImageBuilder`
pub trait Builder {
  /// Updates margin (default: 4)
  fn margin(&mut self, margin: usize) -> &mut Self;
  /// Updates module color (default: #000000)
  fn module_color<C: Into<Color>>(&mut self, module_color: C) -> &mut Self;
  /// Updates background color (default: #FFFFFF)
  fn background_color<C: Into<Color>>(&mut self, background_color: C) -> &mut Self;
  /// Adds a shape to the shapes list
  fn shape(&mut self, shape: Shape) -> &mut Self;
  /// Add a shape to the shapes list with a specific color
  fn shape_color<C: Into<Color>>(&mut self, shape: Shape, color: C) -> &mut Self;

  // Manages the image part

  /// Provides the image path or an base64 encoded image
  fn image(&mut self, image: String) -> &mut Self;
  /// Updates the image background color (default: #FFFFFF)
  fn image_background_color<C: Into<Color>>(&mut self, image_background_color: C) -> &mut Self;
  /// Updates the image background shape (default: Square)
  fn image_background_shape(&mut self, image_background_shape: ImageBackgroundShape) -> &mut Self;
  /// Updates the image size and the gap between the image and the [`crate::QRCode`]
  /// Default is around 30% of the [`crate::QRCode`] size
  fn image_size(&mut self, image_size: f64) -> &mut Self;
  /// Updates the gap between the image and the [`crate::QRCode`]
  fn image_gap(&mut self, gap: f64) -> &mut Self;
  /// Updates the image position, anchor is the center of the image. Default is the center of the [`crate::QRCode`]
  fn image_position(&mut self, x: f64, y: f64) -> &mut Self;
}
