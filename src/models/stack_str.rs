pub fn from_hex<const SIZE: usize>(s: &str) -> Option<[u8; SIZE]> {
    // make sure it has the correct length (2 characters per byte) and that it is only valic characters
    if !s.as_bytes().len() == SIZE * 2 || !s.is_ascii() {
        return None;
    }
    let mut data = [0u8; SIZE];
    hex::decode_to_slice(s, &mut data).ok()?;
    Some(data)
}

// Safety: The hex crate always writes valid ascii which is valid utf-8
/// Used for more efficient String operations
pub struct StackStr<const X: usize>([u8; X]);

impl<const X: usize> StackStr<X> {
    pub fn new(data: [u8; X]) -> Self {
        StackStr(data)
    }
}

impl<const X: usize> AsRef<[u8]> for StackStr<X> {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}
impl<const X: usize> AsRef<str> for StackStr<X> {
    fn as_ref(&self) -> &str {
        unsafe { ::core::str::from_utf8_unchecked(self.as_ref()) }
    }
}
impl<const X: usize> AsRef<[u8; X]> for StackStr<X> {
    fn as_ref(&self) -> &[u8; X] {
        &self.0
    }
}
impl<const X: usize> std::ops::Deref for StackStr<X> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<const X: usize> ::std::fmt::Display for StackStr<X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<const X: usize> ::std::fmt::Debug for StackStr<X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
