//! This module handles cryptographic signatures, including the `RevisionSignature` and `Signature` structs, as well as the `ReadError` enum for error handling during signature parsing and processing.


use ethaddr::Address;

use crate::models::stack_str::{StackStr, from_hex};
use crate::models::hash::Hash;

use super::public_key::PublicKey;

/// Represents an ECDSA secp256k1 signature used for signing Aqua-Chain transactions.
/// 
/// This structure includes:
/// - `recovery_id`: The recovery ID, which is required to reconstruct the public key from the signature.
/// - `signature`: The cryptographic signature itself.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    pub recovery_id: libsecp256k1::RecoveryId,
    pub signature: libsecp256k1::Signature,
}

/// Implements the `std::fmt::Debug` trait for `Signature`.
/// 
/// Formats the `Signature` as a hexadecimal stack string prefixed with `0x`.
impl std::fmt::Debug for Signature {
    /// # Parameters
    /// - `f`: The formatter instance.
    /// 
    /// # Returns
    /// - `Ok` if formatting succeeds.
    /// - `Err` if formatting fails.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_stackstr()[..])
    }
}

impl Signature {
    /// Converts the `Signature` into a stack-allocated hexadecimal string.
    ///
    /// # Returns
    /// - A `StackStr` representing the signature in a `0x`-prefixed hexadecimal format.
    ///
    /// # Example
    /// If the signature is valid, it will be serialized into a 65-byte array and converted to a hex string.
    pub fn to_stackstr(self) -> StackStr<{ 2 + 2 * 65 }> {
        let mut s = [0u8; 2 + 2 * 65];
        s[0] = b'0';
        s[1] = b'x';
        let arr: [u8; 65] = self.into();
        // Safety: This will never error as it has exactly enough space in the buffer.
        unsafe {
            hex::encode_to_slice(arr, &mut s[2..]).unwrap_unchecked();
        }
        StackStr::new(s)
    }
}

/// Implements the `From` trait to convert a `(libsecp256k1::Signature, libsecp256k1::RecoveryId)`
/// tuple into a `Signature`.
impl From<(libsecp256k1::Signature, libsecp256k1::RecoveryId)> for Signature {
    /// Converts the tuple into a `Signature`.
    ///
    /// # Parameters
    /// - `value`: A tuple containing the signature and recovery ID.
    ///
    /// # Returns
    /// - A `Signature` struct containing the provided values.
    fn from(value: (libsecp256k1::Signature, libsecp256k1::RecoveryId)) -> Self {
        Signature { recovery_id: value.1, signature: value.0 }
    }
}

/// Represents the encoded form of a `Signature` with a recovery ID.
/// 
/// This is used for conversion between `Signature` and byte arrays.
#[repr(C)]
struct EncSignature {
    signature: [u8; 64],
    recovery_id: u8,
}

/// Implements the `From` trait to convert a `Signature` into a `[u8; 65]` byte array.
impl From<Signature> for [u8; 65] {
    /// Converts the `Signature` into its byte representation.
    ///
    /// # Parameters
    /// - `value`: The `Signature` to convert.
    ///
    /// # Returns
    /// - A 65-byte array containing the serialized signature and recovery ID.
    ///
    /// # Note
    /// The recovery ID is incremented by 27 as a magic number (common convention).
    fn from(value: Signature) -> Self {
        let enc_sign = EncSignature {
            signature: value.signature.serialize(),
            recovery_id: value.recovery_id.serialize() + 27, // Magic number, consult ducks.
        };
        unsafe { std::mem::transmute(enc_sign) }
    }
}

/// Implements the `TryFrom` trait to convert a `[u8; 65]` byte array into a `Signature`.
impl TryFrom<[u8; 65]> for Signature {
    type Error = libsecp256k1::Error;

    /// Attempts to parse a 65-byte array into a `Signature`.
    ///
    /// # Parameters
    /// - `value`: The byte array to parse.
    ///
    /// # Returns
    /// - `Ok(Signature)` if the byte array is successfully parsed.
    /// - `Err(libsecp256k1::Error)` if parsing fails.
    ///
    /// # Errors
    /// - Returns an error if the signature or recovery ID is invalid.
    fn try_from(value: [u8; 65]) -> Result<Self, Self::Error> {
        let enc_sign: EncSignature = unsafe { std::mem::transmute(value) };
        Ok(Signature {
            signature: libsecp256k1::Signature::parse_standard(&enc_sign.signature)?,
            recovery_id: libsecp256k1::RecoveryId::parse_rpc(enc_sign.recovery_id)?,
        })
    }
}


// impl Default for Signature {
//     fn default() -> Self {
//         Self {
//             recovery_id: libsecp256k1::RecoveryId::parse(0).unwrap(),
//             signature: libsecp256k1::Signature::parse_overflowing(&[0u8; 64]),
//         }
//     }
// }


/// Error types for parsing or handling `Signature`.
///
/// This enumeration represents the different kinds of errors that can occur
/// during parsing or handling of a `Signature`.
#[derive(thiserror::Error, Debug)]
pub enum ReadError {
    /// Error when the input string is not in ASCII lowercase.
    #[error("ascii or smth idk")]
    NotAsciiLower,

