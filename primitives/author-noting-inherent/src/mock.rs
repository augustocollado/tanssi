/// Inherent data provider that supplies mocked author noting data.
///
/// This is useful when running a node that is not actually backed by any relay chain.
/// For example when running a local node, or running integration tests.
///
/// We mock a relay chain block number as follows:
/// relay_block_number = offset + relay_blocks_per_para_block * current_para_block
/// To simulate a parachain that starts in relay block 1000 and gets a block in every other relay
/// block, use 1000 and 2
///
/// para_id: the parachain of which we are gonna mock the headData
/// slots_per_para_block: the number of slots to be applied per parachain block
use crate::AuthorNotingSproofBuilder;
use crate::HeaderAs;
use crate::OwnParachainInherentData;
use cumulus_primitives_core::PersistedValidationData;
use parity_scale_codec::Encode;
use sp_consensus_aura::inherents::InherentType;
use sp_consensus_aura::AURA_ENGINE_ID;
use sp_inherents::InherentData;
use sp_inherents::InherentDataProvider;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::DigestItem;

pub struct MockAuthorNotingInherentDataProvider {
    /// The current block number of the local block chain (the parachain)
    pub current_para_block: u32,
    /// The relay block in which this parachain appeared to start. This will be the relay block
    /// number in para block #P1
    pub relay_offset: u32,
    /// The number of relay blocks that elapses between each parablock. Probably set this to 1 or 2
    /// to simulate optimistic or realistic relay chain behavior.
    pub relay_blocks_per_para_block: u32,
    /// List of para ids for which to include the header proof. They will all have the same slot number.
    pub para_ids: Vec<u32>,
    /// Number of parachain blocks per relay chain epoch
    /// Mock epoch is computed by dividing `current_para_block` by this value.
    pub slots_per_para_block: u32,
}

#[async_trait::async_trait]
impl InherentDataProvider for MockAuthorNotingInherentDataProvider {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut InherentData,
    ) -> Result<(), sp_inherents::Error> {
        // Calculate the mocked relay block based on the current para block
        let relay_parent_number =
            self.relay_offset + self.relay_blocks_per_para_block * self.current_para_block;

        // Calculate the mocked relay block based on the current para block
        let slot_number =
            InherentType::from(self.slots_per_para_block as u64 * self.current_para_block as u64);

        let mut sproof_builder = AuthorNotingSproofBuilder::default();

        // Use the "sproof" (spoof proof) builder to build valid mock state root and proof.
        for para_id in self.para_ids.iter() {
            let mut sproof_builder_item = crate::AuthorNotingSproofBuilderItem::default();
            sproof_builder_item.para_id = (*para_id).into();

            let header = HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: Default::default(),
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot_number.encode())],
                },
            });
            sproof_builder_item.author_id = header;

            sproof_builder.items.push(sproof_builder_item);
        }

        let (relay_parent_storage_root, proof) = sproof_builder.into_state_root_and_proof();

        inherent_data.put_data(
            crate::INHERENT_IDENTIFIER,
            &OwnParachainInherentData {
                validation_data: PersistedValidationData {
                    parent_head: Default::default(),
                    relay_parent_storage_root,
                    relay_parent_number,
                    max_pov_size: Default::default(),
                },
                relay_chain_state: proof,
            },
        )?;

        Ok(())
    }

    // Copied from the real implementation
    async fn try_handle_error(
        &self,
        _: &sp_inherents::InherentIdentifier,
        _: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        None
    }
}
