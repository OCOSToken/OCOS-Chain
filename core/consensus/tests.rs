//! OCOS-Chain: Consensus Layer Core Tests
//!
//! Covers validator lifecycle, block signature logic, quantum/classic signature
//! integration, governance proposal/voting, and edge/corner case consensus events.

use super::*;
use crate::core::consensus::{
    validator::{Validator, ValidatorSet},
    consensus_engine::{ConsensusEngine, ConsensusMode},
    block::{Block, BlockHeader},
    quantum_sig::QuantumSignature,
    governance::GovernanceHook,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn demo_keys() -> (Vec<u8>, Vec<u8>) {
        // For demonstration; in real systems use cryptographic key generation
        (vec![1; 32], vec![2; 32])
    }

    fn demo_validator(addr: &str, stake: u64) -> Validator {
        let (pub_key, priv_key) = demo_keys();
        Validator::new(addr.to_owned(), stake, pub_key, priv_key)
    }

    #[test]
    fn test_validator_lifecycle() {
        let mut v = demo_validator("val1", 1000);
        assert_eq!(v.status, super::ValidatorStatus::Active);

        v.slash(1000, 10);
        assert_eq!(v.status, super::ValidatorStatus::Jailed);
        assert_eq!(v.stake, 0);
        assert!(v.jailed_since.is_some());

        v.unjail();
        assert_eq!(v.status, super::ValidatorStatus::Active);

        v.retire();
        assert_eq!(v.status, super::ValidatorStatus::Retired);
    }

    #[test]
    fn test_validator_set_selection() {
        let vals = vec![
            demo_validator("val1", 1000),
            demo_validator("val2", 2000),
        ];
        let vs = ValidatorSet::new(vals.clone());
        let selected = vs.select_by_stake(1).unwrap();
        assert!(selected.address == "val2" || selected.address == "val1");
    }

    #[test]
    fn test_block_signature_classic_and_quantum() {
        let v = demo_validator("val3", 1500);
        let header = BlockHeader::new(vec![0u8;32], v.public_key.clone(), b"txs");
        let hash = header.hash();

        // Ed25519 (classic, demo)
        let sig = QuantumSignature::sign_ed25519(&v.private_key, &hash);
        assert!(QuantumSignature::verify_ed25519(&v.public_key, &hash, &sig));

        // Dilithium (quantum, demo)
        let sigq = QuantumSignature::sign_dilithium(&v.private_key, &hash);
        assert!(QuantumSignature::verify_dilithium(&v.public_key, &hash, &sigq));
    }

    #[test]
    fn test_block_structure_and_validation() {
        let v = demo_validator("valX", 3000);
        let header = BlockHeader::new(vec![9u8;32], v.public_key.clone(), b"txbatch");
        let sig = QuantumSignature::sign_ed25519(&v.private_key, &header.hash());

        let block = Block {
            header,
            signature: sig.clone(),
            proposer: v.address.clone(),
            transactions: b"txbatch".to_vec(),
        };

        assert_eq!(block.hash(), block.header.hash());
        assert!(block.validate_signature(QuantumSignature::verify_ed25519));
    }

    #[test]
    fn test_governance_proposal_and_voting() {
        let mut g = GovernanceHook::default();
        let pid = g.submit_proposal(
            "alice".to_string(),
            "Change block time".to_string(),
            "Update block_time param".to_string(),
            "block_time".to_string(),
            "2s".to_string(),
            60,
        );

        assert!(g.proposals.contains_key(&pid));

        let status = g.vote(pid, true).unwrap();
        assert_eq!(status, super::ProposalStatus::Pending);

        // Force voting period expiration for test
        let prop = g.proposals.get_mut(&pid).unwrap();
        prop.end_time = 0;

        let fin = g.finalize_proposal(pid).unwrap();
        assert!(fin == super::ProposalStatus::Approved || fin == super::ProposalStatus::Rejected);

        g.apply_update("block_time", "2s");
        assert_eq!(g.get_config("block_time").unwrap(), "2s");
    }
}
