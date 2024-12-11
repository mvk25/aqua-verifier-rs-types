//! Hash module defines the `Hash` struct, which wraps a cryptographic hash value and provides utility methods for serialization, deserialization, and type conversions.


use crate::models::stack_str::{StackStr, from_hex};


// Represents a cryptographic hash, specifically a SHA-3 512-bit hash.
/// 
/// The `Hash` struct wraps a cryptographic hash value and provides utility
/// methods for serialization, deserialization, formatting and conversion
/// to and from other types.
#[derive(Hash, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hash(crate::crypt::Hash);

impl core::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_stackstr()[..])
        // f.write_fmt(format_args!("{}..", &self.to_stackstr()[..20]))
    }
}

impl Hash {
    /// Converts the `Hash` into a `StackStr<128>` containing the hex-encoded hash.
    /// 
    /// # Safety
    /// This method assumes that the `hex::encode_to_slice` function always succeeds
    /// because the provided buffer is sized correctly.
    pub fn to_stackstr(self) -> StackStr<128> {
        let mut arr = [0; 128];
        // Safety: data is exactly the right size for the hex output
        unsafe {
            hex::encode_to_slice(self.0, &mut arr[..]).unwrap_unchecked();
        }
        // StackStr(arr)
        StackStr::new(arr)
    }
}


impl std::str::FromStr for Hash {
    // todo: err
    /// Error type for failing parsing, represented as ().
    type Err = ();

    /// Parses the hex string into a `Hash`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hash(from_hex(s).ok_or(())?.into()))
    }
}

impl From<[u8; 64]> for Hash {
    /// Converts a byte array of length 64 into a `Hash`.
    fn from(value: [u8; 64]) -> Self {
        crate::crypt::Hash::from(value).into()
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = [0u8; 64 * 2];
        // Safety: data is exactly the right size for the hex output
        unsafe {
            hex::encode_to_slice(<[u8; 64]>::from(self.0), &mut data).unwrap_unchecked();
        }
        f.write_str(StackStr::new(data).as_ref())
    }
}

/// Implements the `std::ops::Deref` trait for `Hash`.
/// This allows `Hash` to be treated as a reference to `crate::crypt::Hash`.
impl std::ops::Deref for Hash {
    /// The target type that `Hash` dereferences to.
    type Target = crate::crypt::Hash;

    /// Dereferences `Hash` to access the inner `crate::crypt::Hash`.
    /// 
    /// # Returns
    /// A reference to the inner `crate::crypt::Hash`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Converts a `crate::crypt::Hash` into a `Hash`.
impl From<crate::crypt::Hash> for Hash {
    /// Performs the conversion by wrapping the `crate::crypt::Hash` into `Hash`.
    ///
    /// # Parameters
    /// - `value`: The `crate::crypt::Hash` to be converted.
    ///
    /// # Returns
    /// A new `Hash` instance containing the given `crate::crypt::Hash`.
    fn from(value: crate::crypt::Hash) -> Self {
        Self(value)
    }
}

// impl From<[u8; 64]> for Hash {
//     fn from(value: [u8; 64]) -> Self {
//         Into::<crate::crypt::Hash>::into(value).into()
//     }
// }

/// Converts a `Hash` into a `crate::crypt::Hash`.
impl From<Hash> for crate::crypt::Hash {
    /// Extracts the inner `crate::crypt::Hash` from the `Hash`.
    ///
    /// # Parameters
    /// - `val`: The `Hash` instance to be converted.
    ///
    /// # Returns
    /// The inner `crate::crypt::Hash`.
    fn from(val: Hash) -> Self {
        val.0
    }
}

/// Implements `serde::Deserialize` for `Hash`.
/// This allows a `Hash` to be deserialized from a string representation.
impl<'de> serde::Deserialize<'de> for Hash {
    /// Deserializes a `Hash` from a string.
    ///
    /// # Parameters
    /// - `deserializer`: The deserializer instance.
    ///
    /// # Returns
    /// - `Ok(Hash)` if the string is successfully parsed as a valid `sha3_512` hash.
    /// - `Err(D::Error)` if the string is invalid.
    ///
    /// # Errors
    /// Returns a custom error if the string is not a valid `sha3_512` hash.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom("Invalid sha3_512 hash"))
    }
}

/// Implements `serde::Serialize` for `Hash`.
/// This allows a `Hash` to be serialized as a hexadecimal string.
impl serde::Serialize for Hash {
    /// Serializes the `Hash` into a hexadecimal string.
    ///
    /// # Parameters
    /// - `serializer`: The serializer instance.
    ///
    /// # Returns
    /// - `Ok(S::Ok)` if serialization succeeds.
    /// - `Err(S::Error)` if serialization fails.
    ///
    /// # Example
    /// Converts the inner hash into a hexadecimal string and serializes it.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0[..]))
    }
}

#[test]
fn test_read() {
    const TEST_DATA: &str = "d9e09f8529fed3b909876f34f21c7148d73de01d82f8aee43c52d9ee2601999ddcbf4593a19baac497d9d83bb98c94c2508b8157efafcd6484cbca7c4953af5f";
    let _hash: Hash = TEST_DATA.parse().expect("Correct Hash not read.");
    //dbg!(_hash);
    const TEST_DATA_NOPREFIX: &str = 
        "0xd9e09f8529fed3b909876f34f21c7148d73de01d82f8aee43c52d9ee2601999ddcbf4593a19baac497d9d83bb98c94c2508b8157efafcd6484cbca7c4953af5f";
    <Hash as std::str::FromStr>::from_str(TEST_DATA_NOPREFIX)
        .expect_err("Accepted data with prefix.");
    const TEST_DATA_WITH_UPPER: &str = 
        "0xd9e09f8529fed3b909876F34f21c7148d73de01d82f8aEe43c52d9ee2601999dDcbf4593a19baac497d9d83bb98c94c2508b8157efafcd6484cbca7c4953af5f";
    <Hash as std::str::FromStr>::from_str(TEST_DATA_WITH_UPPER)
        .expect_err("Accepted data witH mIxeD cAsE.");
}

#[test]
fn test_write() {
    const TEST_DATA: &str = "d9e09f8529fed3b909876f34f21c7148d73de01d82f8aee43c52d9ee2601999ddcbf4593a19baac497d9d83bb98c94c2508b8157efafcd6484cbca7c4953af5f";
    let hash_thing: Hash = TEST_DATA.parse().expect("Correct Hash not read.");
    println!("Cannot Check Output at this time.");
    assert_eq!(TEST_DATA, &hash_thing.to_string(), "stuff broke");
}
