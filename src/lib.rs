#![no_std]
extern crate alloc;

/// `encoding` module contains the definition of the encoding API and auto implementations.
#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
mod encoding;
#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
pub use encoding::*;

/// `error` module contains the definition of the Error struct.
mod error;
pub use error::Error;

#[cfg(feature = "serde_cbor")]
pub use serde_cbor;
#[cfg(feature = "serde_json")]
pub use serde_json;
#[cfg(feature = "serde_yaml")]
pub use serde_yaml;
