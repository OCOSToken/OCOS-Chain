//! OCOS-Chain: Consensus Engine Core Implementation
//!
//! Hybrid PoS/PoA consensus engine with post-quantum security,
//! modular validator management, block proposal/finality, and governance hooks.

use crate::core::consensus::{
    validator::{Validator, ValidatorSet},
    quantum_sig::QuantumSignature,
    block::{Block, BlockHeader},
    slashing::SlashingManager,
    governance::GovernanceHook,
    ConsensusError, ConsensusEvent,
};

/// Consensus operation mode (PoS, PoA, Hybrid, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsensusMode {
    ProofOfStake,
    ProofOfAuthority,
    Hybrid,
}

/// Main Consensus Engine struct
pub struct ConsensusEngine {
    pub mode: ConsensusMode,
    pub validators: ValidatorSet,
    pub slashing: SlashingManager,
    pub governance: GovernanceHook,
    pub quantum_mode: bool,
}

impl ConsensusEngine {
    /// Create a new ConsensusEngine instance
    pub fn new(validators: Vec<Validator>, mode: ConsensusMode) -> Self {
        ConsensusEngine {
            mode,
            quantum_mode: true, // default: quantum-safe
            validators: ValidatorSet::new(validators),
            slashing: SlashingManager::default(),
            governance: GovernanceHook::default(),
        }
    }

    /// Propose a new block (by validator)
    pub fn propose_block(&self, validator_addr: &str, prev_block: &Block, txs: &[u8]) -> Result<Block, ConsensusError> {
        let validator = self.validators.get_by_address(validator_addr)
            .ok_or(ConsensusError::Unauthorized)?;

        // Create block header
        let header = BlockHeader::new(
            prev_block.header.hash(),
            validator.public_key.clone(),
            txs,
        );

        // Block signature (quantum or classic)
        let signature = if self.quantum_mode {
            QuantumSignature::sign_dilithium(&validator.private_key, &header.hash())
        } else {
            QuantumSignature::sign_ed25519(&validator.private_key, &header.hash())
        };

        let block = Block {
            header,
            signature,
            proposer: validator_addr.to_owned(),
            transactions: txs.to_vec(),
        };

        // Audit event
        self.log_event(ConsensusEvent::BlockProposed {
            proposer: validator_addr.to_owned(),
            height: block.header.height,
        });

        Ok(block)
    }

    /// Validate block signature (for block finality)
    pub fn verify_block(&self, block: &Block) -> Result<(), ConsensusError> {
        let pub_key = &block.header.proposer_public_key;
        let hash = block.header.hash();
        let signature = &block.signature;

        let valid = if self.quantum_mode {
            QuantumSignature::verify_dilithium(pub_key, &hash, signature)
        } else {
            QuantumSignature::verify_ed25519(pub_key, &hash, signature)
        };
        if !valid {
            return Err(ConsensusError::InvalidSignature);
        }
        Ok(())
    }

    /// Select next leader/validator (PoS/PoA)
    pub fn select_leader(&self, height: u64) -> Option<&Validator> {
        match self.mode {
            ConsensusMode::ProofOfStake | ConsensusMode::Hybrid => self.validators.select_by_stake(height),
            ConsensusMode::ProofOfAuthority => self.validators.select_by_round_robin(height),
        }
    }

    /// Slash validator (for misbehavior)
    pub fn slash_validator(&mut self, address: &str, reason: &str, amount: u64) -> Result<(), ConsensusError> {
        let validator = self.validators.get_by_address_mut(address)
            .ok_or(ConsensusError::Unauthorized)?;
        self.slashing.slash(validator, amount);
        self.log_event(ConsensusEvent::ValidatorSlashed {
            validator: address.to_owned(),
            amount,
        });
        Ok(())
    }

    /// Apply governance update (on-chain config changes)
    pub fn apply_governance_update(&mut self, key: &str, value: &str) -> Result<(), ConsensusError> {
        if self.governance.validate_update(key, value) {
            self.governance.apply_update(key, value);
            self.log_event(ConsensusEvent::GovernanceUpdate {
                key: key.to_owned(),
                value: value.to_owned(),
            });
            Ok(())
        } else {
            Err(ConsensusError::GovernanceRejected)
        }
    }

    /// Emit consensus events (for audit/tracing)
    fn log_event(&self, event: ConsensusEvent) {
        // Here you can implement full audit log integration, persistent storage, etc.
        println!("[CONSENSUS EVENT] {:?}", event);
    }
}
