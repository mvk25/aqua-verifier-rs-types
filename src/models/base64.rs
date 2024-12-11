//! # Base 64 encoded data

#[derive(Debug, Clone)]
/// A wrapper type for a `Vec<u8>` that represents Base64-encoded data
/// 
/// The `Base64` struct is designed to encapsulate a vector of bytes
/// and provide addition functionality, such as implementing traits
/// for interoperability and convenience
pub struct Base64(Vec<u8>);

/// Implements the `AsRef<[u8]> trait for `Base64`. Using the
/// AsRef trait to get a reference to the inner byte slice.
impl AsRef<[u8]> for Base64 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Allows `Base64` to be dereferenced into a slice of bytes(`[u8]`)
impl std::ops::Deref for Base64 {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Conversion from a vector of bytes into the `Base64` struct`.
impl From<Vec<u8>> for Base64 {
    fn from(value: Vec<u8>) -> Self {
        Base64(value)
    }
}

// Into<Base64> for Vec<u8> (blanket impl)
/// Conversion from a `Base64` Struct to a Vector of `u8`.
impl From<Base64> for Vec<u8> {
    fn from(value: Base64) -> Self {
        value.0
    }
}

// todo: needed?

// impl TryFrom<&str> for Base64 {
//     type Error = Error;

//     fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
//         let Ok(vec) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, value) else {
//             return Err(Error);
//         };
//         Ok(Base64::from(vec))
//     }
// }

impl std::fmt::Display for Base64 {
    /// Formats the bytes as a base64 string with the standard encoding.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = base64::display::Base64Display::new(
            &self.0,
            &base64::engine::general_purpose::STANDARD,
        );
        disp.fmt(f)
    }
}

impl std::str::FromStr for Base64 {
    //todo: err
    type Err = ();

    /// Attempts to decode a base64 string into bytes
    /// 
    /// # Errors
    /// Returns `Err(())` if the input is not a valid base64
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Ok(vec) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, value)
        else {
            return Err(());
        };

        Ok(Base64::from(vec))
    }
}

impl<'de> serde::Deserialize<'de> for Base64 {
    /// Deserialized a Base64 encoded string into a `Base64` struct.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom("Invalid Base64"))
    }
}

impl serde::Serialize for Base64 {
    /// Serialized the `Base64` struct as a Base64-encoded string.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

#[test]
fn read_base64() {
    const TEST_DATA: &str = "TmV2ZXIgZ29ubmEgZ2l2ZSB5b3UgdXAsbmV2ZXIgZ29ubmEgbGV0IHlvdSBkb3duIQ==";
    let _base64_str: Base64 = TEST_DATA.parse().expect("Base 64 cannot read.");
    //dbg!(_base64_str);
    const TEST_DATA_WITH_WHITESPACE: &str =
        "TmV2 ZXIg Z29u bmEg Z2l2 ZSB5 b3Ug dXAs bmV2 ZXIg Z29u bmEg bGV0 IHlv dSBk b3du IQ==";
    <Base64 as std::str::FromStr>::from_str(TEST_DATA_WITH_WHITESPACE)
        .expect_err("Whitespace was wrongfully accepted.");
}

#[test]
fn test_write() {
    const TEST_DATA: &str = "TmV2ZXIgZ29ubmEgZ2l2ZSB5b3UgdXAsbmV2ZXIgZ29ubmEgbGV0IHlvdSBkb3duIQ==";
    let base64_thing: Base64 = TEST_DATA.parse().expect("Correct B64 Data not read.");
    assert_eq!(TEST_DATA, &base64_thing.to_string(), "stuff broke");
}
