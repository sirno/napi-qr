//! Converts [`QRCode`] to SVG
//!
//! ```rust
//! use fast_qr::convert::ConvertError;
//! use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilde::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let _svg = SvgBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .to_file(&qrcode, "out.svg");
//!
//! #     std::fs::remove_file("out.svg");
//! #     Ok(())
//! # }
//! ```

use crate::{QRCode, Version};

use super::{Builder, Color, ImageBackgroundShape, ModuleStyle, Shape};

/// Builder for svg, can set shape, margin, background_color, dot_color
pub struct SvgBuilder {
  /// Styles vector allows to set multiple style definitions
  styles: Vec<ModuleStyle>,
  /// The margin for the svg, default is 4
  margin: usize,
  /// The background color for the svg, default is #FFFFFF
  background_color: Color,
  /// The color for each module, default is #000000
  dot_color: Color,

  // Image Embedding
  /// Image to embed in the svg, can be a path or a base64 string
  image: Option<String>,
  /// Background color for the image, default is #FFFFFF
  image_background_color: Color,
  /// Background shape for the image, default is square
  image_background_shape: ImageBackgroundShape,
  /// Size of the image (in module size), default is ~1/3 of the svg
  image_size: Option<f64>,
  /// Gap between the image and the border (in module size), default is calculated
  image_gap: Option<f64>,
  /// Position of the image, default is center
  image_position: Option<(f64, f64)>,
}

#[derive(Debug)]
/// Possible errors when converting to SVG
pub enum SvgError {
  /// Error while writing file
  IoError(std::io::Error),
  /// Error while creating svg
  SvgError(String),
}

/// Creates a Builder instance
impl Default for SvgBuilder {
  fn default() -> Self {
    SvgBuilder {
      background_color: [255; 4].into(),
      dot_color: [0, 0, 0, 255].into(),
      margin: 4,
      styles: Vec::new(),

      // Image Embedding
      image: None,
      image_background_color: [255; 4].into(),
      image_background_shape: ImageBackgroundShape::Square,
      image_size: None,
      image_gap: None,
      image_position: None,
    }
  }
}

impl Builder for SvgBuilder {
  fn margin(&mut self, margin: usize) -> &mut Self {
    self.margin = margin;
    self
  }

  fn module_color<C: Into<Color>>(&mut self, dot_color: C) -> &mut Self {
    self.dot_color = dot_color.into();
    self
  }

  fn background_color<C: Into<Color>>(&mut self, background_color: C) -> &mut Self {
    self.background_color = background_color.into();
    self
  }

  fn style(&mut self, style: ModuleStyle) -> &mut Self {
    self.styles.push(style);
    self
  }

  fn image(&mut self, image: String) -> &mut Self {
    self.image = Some(image);
    self
  }

  fn image_background_color<C: Into<Color>>(&mut self, image_background_color: C) -> &mut Self {
    self.image_background_color = image_background_color.into();
    self
  }

  fn image_background_shape(&mut self, image_background_shape: ImageBackgroundShape) -> &mut Self {
    self.image_background_shape = image_background_shape;
    self
  }

  fn image_size(&mut self, image_size: f64) -> &mut Self {
    self.image_size = Some(image_size);
    self
  }

  fn image_gap(&mut self, gap: f64) -> &mut Self {
    self.image_gap = Some(gap);
    self
  }

  fn image_position(&mut self, x: f64, y: f64) -> &mut Self {
    self.image_position = Some((x, y));
    self
  }
}

impl SvgBuilder {
  fn image_placement(image_background_shape: ImageBackgroundShape, n: usize) -> (f64, f64) {
    use ImageBackgroundShape::{Circle, RoundedSquare, Square};

    #[rustfmt::skip]
        const SQUARE: [f64; 40] = [
            5f64,   9f64,  9f64, 11f64, 13f64,
            13f64, 15f64, 17f64, 17f64, 19f64,
            21f64, 21f64, 23f64, 25f64, 25f64,
            27f64, 29f64, 29f64, 31f64, 33f64,
            33f64, 35f64, 37f64, 37f64, 39f64,
            41f64, 41f64, 43f64, 45f64, 45f64,
            47f64, 49f64, 49f64, 51f64, 53f64,
            53f64, 55f64, 57f64, 57f64, 59f64,
        ];
    const ROUNDED_SQUARE: [f64; 40] = SQUARE;
    const CIRCLE: [f64; 40] = SQUARE;

    // Using hardcoded values
    let version = Version::from(n) as usize;
    let border_size = match image_background_shape {
      Square => SQUARE[version],
      RoundedSquare => ROUNDED_SQUARE[version],
      Circle => CIRCLE[version],
    };

    // Allows for a module gap between the image and the border
    let gap = match image_background_shape {
      Square | RoundedSquare => 2f64,
      Circle => 3f64,
    };
    // Make the image border bigger for bigger versions
    let gap = gap * (version + 10) as f64 / 10f64;
    (border_size, (border_size - gap).round())
  }

