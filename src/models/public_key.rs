//! Provides a wrapper around `libsecp256k1::PublicKey`, offering additional methods for serialization, deserialization, and cryptographic transformations.


use sha3::Digest;
use std::ops::Deref;

use crate::{
    crypt,
    models::stack_str::{from_hex, StackStr},
};

/// A wrapper for `libsecp256k1::PublickKey` with additional methods
/// for serialization, deserialization and cryptographic transformations.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PublicKey(libsecp256k1::PublicKey);

impl std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_stackstr()[..])
    }
}

impl PublicKey {
    /// Converts the `PublicKey` into a hexadecimal string prefixed with 0x.
    /// The output uses `StackStr`` for efficient storage and manipulation.
    pub fn to_stackstr(self) -> StackStr<{ 2 + 2 * 65 }> {
        let mut s = [0u8; 2 + 2 * 65];
        s[0] = b'0';
        s[1] = b'x';
        let arr: [u8; 65] = self.into();
        // Safety: This will never error as it has exactly enough space in the buffer
        unsafe {
            hex::encode_to_slice(arr, &mut s[2..]).unwrap_unchecked();
        }
        StackStr::new(s)
    }
}

/// Allows wrapping a `libsecp256k1::PublicKey` into a `PublicKey`.
impl From<libsecp256k1::PublicKey> for PublicKey {
    fn from(value: libsecp256k1::PublicKey) -> Self {
        PublicKey(value)
    }
}

/// Converts the `PublicKey` into its serialized byte 
/// representation (uncompressed format, 65 bytes).
impl From<PublicKey> for [u8; 65] {
    fn from(value: PublicKey) -> Self {
        value.0.serialize()
    }
}

/// Derives the Ethereum address by hashing the public key using Keccak-256 
/// (excluding the first byte) and taking the last 20 bytes.
impl From<PublicKey> for ethaddr::Address {
    fn from(value: PublicKey) -> Self {
        // use crate::prelude::*;
        let mut hasher = crypt::Keccak256::default();
        hasher.update(&<[u8; 65]>::from(value)[1..]);
        let bytes32: [u8; 32] = hasher.finalize().into();

        ethaddr::Address(bytes32[12..].try_into().unwrap())
    }
}

/// Tries to parse a 65-byte array into a valid `PublicKey`.
impl TryFrom<[u8; 65]> for PublicKey {
    type Error = libsecp256k1::Error;

    fn try_from(value: [u8; 65]) -> Result<Self, Self::Error> {
        libsecp256k1::PublicKey::parse(&value).map(Self)
    }
}

/// Implements the `std::ops::Deref` trait for `PublicKey`.
/// This allows `PublicKey` to be treated as a reference to `libsecp256k1::PublicKey`.
impl Deref for PublicKey {
    /// The target type that `PublicKey` dereferences to.
    type Target = libsecp256k1::PublicKey;

    /// Dereferences `PublicKey` to access the inner `libsecp256k1::PublicKey`.
    ///
    /// # Returns
    /// A reference to the inner `libsecp256k1::PublicKey`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implements the `std::str::FromStr` trait for `PublicKey`.
/// This allows a `PublicKey` to be parsed from a string.
impl std::str::FromStr for PublicKey {
    /// The error type returned when parsing fails.
    type Err = ();

    /// Parses a `PublicKey` from a hexadecimal string.
    ///
    /// # Parameters
    /// - `s`: The string slice to parse.
    ///
    /// # Returns
    /// - `Ok(PublicKey)` if the string is successfully parsed.
    /// - `Err(())` if the string is invalid.
    ///
    /// # Errors
    /// - Returns an error if the string contains uppercase characters.
    /// - Returns an error if the string does not start with "0x".
    /// - Returns an error if the string cannot be parsed as a valid `PublicKey`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_ascii_lowercase() != s {
            return Err(());
        }
        let s = s.strip_prefix("0x").ok_or(())?;
        let h = from_hex(s).ok_or(())?;
        h.try_into().map_err(|_| ())
    }
}



