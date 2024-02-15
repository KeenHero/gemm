use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;

use crate::{impl_account_from_bytes, impl_to_bytes};

use super::{AccountDiscriminator, Discriminator, Hash};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Proof {
    /// The bump of the proof account PDA.
    pub bump: u64,

    /// The account (i.e. miner) authorized to use this proof.
    pub authority: Pubkey,

    /// The quantity of tokens this miner may claim from the treasury.
    pub claimable_rewards: u64,

    /// The proof's current hash.
    pub hash: Hash,

    /// The total lifetime hashes provided by this miner.
    pub total_hashes: u64,

    /// The total lifetime rewards distributed to this miner.
    pub total_rewards: u64,
}

impl Discriminator for Proof {
    fn discriminator() -> super::AccountDiscriminator {
        AccountDiscriminator::Proof
    }
}

impl_to_bytes!(Proof);
impl_account_from_bytes!(Proof);
