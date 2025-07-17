//! OCOS-Chain: NFT Token (ERC-721/1155-style) Core Logic
//!
//! Provides mint, burn, transfer, approve, and owner query logic for non-fungible tokens.
//! Fully audit-ready and DAO/governance extensible.

use crate::nft::types::{NFTId, CollectionId, Address};
use crate::nft::error::NFTError;
use crate::nft::events::NFTEvent;
use std::collections::HashMap;

/// NFT ownership and approval state
#[derive(Default)]
pub struct NFTLedger {
    /// nft_id → owner address
    pub owners: HashMap<NFTId, Address>,
    /// nft_id → approved address
    pub approvals: HashMap<NFTId, Address>,
    /// owner address → all owned NFTs (for enumeration)
    pub owned_tokens: HashMap<Address, Vec<NFTId>>,
}

impl NFTLedger {
    /// Mint a new NFT to a given address
    pub fn mint(
        &mut self,
        nft_id: NFTId,
        to: Address,
        events: &mut Vec<NFTEvent>,
    ) -> Result<(), NFTError> {
        if self.owners.contains_key(&nft_id) {
            return Err(NFTError::AlreadyExists);
        }
        self.owners.insert(nft_id, to);
        self.owned_tokens.entry(to).or_default().push(nft_id);
        events.push(NFTEvent::Minted { nft_id, to });
        Ok(())
    }

    /// Burn an NFT (permanently remove)
    pub fn burn(
        &mut self,
        nft_id: NFTId,
        caller: Address,
        events: &mut Vec<NFTEvent>,
    ) -> Result<(), NFTError> {
        let owner = self.owners.get(&nft_id).ok_or(NFTError::NotFound)?;
        if owner != &caller {
            return Err(NFTError::Unauthorized);
        }
        self.owners.remove(&nft_id);
        self.approvals.remove(&nft_id);
        if let Some(tokens) = self.owned_tokens.get_mut(owner) {
            tokens.retain(|id| id != &nft_id);
        }
        events.push(NFTEvent::Burned { nft_id, by: caller });
        Ok(())
    }

    /// Transfer NFT from caller to another address
    pub fn transfer(
        &mut self,
        nft_id: NFTId,
        from: Address,
        to: Address,
        events: &mut Vec<NFTEvent>,
    ) -> Result<(), NFTError> {
        let owner = self.owners.get(&nft_id).ok_or(NFTError::NotFound)?;
        let approved = self.approvals.get(&nft_id);
        if owner != &from && approved != Some(&from) {
            return Err(NFTError::Unauthorized);
        }
        self.owners.insert(nft_id, to);
        if let Some(tokens) = self.owned_tokens.get_mut(&from) {
            tokens.retain(|id| id != &nft_id);
        }
        self.owned_tokens.entry(to).or_default().push(nft_id);
        self.approvals.remove(&nft_id);
        events.push(NFTEvent::Transferred { nft_id, from, to });
        Ok(())
    }

    /// Approve another address to transfer a specific NFT
    pub fn approve(
        &mut self,
        nft_id: NFTId,
        owner: Address,
        to: Address,
        events: &mut Vec<NFTEvent>,
    ) -> Result<(), NFTError> {
        let current_owner = self.owners.get(&nft_id).ok_or(NFTError::NotFound)?;
        if current_owner != &owner {
            return Err(NFTError::Unauthorized);
        }
        self.approvals.insert(nft_id, to);
        events.push(NFTEvent::Approved { nft_id, owner, approved: to });
        Ok(())
    }

    /// Query the owner of an NFT
    pub fn owner_of(&self, nft_id: NFTId) -> Option<Address> {
        self.owners.get(&nft_id).cloned()
    }

    /// Check if an address owns any NFTs
    pub fn tokens_of_owner(&self, owner: Address) -> Vec<NFTId> {
        self.owned_tokens.get(&owner).cloned().unwrap_or_default()
    }
}
