#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
use core::fmt::{Display, Formatter, Result};

mod test;

#[cfg(feature = "serde_cbor")]
static CBOR_ERROR: &str = "Encoding Error: Serializing or deserializing CBOR";
#[cfg(feature = "serde_json")]
static JSON_ERROR: &str = "Encoding Error: Serializing or deserializing JSON";
#[cfg(feature = "serde_yaml")]
static YAML_ERROR: &str = "Encoding Error: Serializing or deserializing YAML";

/// Errors which may occur during serializing and deserializing the graph
/// data structure.
#[derive(PartialEq, Debug)]
pub enum Error {
    #[cfg(feature = "serde_cbor")]
    CborError,
    #[cfg(feature = "serde_json")]
    JsonError,
    #[cfg(feature = "serde_yaml")]
    YamlError,
}

#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            #[cfg(feature = "serde_cbor")]
            Error::CborError => write!(f, "{}", CBOR_ERROR),
            #[cfg(feature = "serde_json")]
            Error::JsonError => write!(f, "{}", JSON_ERROR),
            #[cfg(feature = "serde_yaml")]
            Error::YamlError => write!(f, "{}", YAML_ERROR),
        }
    }
}

#[cfg(feature = "serde_cbor")]
impl From<serde_cbor::Error> for Error {
    fn from(_: serde_cbor::Error) -> Error {
        Error::CborError
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Error {
        Error::JsonError
    }
}

#[cfg(feature = "serde_yaml")]
impl From<serde_yaml::Error> for Error {
    fn from(_: serde_yaml::Error) -> Error {
        Error::YamlError
    }
}
