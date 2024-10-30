// pub mod models;

pub mod models {
    pub mod content;
    pub mod hash;
    pub mod metadata;
    pub mod signature;
    pub mod witness;
    pub mod base64;
    pub mod stack_str;
    pub mod timestamp;
    pub mod public_key;
    pub mod tx_hash;
    pub mod page_data;
    pub mod revision;

    pub mod storage;
    pub mod branch;

    // Tests
    pub mod tests;
}

pub mod crypt {
    pub type Hasher = sha3::Sha3_512;
    pub type Hash = sha3::digest::Output<Hasher>;
    pub use sha3::*;
}


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
