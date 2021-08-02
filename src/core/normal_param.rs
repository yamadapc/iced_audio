//! Module for the [`NormalParam`] struct
//!
//! [`NormalParam`]: struct.NormalParam.html

use crate::core::Normal;

use std::fmt::Debug;

/// A paramater that contains a normalized `value` and a `default_value`.
///
/// The values are stored as the [`Normal`] type.
///
/// [`Normal`]: ../struct.Normal.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NormalParam {
    /// The value of the parameter represented as a [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub value: Normal,

    /// The default value of the parameter represented as a [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub default: Normal,
}

impl NormalParam {
    /// Create a new param with [`Normal`] as the value and default
    pub fn new(value: Normal) -> Self {
        NormalParam {
            value,
            default: value,
        }
    }
}

impl Default for NormalParam {
    fn default() -> Self {
        Self {
            value: Normal::min(),
            default: Normal::min(),
        }
    }
}

impl From<f32> for NormalParam {
    fn from(value: f32) -> Self {
        let normal = Normal::new(value);
        Self::new(normal)
    }
}
