//! OCOS-Chain: Identity Module Tests
//!
//! Unit & integration tests for profiles, DIDs, SBTs, KYC, recovery, reputation, group, attestation, and governance.

use crate::identity::{
    profile::IdentityProfile,
    did::DIDRegistry,
    soulbound::SBTRegistry,
    kyc::{KYCRegistry, KYCStatus},
    recovery::RecoveryRegistry,
    reputation::ReputationRegistry,
    group::{GroupRegistry, GroupType},
    attestation::AttestationRegistry,
    governance::{IdentityGovernance, IdentityProposalKind, ProposalStatus},
    types::*,
    error::IdentityError,
};

fn dummy_address(n: u8) -> Address {
    let mut arr = [0u8; 20];
    arr[0] = n;
    arr
}

#[test]
fn test_profile_and_did_registry() {
    let mut profiles = std::collections::HashMap::new();
    let identity_id = 1;
    let addr = dummy_address(1);
    let profile = IdentityProfile {
        identity_id,
        address: addr,
        nickname: Some("Alice".to_string()),
        avatar_url: None,
        email: None,
        created_at: 42,
    };
    profiles.insert(identity_id, profile);

    let mut did_registry = DIDRegistry::default();
    let did = "did:ocos:alice".to_string();
    let controller = addr;
    let vm = VerificationMethod {
        key_type: "Ed25519".to_string(),
        public_key: vec![1,2,3],
        fragment: None,
    };
    let service = ServiceEndpoint {
        id: "svc".to_string(),
        service_type: "msg".to_string(),
        endpoint: "https://a".to_string(),
    };
    assert!(did_registry.create_did(did.clone(), controller, vm, vec![service]).is_ok());
    assert!(did_registry.resolve(&did).is_some());
}

#[test]
fn test_soulbound_token_mint_and_revoke() {
    let mut sbt_registry = SBTRegistry::default();
    let sbt_id = 1001;
    let owner = dummy_address(2);
    let meta = SBTMetadata {
        sbt_id,
        name: "KYC Pass".to_string(),
        description: "KYC Verified".to_string(),
        issued_by: owner,
        issued_at: 77,
        expiry: None,
        attributes: vec![("level".to_string(), "basic".to_string())],
    };
    assert!(sbt_registry.mint(sbt_id, owner, meta).is_ok());
    assert_eq!(sbt_registry.tokens_of(owner), vec![sbt_id]);
    assert!(sbt_registry.revoke(sbt_id, owner).is_ok());
    assert!(sbt_registry.is_revoked(sbt_id));
}

#[test]
fn test_kyc_registry_and_status() {
    let mut kyc = KYCRegistry::default();
    let identity_id = 11;
    assert!(kyc.request_verification(identity_id).is_ok());
    let provider = dummy_address(7);
    assert!(kyc.approve(identity_id, provider, 1_000_000).is_ok());
    assert_eq!(kyc.get_status(identity_id), Some(&KYCStatus::Approved));
    assert!(kyc.revoke(identity_id, provider).is_ok());
    assert_eq!(kyc.get_status(identity_id), Some(&KYCStatus::Revoked));
}

#[test]
fn test_recovery_workflow() {
    let mut recovery = RecoveryRegistry::default();
    let identity_id = 5001;
    let owner = dummy_address(3);
    let guardian1 = dummy_address(4);
    let guardian2 = dummy_address(5);

    let mut guardians = std::collections::HashSet::new();
    guardians.insert(guardian1);
    guardians.insert(guardian2);
    assert!(recovery.set_recovery(identity_id, owner, guardians.clone(), 2).is_ok());
    assert!(recovery.request_recovery(identity_id, guardian1).is_ok());
    assert!(recovery.approve_recovery(identity_id, guardian2).is_ok());
    assert!(recovery.complete_recovery(identity_id, dummy_address(9)).is_ok());
}

#[test]
fn test_reputation_registry() {
    let mut reputation = ReputationRegistry::default();
    let addr = dummy_address(11);
    reputation.set(addr, 100);
    reputation.reward(addr, 25);
    reputation.slash(addr, 30);
    assert_eq!(reputation.get(addr), 95);
    reputation.peer_review(addr, false, 40);
    assert_eq!(reputation.get(addr), 55);
}

#[test]
fn test_group_create_and_membership() {
    let mut group_registry = GroupRegistry::default();
    let group_id = 42;
    let creator = dummy_address(7);
    assert!(group_registry.create_group(group_id, GroupType::DAO, "OCOS DAO".to_string(), "desc".to_string(), creator, true, 111).is_ok());
    let member = dummy_address(12);
    assert!(group_registry.add_member(group_id, member, creator).is_ok());
    assert!(group_registry.is_member(group_id, member));
    assert!(group_registry.remove_member(group_id, member, creator).is_ok());
    assert!(!group_registry.is_member(group_id, member));
}

#[test]
fn test_attestation_registry() {
    let mut attestation = AttestationRegistry::default();
    let identity_id = 2222;
    let attestor = dummy_address(5);
    assert!(attestation.add(identity_id, attestor, "KYC OK".to_string()).is_ok());
    let attests = attestation.list(dummy_address(5));
    assert!(!attests.is_empty());
}

#[test]
fn test_identity_governance_proposal_flow() {
    let mut gov = IdentityGovernance::default();
    let proposal_id = 8888;
    let creator = dummy_address(1);

    assert!(gov.submit_proposal(
        proposal_id,
        IdentityProposalKind::UpdateConfig { key: "limit".to_string(), value: 42 },
        creator,
        100,
        200
    ).is_ok());

    let voter = dummy_address(2);
    assert!(gov.vote(proposal_id, voter, true, 150).is_ok());
    assert!(gov.execute(proposal_id, 201).is_ok());
    assert_eq!(gov.proposals.get(&proposal_id).unwrap().status, ProposalStatus::Executed);
}