/// Implements the `std::fmt::Display` trait for `PublicKey`.
/// This allows a `PublicKey` to be formatted as a string.
impl std::fmt::Display for PublicKey {
    /// Formats the `PublicKey` as a string using its stack string representation.
    ///
    /// # Parameters
    /// - `f`: The formatter instance.
    ///
    /// # Returns
    /// - `Ok` if formatting succeeds.
    /// - `Err` if formatting fails.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_stackstr().fmt(f)
    }
}

/// Implements `serde::Deserialize` for `PublicKey`.
/// This allows a `PublicKey` to be deserialized from a string representation.
impl<'de> serde::Deserialize<'de> for PublicKey {
    /// Deserializes a `PublicKey` from a string.
    ///
    /// # Parameters
    /// - `deserializer`: The deserializer instance.
    ///
    /// # Returns
    /// - `Ok(PublicKey)` if the string is successfully parsed.
    /// - `Err(D::Error)` if the string is invalid.
    ///
    /// # Errors
    /// Returns a custom error if the string is not a valid public key or is unsupported.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom("not a valid signature (or maybe not supported)"))
    }
}


/// Implements `serde::Serialize` for `PublicKey`.
/// This allows a `PublicKey` to be serialized as a string.
impl serde::Serialize for PublicKey {
    /// Serializes the `PublicKey` into its stack string representation.
    ///
    /// # Parameters
    /// - `serializer`: The serializer instance.
    ///
    /// # Returns
    /// - `Ok(S::Ok)` if serialization succeeds.
    /// - `Err(S::Error)` if serialization fails.
    ///
    /// # Example
    /// Converts the `PublicKey` to a stack string and serializes it.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_stackstr().as_ref())
    }
}

#[test]
fn test_read() {
    const TEST_DATA: &str =          "0x04062274ed5bba92b9ab6b8687a86d87066d3dbac83e4f7e0e996a4d163e1bb294a75d8bbef8c9b2425bf7c020c7fe298580bc37fe8562227cb50e574dabb79701";
    let _encoded_str: PublicKey =
        std::str::FromStr::from_str(TEST_DATA).expect("Correct public key not read.");
    //dbg!(_encoded_str);
    const TEST_DATA_TOO_LONG: &str = "0x04062274ed5bba92b9ab6b8687a86d87066d3dbac83e4f7e0e996a0484d163e1bb294a75d8bbef8c9b2425bf7c020c7fe298580bc37fe8562227cb50e574dabb79701";
    <PublicKey as std::str::FromStr>::from_str(TEST_DATA_TOO_LONG)
        .expect_err("accepted overly long public key");
    const TEST_DATA_NOPREFIX: &str = "04062274ed5bba92b9ab6b8687a86d87066d3dbac83e4f7e0e996a4d163e1bb294a75d8bbef8c9b2425bf7c020c7fe298580bc37fe8562227cb50e574dabb79701";
    <PublicKey as std::str::FromStr>::from_str(TEST_DATA_NOPREFIX)
        .expect_err("accepted public key without 0x prefix.");
    const TEST_DATA_WITH_UPPER: &str = "0x04062274ed5bba92b9Ab6b8687a86d87066d3dbac83e4f7e0e996a4d163e1bB294a75d8bBef8c9b2425bf7c020c7Fe298580bc37fe8562227cb50e574dabb79701";
    <PublicKey as std::str::FromStr>::from_str(TEST_DATA_WITH_UPPER)
        .expect_err("accepted public key with Uppercase Letters.");
}

#[test]
fn test_write() {
    const TEST_DATA: &str = "0x04062274ed5bba92b9ab6b8687a86d87066d3dbac83e4f7e0e996a4d163e1bb294a75d8bbef8c9b2425bf7c020c7fe298580bc37fe8562227cb50e574dabb79701";
    let pubkey_thing: PublicKey = TEST_DATA.parse().expect("Correct public key not read.");
    assert_eq!(TEST_DATA, &*pubkey_thing.to_stackstr(), "stuff broke");
}