    /// Error when the string is missing the required `0x` prefix.
    #[error("goddamit WITH prefix")]
    NoPrefix,

    /// Error when the input contains non-hexadecimal characters.
    #[error("stay with HEX inputs only")]
    NotHex,

    /// Error when `libsecp256k1` fails to parse or handle the input.
    #[error("libsecp256k1: {0}")]
    DecryptFail(#[from] libsecp256k1::Error),
}

/// Implements the `FromStr` trait to parse a `Signature` from a string.
impl std::str::FromStr for Signature {
    type Err = ReadError;

    /// Parses a `Signature` from a given string.
    ///
    /// # Parameters
    /// - `s`: The input string, which must:
    ///   - Be in lowercase ASCII.
    ///   - Start with the `0x` prefix.
    ///   - Contain valid hexadecimal characters.
    ///
    /// # Returns
    /// - `Ok(Signature)` if the parsing succeeds.
    /// - `Err(ReadError)` if the string is invalid or parsing fails.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_ascii_lowercase() != s {
            return Err(ReadError::NotAsciiLower);
        }
        let s = s.strip_prefix("0x").ok_or(ReadError::NoPrefix)?;
        let h = from_hex(s).ok_or(ReadError::NotHex)?;
        h.try_into().map_err(ReadError::DecryptFail)
    }
}

/// Implements the `Deserialize` trait for `Signature` using Serde.
impl<'de> serde::Deserialize<'de> for Signature {
    /// Deserializes a `Signature` from a string in JSON.
    ///
    /// # Parameters
    /// - `deserializer`: The Serde deserializer.
    ///
    /// # Returns
    /// - `Ok(Signature)` if the deserialization and parsing succeed.
    /// - `Err(D::Error)` if deserialization or parsing fails.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        s.parse()
            .map_err(|problem| serde::de::Error::custom(format!("Signature problem is: {}", problem)))
    }
}

/// Implements the `Serialize` trait for `Signature` using Serde.
impl serde::Serialize for Signature {
    /// Serializes a `Signature` to a string in JSON.
    ///
    /// # Parameters
    /// - `serializer`: The Serde serializer.
    ///
    /// # Returns
    /// - The serialized string representation of the `Signature`.
    ///
    /// # Example
    /// The `Signature` will be serialized as a hexadecimal string prefixed with `0x`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = [0u8; 2 + 2 * 65];
        s[0] = b'0';
        s[1] = b'x';
        let arr: [u8; 65] = (*self).into();
        // Safety: This will never error as it has exactly enough space in the buffer.
        unsafe {
            hex::encode_to_slice(arr, &mut s[2..]).unwrap_unchecked();
        }
        serializer.serialize_str(StackStr::new(s).as_ref())
    }
}


/// Represents a sep256k1 public key that has been used to sign an Aqua-Chain.
/// Includes the signature itself, the public key used to verify it,
/// and the associated hash and wallet address.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RevisionSignature {
    pub signature: Signature,
    pub public_key: PublicKey,
    pub signature_hash: Hash,
    // todo: remove with v1.2
    pub wallet_address: Address,
}

#[test]
fn test_read() {
    const TEST_DATA: &str = 
        "0xf0d0cadd0c82ade49db1e3443615dca67856e94b85d5590a2970d442e09b96e66fe9326f55a1e24b95f960f985bb524200be428d7084833db9ce7e778e2932121c";
    let _encoded_str: Signature =
        std::str::FromStr::from_str(TEST_DATA).expect("Correct Signature not read.");
    //dbg!(_encoded_str);
    const TEST_DATA_TOO_LONG: &str =
        "0xf0d0cadd0c82ade49db1e3443615dca67856e94b85d5590a2970d442e09b96e66fe9326f55a1e24b95f960f985bb524200be42048b18d7084833db9ce7e778e2932121c";
    <Signature as std::str::FromStr>::from_str(TEST_DATA_TOO_LONG)
        .expect_err("Accepted overly long signature");
    const TEST_DATA_NOPREFIX: &str =
        "f0d0cadd0c82ade49db1e3443615dca67856e94b85d5590a2970d442e09b96e66fe9326f55a1e24b95f960f985bb524200be428d7084833db9ce7e778e2932121c";
    <Signature as std::str::FromStr>::from_str(TEST_DATA_NOPREFIX)
        .expect_err("Accepted signature without 0x prefix.");
    const TEST_DATA_WITH_UPPER: &str = 
        "0xf0d0cadd0c82aDe49db1e3443615dca67856E94b85D5590a2970d442e09b96E66fe9326f55A1e24b95f960f985bb524200be428d7084833db9ce7e778e2932121C";
    <Signature as std::str::FromStr>::from_str(TEST_DATA_WITH_UPPER)
        .expect_err("Accepted signature with miXeD caSe.");
}

#[test]
fn test_write() {
    const TEST_DATA: &str = "0x52e60271ddeb607df95393b41d941f716de90ea7a901067b9f112aa5b737b8cc5c940b9374c950e518c06972a18feecff7b303977c0baf029b64e99b5754b4cf1c";
    let signature_thing: Signature = TEST_DATA.parse().expect("Correct Signature not read.");
    assert_eq!(TEST_DATA, &*signature_thing.to_stackstr(), "stuff broke");
}