  fn precompute_image_placement(&self, n: usize) -> Option<(Vec<bool>, f64, f64, f64, f64)> {
    // Precompute the image placement
    let _image = self.image.as_ref()?;

    let (mut border_size, mut image_size) = Self::image_placement(self.image_background_shape, n);

    if let Some(override_size) = self.image_size {
      dbg!(override_size);
      let gap = -(image_size - border_size);
      border_size = override_size + gap;
      image_size = override_size;
    }

    if let Some(override_gap) = self.image_gap {
      border_size = image_size + override_gap * 2f64;
    }

    let mut placed_coord_x = (self.margin * 2 + n) as f64 - border_size;

    // Adjust for non-integer initial x coordinates so as not to partially cover bits by rounding down.
    if placed_coord_x % 2f64 != 0f64 {
      placed_coord_x += 1f64;
      border_size -= 1f64;
    }

    placed_coord_x /= 2f64;

    let mut placed_coord = (placed_coord_x, placed_coord_x);

    if let Some((x, y)) = self.image_position {
      placed_coord = (x - border_size / 2f64, y - border_size / 2f64);
    }

    // Create module mask
    let mut mask = vec![false; n * n];

    dbg!(border_size, image_size, placed_coord);

    // Calculate image boundaries once
    let left = ((n as f64 - border_size) / 2.0) as usize;
    let right = ((n as f64 + border_size) / 2.0) as usize;
    let top = left; // Same calculation if square
    let bottom = right; // Same calculation if square

    dbg!(n, left, right, top, bottom, placed_coord, image_size);

    for y in 0..n {
      for x in 0..n {
        let index = y * n + x;
        mask[index] = x >= left && x < right && y >= top && y < bottom;
      }
    }

    Some((mask, placed_coord.0, placed_coord.1, border_size, image_size))
  }

  fn image(&self, coord_x: f64, coord_y: f64, border_size: f64, image_size: f64) -> String {
    if self.image.is_none() {
      return String::new();
    }

    let image = self.image.as_ref().unwrap();
    let mut out = String::with_capacity(image.len() + 100);

    let format = match self.image_background_shape {
      ImageBackgroundShape::Square => {
        r#"<rect x="{0}" y="{1}" width="{2}" height="{2}" fill="{3}"/>"#
      }
      ImageBackgroundShape::Circle => {
        r#"<rect x="{0}" y="{1}" width="{2}" height="{2}" fill="{3}" rx="1000px"/>"#
      }
      ImageBackgroundShape::RoundedSquare => {
        r#"<rect x="{0}" y="{1}" width="{2}" height="{2}" fill="{3}" rx="1px"/>"#
      }
    };

    let format = format
      .replace("{0}", &coord_x.to_string())
      .replace("{1}", &coord_y.to_string())
      .replace("{2}", &border_size.to_string())
      .replace("{3}", self.image_background_color.to_str());

    out.push_str(&format);

    out.push_str(&format!(
      r#"<image x="{0:.2}" y="{1:.2}" width="{2:.2}" height="{2:.2}" href="{3}" />"#,
      coord_x + (border_size - image_size) / 2f64,
      coord_y + (border_size - image_size) / 2f64,
      image_size,
      image
    ));

    out
  }

  /// Return the neighborhood of a module
  ///
  /// The neighborhood should be encoded as follows:
  ///
  /// 0b00000001: top left
  /// 0b00000010: top
  /// 0b00000100: top right
  /// 0b00001000: right
  /// 0b00010000: bottom right
  /// 0b00100000: bottom
  /// 0b01000000: bottom left
  /// 0b10000000: left
  ///
  fn get_neighborhood(&self, qr: &QRCode, mask: &[bool], x: usize, y: usize) -> Neighborhood {
    let mut neighbors = 0u8;

    if x > 0 && y > 0 && qr[y - 1][x - 1].value() && !mask[(y - 1) * qr.size + (x - 1)] {
      neighbors |= 0b00000001;
    }
    if y > 0 && qr[y - 1][x].value() && !mask[(y - 1) * qr.size + x] {
      neighbors |= 0b00000010;
    }
    if x < qr.size - 1 && y > 0 && qr[y - 1][x + 1].value() && !mask[(y - 1) * qr.size + (x + 1)] {
      neighbors |= 0b00000100;
    }
    if x < qr.size - 1 && qr[y][x + 1].value() && !mask[y * qr.size + (x + 1)] {
      neighbors |= 0b00001000;
    }
    if x < qr.size - 1
      && y < qr.size - 1
      && qr[y + 1][x + 1].value()
      && !mask[(y + 1) * qr.size + (x + 1)]
    {
      neighbors |= 0b00010000;
    }
    if y < qr.size - 1 && qr[y + 1][x].value() && !mask[(y + 1) * qr.size + x] {
      neighbors |= 0b00100000;
    }
    if x > 0 && y < qr.size - 1 && qr[y + 1][x - 1].value() && !mask[(y + 1) * qr.size + (x - 1)] {
      neighbors |= 0b01000000;
    }
    if x > 0 && qr[y][x - 1].value() && !mask[y * qr.size + (x - 1)] {
      neighbors |= 0b10000000;
    }

    Neighborhood(neighbors)
  }

