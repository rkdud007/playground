use alloy_primitives::{keccak256, B256, U256};
use alloy_rlp::{RlpDecodable, RlpEncodable};

/// An Ethereum account as represented in the trie.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, RlpEncodable, RlpDecodable)]
pub struct TrieAccount {
    /// Account nonce.
    nonce: u64,
    /// Account balance.
    balance: U256,
    /// Account's storage root.
    storage_root: B256,
    /// Hash of the account's bytecode.
    code_hash: B256,
}

impl TrieAccount {
    /// Get account's storage root.
    pub fn storage_root(&self) -> B256 {
        self.storage_root
    }
}
