/// `TryJsonFrom` defines a fallible conversion trait which is intended
/// to be implemented on a type which may be converted from a JSON
/// formatted type.
#[cfg(feature = "serde_json")]
pub trait TryJsonFrom<T>
where
    Self: Sized,
{
    type Error;
    fn try_json_from(_: T) -> Result<Self, Self::Error>;
}

/// `TryJsonInto` is defined in order to mirror the TryFrom and TryInto
/// core traits. `TryJsonInto` is automatically implemented for types
/// which implement `TryJsonFrom`.
#[cfg(feature = "serde_json")]
pub trait TryJsonInto<T> {
    type Error;
    fn try_json_into(self) -> Result<T, Self::Error>;
}

/// `TryCborFrom` defines a fallible conversion trait which is intended
/// to be implemented on a type which may be converted from a CBOR
/// formatted type.
#[cfg(feature = "serde_cbor")]
pub trait TryCborFrom<T>
where
    Self: Sized,
{
    type Error;
    fn try_cbor_from(_: T) -> Result<Self, Self::Error>;
}

/// `TryCborInto` is defined in order to mirror the TryFrom and TryInto
/// core traits. `TryCborInto` is automatically implemented for types
/// which implement `TryCborFrom`.
#[cfg(feature = "serde_cbor")]
pub trait TryCborInto<T> {
    type Error;
    fn try_cbor_into(self) -> Result<T, Self::Error>;
}

/// `TryFromYaml` defines a fallible conversion trait which is intended
/// to be implemented on a type which may be converted from a Yaml
/// formatted type.
#[cfg(feature = "serde_yaml")]
pub trait TryYamlFrom<T>
where
    Self: Sized,
{
    type Error;
    fn try_yaml_from(_: T) -> Result<Self, Self::Error>;
}

/// `TryIntoYaml` is defined in order to mirror the TryFrom and TryInto
/// core traits. `TryIntoYaml` is automatically implemented for types
/// which implement `TryFromYaml`.
#[cfg(feature = "serde_yaml")]
pub trait TryYamlInto<T> {
    type Error;
    fn try_yaml_into(self) -> Result<T, Self::Error>;
}
