//! Page_data defines structures related to page data, including `HashChain` for revision chains, `NameSpace` for domain namespaces, `PageData` for page and site information, and `SiteInfo` for site-level metadata.


use crate::models::hash::Hash;
use crate::models::revision::Revision;
use serde::{Deserialize, Serialize};
// use serde_with::serde_as;
// use serde_with::{formats::tuple_list};
// use serde_with::{DisplayFromStr, formats::};

/// Represents a namespace within a domain.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameSpace {
    /// Indicates if the namespace is case-sensitive.
    case: bool,
    /// The title of the namespace.
    title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiteInfo {}

/// Contains information about pages and site metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageData {
    /// A collection of hash chains representing pages
    pub pages: Vec<HashChain>,
    /// Metadata about the site.
    pub site_info: SiteInfo,
}

/// Represents a chain of revisions for a specific page.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashChain {
    /// The initial hash of the chain, marking its starting point.
    pub genesis_hash: String,
    /// The domain ID to which this chains belong.
    pub domain_id: String,
    /// The title of the page this chain represents.
    pub title: String,
    /// The namespace of the identifier.
    pub namespace: u64,
    /// The number of revisions in the chain.
    pub chain_height: u64,
    // #[serde_as(as = "Vec<(_, _)>")]
    // #[serde(with = "tuple_list")]
    /// A list of revisions represented as tuples
    /// where the first element is `Hash` representing the unique hash
    /// of the revision and the second element is `Revision` object 
    /// describing the revision details
    #[serde(with = "tuple_vec_map")]
    pub revisions: Vec<(Hash, Revision)>,
}
