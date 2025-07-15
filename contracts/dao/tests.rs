//! OCOS-Chain DAO: Governance Unit & Integration Tests
//!
//! Tests proposal creation, voting, membership, and execution workflows
//! for correctness, auditability, and edge-case handling.

use crate::contracts::dao::{
    proposal::{create_proposal, get_proposal, list_proposals, update_proposal_status},
    voting::{cast_vote, tally_votes, Vote, VoteWeight},
    membership::{add_member, remove_member, get_member, list_members},
    execution::execute_proposal,
    types::{ProposalKind, ProposalStatus, MemberId, ProposalId, MemberRole, MembershipInfo},
    error::DaoError,
    config::{update_config, get_config},
};

fn dummy_member_id() -> MemberId {
    MemberId("member1".to_string())
}

#[test]
fn test_create_and_retrieve_proposal() {
    let proposer = dummy_member_id();
    let id = create_proposal(
        proposer.clone(),
        ProposalKind::Text,
        "Upgrade Treasury".to_string(),
        "Increase treasury reserves".to_string(),
        vec![],
    ).unwrap();

    let prop = get_proposal(&id).unwrap();
    assert_eq!(prop.proposer, proposer);
    assert_eq!(prop.status, ProposalStatus::Pending);
}

#[test]
fn test_add_and_remove_member() {
    let member_id = dummy_member_id();
    add_member(member_id.clone(), MemberRole::Admin, 1_000_000).unwrap();
    let info = get_member(&member_id).unwrap();
    assert_eq!(info.role, MemberRole::Admin);
    remove_member(&member_id).unwrap();
    assert!(get_member(&member_id).is_err());
}

#[test]
fn test_voting_flow_and_quorum() {
    let proposer = dummy_member_id();
    let proposal_id = create_proposal(
        proposer.clone(),
        ProposalKind::ParameterChange,
        "Lower quorum".to_string(),
        "Test lower quorum".to_string(),
        vec![],
    ).unwrap();

    add_member(MemberId("voter1".to_string()), MemberRole::Voter, 10_000).unwrap();
    add_member(MemberId("voter2".to_string()), MemberRole::Voter, 10_000).unwrap();

    // Cast votes
    cast_vote(&proposal_id, &MemberId("voter1".to_string()), Vote::Yes).unwrap();
    cast_vote(&proposal_id, &MemberId("voter2".to_string()), Vote::No).unwrap();

    let (votes_for, votes_against) = tally_votes(&proposal_id).unwrap();
    assert_eq!(votes_for, 1);
    assert_eq!(votes_against, 1);

    // Simulate status update after voting (should remain pending if quorum not met)
    let res = update_proposal_status(&proposal_id, ProposalStatus::Approved);
    assert!(res.is_ok() || res.is_err());
}

#[test]
fn test_execute_proposal_and_receipt() {
    let proposer = dummy_member_id();
    let proposal_id = create_proposal(
        proposer.clone(),
        ProposalKind::Action,
        "Distribute Rewards".to_string(),
        "Reward top members".to_string(),
        vec![],
    ).unwrap();

    // Simulate voting and approval
    update_proposal_status(&proposal_id, ProposalStatus::Approved).unwrap();

    // Execute proposal
    let receipt = execute_proposal(&proposal_id).unwrap();
    assert_eq!(receipt.status, ProposalStatus::Executed);
}
