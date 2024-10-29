use super::hash::Hash;
use crate::models::revision::Revision;
use serde::{Deserialize, Serialize};
// use serde_with::serde_as;
// use serde_with::{formats::tuple_list};
// use serde_with::{DisplayFromStr, formats::};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameSpace {
    case: bool,
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiteInfo {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageData {
    pub pages: Vec<HashChain>,
    pub site_info: SiteInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashChain {
    pub genesis_hash: String,
    pub domain_id: String,
    pub title: String,
    pub namespace: u64,
    pub chain_height: u64,
    // #[serde(with = "tuple_vec_map")]
    // #[serde_as(as = "Vec<(_, _)>")]
    // #[serde(with = "tuple_list")]
    pub revisions: Vec<(Hash, Revision)>,
}
