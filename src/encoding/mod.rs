#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use crate::Error;
#[cfg(any(feature = "serde_json", feature = "serde_yaml"))]
use alloc::string::String;
#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use alloc::vec::Vec;
#[cfg(any(feature = "serde_json", feature = "serde_cbor"))]
use serde::Deserialize;
#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use serde::{de::DeserializeOwned, Serialize};

mod test;
mod api;
pub use api::*;

#[cfg(feature = "serde_json")]
impl<'a, T> TryJsonFrom<T> for String
where
    T: Serialize + Deserialize<'a>,
{
    type Error = Error;
    fn try_json_from(t: T) -> Result<String, Self::Error> {
        Ok(serde_json::to_string(&t)?)
    }
}

#[cfg(feature = "serde_json")]
impl<T> TryJsonInto<T> for String
where
    T: DeserializeOwned,
{
    type Error = Error;
    fn try_json_into(self) -> Result<T, Self::Error> {
        Ok(serde_json::from_str(self.as_str())?)
    }
}

#[cfg(feature = "serde_json")]
impl<'a, T> TryJsonFrom<T> for Vec<u8>
where
    T: Serialize + Deserialize<'a>,
{
    type Error = Error;
    fn try_json_from(t: T) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_json::to_vec(&t)?)
    }
}

#[cfg(feature = "serde_json")]
impl<T> TryJsonInto<T> for Vec<u8>
where
    T: DeserializeOwned,
{
    type Error = Error;
    fn try_json_into(self) -> Result<T, Self::Error> {
        Ok(serde_json::from_slice(self.as_slice())?)
    }
}

#[cfg(feature = "serde_cbor")]
impl<T> TryCborFrom<T> for Vec<u8>
where
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_cbor_from(t: T) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_cbor::to_vec(&t)?)
    }
}

#[cfg(feature = "serde_cbor")]
impl<T> TryCborInto<T> for Vec<u8>
where
    T: DeserializeOwned,
{
    type Error = Error;
    fn try_cbor_into(self) -> Result<T, Self::Error> {
        Ok(serde_cbor::from_slice(self.as_slice())?)
    }
}

#[cfg(feature = "serde_yaml")]
impl<'a, T> TryYamlFrom<T> for String
where
    T: Serialize,
{
    type Error = Error;
    fn try_yaml_from(t: T) -> Result<String, Self::Error> {
        Ok(serde_yaml::to_string(&t)?)
    }
}

#[cfg(feature = "serde_yaml")]
impl<T> TryYamlInto<T> for String
where
    T: DeserializeOwned,
{
    type Error = Error;
    fn try_yaml_into(self) -> Result<T, Self::Error> {
        Ok(serde_yaml::from_str(self.as_str())?)
    }
}

#[cfg(feature = "serde_yaml")]
impl<'a, T> TryYamlFrom<T> for Vec<u8>
where
    T: Serialize + Deserialize<'a>,
{
    type Error = Error;
    fn try_yaml_from(t: T) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_yaml::to_vec(&t)?)
    }
}

#[cfg(feature = "serde_yaml")]
impl<T> TryYamlInto<T> for Vec<u8>
where
    T: DeserializeOwned,
{
    type Error = Error;
    fn try_yaml_into(self) -> Result<T, Self::Error> {
        Ok(serde_yaml::from_slice(self.as_slice())?)
    }
}
