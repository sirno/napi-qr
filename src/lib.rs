//! # Easy to use fast QRCode generator
//!
//! More examples can be found on [GitHub](https://github.com/erwanvivien/fast_qr/tree/master/examples).
//!
//! ## Converts [`QRCode`] to Unicode
//!
//! ```rust
//! # use fast_qr::convert::ConvertError;
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilder::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let str = qrcode.to_str(); // .print() exists
//! println!("{}", str);
//!
//! #     Ok(())
//! # }
//! ```
//!
//! ## Converts [`QRCode`] to SVG
//!
//! ```rust
//! # use fast_qr::convert::ConvertError;
//! use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilder::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let _svg = SvgBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .to_file(&qrcode, "out.svg");
//! #     std::fs::remove_file("out.svg");
//!
//! #     Ok(())
//! # }
//! ```
//!
//! ## Converts [`QRCode`] to an image
//!
//! ```rust
//! # use fast_qr::convert::ConvertError;
//! use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilder::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let _img = ImageBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .background_color([255, 255, 255, 0]) // transparency
//!     .fit_width(600)
//!     .to_file(&qrcode, "out.png");
//! #     std::fs::remove_file("out.png");
//!
//! #     Ok(())
//! # }
//! ```

use napi_derive::napi;

pub use crate::convert::ModuleStyle;
pub use crate::datamasking::Mask;
pub use crate::ecl::ECL;
pub use crate::encode::Mode;
pub use crate::module::{Module, ModuleType};
pub use crate::qr::{QRBuilder, QRCode};
pub use crate::version::Version;

mod compact;
#[doc(hidden)]
pub mod datamasking;

pub mod convert;
mod default;
mod ecl;
mod encode;
mod hardcode;
mod helpers;
mod module;
mod placement;
mod polynomials;
#[macro_use]
pub mod qr;
mod score;
mod version;

#[cfg(test)]
mod tests;

#[napi]
pub struct SvgOptions {
  style: ModuleStyle,
  module_color: Vec<u8>,
  pub margin: u32,

  pub ecl: Option<ECL>,
  pub version: Option<Version>,

  background_color: Vec<u8>,

  pub image: String,
  image_background_color: Vec<u8>,
  pub image_background_shape: convert::ImageBackgroundShape,
  pub image_size: Vec<f64>,
  pub image_position: Vec<f64>,
}

impl Default for SvgOptions {
  fn default() -> Self {
    Self::new()
  }
}

#[napi]
impl SvgOptions {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      style: ModuleStyle::default(),
      module_color: vec![0, 0, 0, 255],
      margin: 4,

      ecl: None,
      version: None,

      background_color: vec![255, 255, 255, 255],

      image: String::new(),
      image_background_color: vec![255, 255, 255, 255],
      image_background_shape: convert::ImageBackgroundShape::Square,
      image_size: vec![],
      image_position: vec![],
    }
  }

  fn color_to_code(color: String) -> Vec<u8> {
    let mut color = color;
    if color.starts_with('#') {
      color.remove(0);
    }
    let color = color.as_bytes();
    let color = color.chunks_exact(2);
    let color = color.map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap());

    let mut color = color.collect::<Vec<u8>>();
    if color.len() == 3 {
      color.push(255);
    }

    color
  }

  /// Update the module style of the QRCode.
  #[napi(setter)]
  pub fn style(&mut self, module_style: &ModuleStyle) {
    self.style = module_style.clone();
  }

  /// Updates the module color of the QRCode. Tales a string in the format `#RRGGBB[AA]`.
  #[napi(setter)]
  pub fn module_color(&mut self, module_color: String) {
    let code = Self::color_to_code(module_color);
    self.module_color = code;
  }

  /// Updates the background color of the QRCode. Tales a string in the format `#RRGGBB[AA]`.
  #[napi(setter)]
  pub fn background_color(&mut self, background_color: String) {
    let code = Self::color_to_code(background_color);
    self.background_color = code;
  }

  /// Updates the background color of the image. Takes a string in the format `#RRGGBB[AA]`.
  #[napi(setter)]
  pub fn image_background_color(&mut self, image_background_color: String) {
    let code = Self::color_to_code(image_background_color);
    self.image_background_color = code;
  }
}

fn bool_to_u8(qr: QRCode) -> Vec<u8> {
  let dim = qr.size;
  qr.data[..dim * dim]
    .iter()
    .map(|x| u8::from(x.value()))
    .collect()
}

#[napi]
pub fn qr(content: String) -> Vec<u8> {
  let qrcode = QRCode::new(content.as_bytes(), None, None, None, None);
  qrcode.map(bool_to_u8).unwrap_or_default()
}

#[napi]
pub fn qr_svg(content: String, options: &SvgOptions) -> String {
  use crate::convert::svg::SvgBuilder;
  use crate::convert::Builder;
  let qrcode = QRCode::new(content.as_bytes(), options.ecl, options.version, None, None);

  let mut builder = SvgBuilder::default();
  builder.style(options.style.clone());
  builder.margin(options.margin as usize);
  builder.background_color(options.background_color.clone());
  builder.module_color(options.module_color.clone());
  if !options.image.is_empty() {
    builder.image(options.image.clone());
  }

  builder.image_background_color(options.image_background_color.clone());
  builder.image_background_shape(options.image_background_shape);

  if options.image_size.len() == 2 {
    let size = options.image_size[0];
    let gap = options.image_size[1];
    builder.image_size(size);
    builder.image_gap(gap);
  }

  if options.image_size.len() == 2 {
    let x = options.image_position[0];
    let y = options.image_position[1];
    builder.image_position(x, y);
  }

  qrcode
    .map(|qrcode| builder.to_str(&qrcode))
    .unwrap_or_default()
}
