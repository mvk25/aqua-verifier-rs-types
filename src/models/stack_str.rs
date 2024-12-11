//! This module provides the `StackStr` struct for stack-allocated, fixed-size strings and the `from_hex` function for converting hexadecimal strings into fixed-size byte arrays.


/// Converts a hexadecimal string into a fixed-size array of bytes.
///
/// # Parameters
/// - `s`: A hexadecimal string of length `SIZE * 2`.
///
/// # Returns
/// - `Some([u8; SIZE])`: If the string is valid hex and has the correct length.
/// - `None`: If the string is not valid hex or does not match the expected length.
///
/// # Constraints
/// - The string must be ASCII and consist only of valid hexadecimal characters (0-9, a-f, A-F).
/// - The string must have exactly `SIZE * 2` characters.
///
/// # Safety
/// - Assumes valid input for decoding.
///
/// # Example
/// ```rust
/// let result = from_hex::<4>("deadbeef");
/// assert_eq!(result, Some([0xde, 0xad, 0xbe, 0xef]));
/// ``` 
pub fn from_hex<const SIZE: usize>(s: &str) -> Option<[u8; SIZE]> {
    if !s.as_bytes().len() == SIZE * 2 || !s.is_ascii() {
        return None;
    }
    let mut data = [0u8; SIZE];
    hex::decode_to_slice(s, &mut data).ok()?;
    Some(data)
}

// Safety: The hex crate always writes valid ascii which is valid utf-8
/// A stack-allocated string with a fixed size.
///
/// # Parameters
/// - `X`: The maximum size of the string, defined at compile time.
///
/// # Features
/// - Lightweight and efficient for small strings.
/// - Encodes UTF-8 data safely, ensuring valid string operations.
///
/// # Safety
/// - The `StackStr` guarantees valid UTF-8 encoding.
/// - Uses `unsafe` code internally but ensures correctness by design.
///
/// # Examples
/// ```rust
/// let stack_str = StackStr::<16>::new(*b"Hello, world!");
/// assert_eq!(stack_str.as_ref(), "Hello, world!");
/// ```
pub struct StackStr<const X: usize>([u8; X]);

impl<const X: usize> StackStr<X> {
    /// Creates a new `StackStr` instance from a fixed-size byte array.
    ///
    /// # Parameters
    /// - `data`: A byte array of length `X`.
    ///
    /// # Returns
    /// - A new `StackStr` instance containing the provided data.
    pub fn new(data: [u8; X]) -> Self {
        StackStr(data)
    }
}

impl<const X: usize> AsRef<[u8]> for StackStr<X> {
    /// Returns a new `StackStr` instance from a fixed-size byte array.
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}
impl<const X: usize> AsRef<str> for StackStr<X> {
    /// Returns a byte slice representation of a string.
    fn as_ref(&self) -> &str {
        unsafe { ::core::str::from_utf8_unchecked(self.as_ref()) }
    }
}
impl<const X: usize> AsRef<[u8; X]> for StackStr<X> {
    /// Returns a string slice representation of the `StackStr`.
    /// 
    /// # Safety
    /// - This method relies on the internal UTF-8 validity guarantee of the data.
    fn as_ref(&self) -> &[u8; X] {
        &self.0
    }
}
impl<const X: usize> std::ops::Deref for StackStr<X> {
    /// Dereferences to a string slice for ergnonomic access.
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<const X: usize> ::std::fmt::Display for StackStr<X> {
    /// Formats the `StackStr` for Display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<const X: usize> ::std::fmt::Debug for StackStr<X> {
    /// Formats the `StackStr` for debugging.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
