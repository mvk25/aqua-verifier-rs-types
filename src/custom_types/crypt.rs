
pub mod crypt {
    pub type Hasher = sha3::Sha3_512;
    pub type Hash = sha3::digest::Output<Hasher>;
    pub use sha3::*;
}