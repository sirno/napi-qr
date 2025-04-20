//! Contains all different levels of quality.
//! And allows to find easily max bits per version/quality pair

#![deny(unsafe_code)]
#![warn(missing_docs)]

use napi_derive::napi;
use std::fmt::Write;

/// Error Correction Coding has 4 levels
#[napi]
#[derive(Debug)]
#[allow(dead_code)]
// TODO: maybe export this?
pub enum ECL {
  /// Low, 7%
  L,
  /// Medium, 15%
  M,
  /// Quartile, 25%
  Q,
  /// High, 30%
  H,
}

impl core::fmt::Display for ECL {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      ECL::L => f.write_char('L'),
      ECL::M => f.write_char('M'),
      ECL::Q => f.write_char('Q'),
      ECL::H => f.write_char('H'),
    }
  }
}