  fn path(&self, qr: &QRCode, mask: &[bool]) -> String {
    const DEFAULT_COMMAND: [ModuleStyle; 1] = [ModuleStyle::default()];

    // TODO: cleanup this basic logic
    let styles: &[ModuleStyle] = if !self.styles.is_empty() {
      &self.styles
    } else {
      &DEFAULT_COMMAND
    };

    let mut paths = vec![String::with_capacity(20 * qr.size * qr.size); styles.len()];
    for path in paths.iter_mut() {
      path.push_str(r#"<path d=""#);
    }
    for y in 0..qr.size {
      let line = &qr[y];
      for (x, &cell) in line.iter().enumerate() {
        if !cell.value() || mask[y * qr.size + x] {
          continue;
        }
        let neighbors = self.get_neighborhood(qr, mask, x, y);

        for (i, style) in styles.iter().enumerate() {
          paths[i].push_str(&style.module_fn(y + self.margin, x + self.margin, cell, &neighbors));
        }
      }
    }

    for (i, style) in styles.iter().enumerate() {
      let style_color = style.get_color().as_ref().unwrap_or(&self.dot_color);
      // Allows to compare if two function pointers are the same
      // This works because there is no notion of Generics for `rounded_square`
      if let Shape::RoundedSquare = style.shape {
        paths[i].push_str(&format!(
          r##"" stroke-width=".3" stroke-linejoin="round" stroke="{}"##,
          style_color.to_str()
        ));
      }

      paths[i].push_str(&format!(r#"" fill="{}"/>"#, style_color.to_str()));
    }

    paths.join("")
  }

  /// Return a string containing the svg for a qr code
  pub fn to_str(&self, qr: &QRCode) -> String {
    let n = qr.size;

    let image_placement = self.precompute_image_placement(n);

    let mut out = String::with_capacity(11 * n * n / 2);
    out.push_str(&format!(
      r#"<svg viewBox="0 0 {0} {0}" xmlns="http://www.w3.org/2000/svg">"#,
      self.margin * 2 + n
    ));

    out.push_str(&format!(
      r#"<rect width="{0}px" height="{0}px" fill="{1}"/>"#,
      self.margin * 2 + n,
      self.background_color.to_str()
    ));

    if let Some((mask, coord_x, coord_y, border_size, image_size)) = image_placement {
      out.push_str(&self.path(qr, &mask));
      out.push_str(&self.image(coord_x, coord_y, border_size, image_size));
    } else {
      out.push_str(&self.path(qr, &vec![false; n * n]));
    }

    out.push_str("</svg>");
    out
  }

  /// Returns the svg for in a buffer
  pub fn as_bytes(&self, qr: &QRCode) -> Vec<u8> {
    let svg = self.to_str(qr);
    svg.into_bytes()
  }

  /// Saves the svg for a qr code to a file
  pub fn to_file(&self, qr: &QRCode, file: &str) -> Result<(), SvgError> {
    use std::fs::File;
    use std::io::Write;

    let out = self.to_str(qr);

    let mut f = File::create(file).map_err(SvgError::IoError)?;
    f.write_all(out.as_bytes()).map_err(SvgError::IoError)?;

    Ok(())
  }
}

pub enum Position {
  TopLeft,
  Top,
  TopRight,
  Right,
  BottomRight,
  Bottom,
  BottomLeft,
  Left,
}

/// Neighborhood of a module
///
/// 0b00000001: top left
/// 0b00000010: top
/// 0b00000100: top right
/// 0b00001000: right
/// 0b00010000: bottom right
/// 0b00100000: bottom
/// 0b01000000: bottom left
/// 0b10000000: left
pub struct Neighborhood(u8);

impl Neighborhood {
  pub fn get(&self, position: Position) -> bool {
    (self.0 & (1 << (position as u8))) != 0
  }

  pub fn mask(&self, mask: u8) -> u8 {
    self.0 & mask
  }
}
