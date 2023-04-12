use cumulus_primitives_core::relay_chain::HeadData;
use cumulus_primitives_core::ParaId;
use frame_support::Hashable;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::HashFor;
use sp_trie::MemoryDB;

// Enum representing how we want to insert the Header
#[derive(Clone)]
pub enum HeaderAs {
    AlreadyEncoded(Vec<u8>),
    NonEncoded(sp_runtime::generic::Header<u32, BlakeTwo256>),
}

/// Builds a sproof (portmanteau of 'spoof' and 'proof') of the relay chain state.
#[derive(Clone)]
pub struct AuthorNotingSproofBuilderItem {
    /// The para id of the current parachain.
    pub para_id: ParaId,

    /// The author_id, which represents a Header with a Aura Digest
    pub author_id: HeaderAs,
}

impl Default for AuthorNotingSproofBuilderItem {
    fn default() -> Self {
        Self {
            para_id: ParaId::from(200),
            author_id: HeaderAs::AlreadyEncoded(vec![]),
        }
    }
}

/// Builds a sproof (portmanteau of 'spoof' and 'proof') of the relay chain state.
/// Receives a vec of individual AuthorNotingSproofBuilderItem items of which
/// we need to insert the header
#[derive(Clone, Default)]
pub struct AuthorNotingSproofBuilder {
    pub items: Vec<AuthorNotingSproofBuilderItem>,
}

impl AuthorNotingSproofBuilder {
    pub fn into_state_root_and_proof(
        self,
    ) -> (
        polkadot_primitives::v2::Hash,
        sp_state_machine::StorageProof,
    ) {
        let (db, root) = MemoryDB::<HashFor<polkadot_primitives::v2::Block>>::default_with_root();
        let state_version = Default::default(); // for test using default.
        let mut backend = sp_state_machine::TrieBackendBuilder::new(db, root).build();

        let mut relevant_keys = Vec::new();
        {
            use parity_scale_codec::Encode as _;

            let mut insert = |key: Vec<u8>, value: Vec<u8>| {
                relevant_keys.push(key.clone());
                backend.insert(vec![(None, vec![(key, Some(value))])], state_version);
            };

            for item in self.items {
                let para_key = item.para_id.twox_64_concat();
                let key = [crate::PARAS_HEADS_INDEX, para_key.as_slice()].concat();

                let encoded = match item.author_id {
                    HeaderAs::AlreadyEncoded(encoded) => encoded,
                    HeaderAs::NonEncoded(header) => header.encode(),
                };

                let head_data: HeadData = encoded.into();
                insert(key, head_data.encode());
            }
        }

        let root = backend.root().clone();
        let proof = sp_state_machine::prove_read(backend, relevant_keys).expect("prove read");

        (root, proof)
    }
}
