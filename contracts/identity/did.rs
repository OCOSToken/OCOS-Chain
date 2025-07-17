//! OCOS-Chain: Decentralized Identifier (DID) Module
//!
//! Provides on-chain creation, update, resolution, and verification of W3C-standard DIDs
//! for OCOS users, DAOs, and Web3 applications. Supports key management, controller lists,
//! service endpoints, ZK/attestation proofs, and audit logging.

use crate::identity::types::{DID, Address, DIDDocument, ServiceEndpoint, VerificationMethod};
use crate::identity::error::IdentityError;
use std::collections::HashMap;

/// DID registry storage
#[derive(Default)]
pub struct DIDRegistry {
    /// did → DIDDocument
    pub documents: HashMap<DID, DIDDocument>,
}

impl DIDRegistry {
    /// Create a new DID document (register a new identity)
    pub fn create_did(
        &mut self,
        did: DID,
        controller: Address,
        vm: VerificationMethod,
        services: Vec<ServiceEndpoint>,
    ) -> Result<(), IdentityError> {
        if self.documents.contains_key(&did) {
            return Err(IdentityError::DIDAlreadyExists);
        }
        let doc = DIDDocument {
            did: did.clone(),
            controller,
            verification_methods: vec![vm],
            services,
            updated_at: Self::current_timestamp(),
            attested_by: vec![],
        };
        self.documents.insert(did, doc);
        Ok(())
    }

    /// Update verification method or service endpoints for a DID
    pub fn update_did(
        &mut self,
        did: &DID,
        new_vm: Option<VerificationMethod>,
        new_services: Option<Vec<ServiceEndpoint>>,
        by: Address,
    ) -> Result<(), IdentityError> {
        let doc = self.documents.get_mut(did).ok_or(IdentityError::DIDNotFound)?;
        if doc.controller != by {
            return Err(IdentityError::Unauthorized);
        }
        if let Some(vm) = new_vm {
            doc.verification_methods.push(vm);
        }
        if let Some(services) = new_services {
            doc.services = services;
        }
        doc.updated_at = Self::current_timestamp();
        Ok(())
    }

    /// Add attestation (DAO or trusted third party) to a DID
    pub fn attest(
        &mut self,
        did: &DID,
        attestor: Address,
        proof: Vec<u8>,
    ) -> Result<(), IdentityError> {
        let doc = self.documents.get_mut(did).ok_or(IdentityError::DIDNotFound)?;
        doc.attested_by.push((attestor, proof));
        Ok(())
    }

    /// Resolve (query) a DID document
    pub fn resolve(&self, did: &DID) -> Option<&DIDDocument> {
        self.documents.get(did)
    }

    /// Helper: current unix timestamp
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}
