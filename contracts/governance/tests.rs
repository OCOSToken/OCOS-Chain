//! OCOS-Chain: Governance Module Tests
//!
//! Tests proposal lifecycle, voting logic, delegation, council, storage, and error handling.

use crate::contracts::governance::{
    types::{ProposalKind, ProposalStatus, VoteOption},
    proposal::Proposal,
    config::GovernanceConfig,
    weighted_vote::WeightedVoting,
    storage::{ProposalStorage, VoteStorage},
    error::GovernanceError,
};

fn dummy_proposal(id: u64, now: u64) -> Proposal {
    Proposal::new(
        id,
        1001, // proposer
        ProposalKind::ConfigUpdate { key: "min_voting_period".to_string() },
        "Change voting period".to_string(),
        now,
        now,
        now + 86_400,
        None,
    )
}

#[test]
fn test_proposal_lifecycle() {
    let now = 1_650_000_000;
    let mut proposal_storage = ProposalStorage::default();
    let mut proposal = dummy_proposal(1, now);

    assert_eq!(proposal.status, ProposalStatus::Pending);
    assert!(proposal_storage.add(proposal.clone()).is_ok());

    proposal.approve(now + 10, &mut vec![]);
    assert_eq!(proposal.status, ProposalStatus::Approved);

    assert!(proposal_storage.mark_as_passed(1).is_ok());
    assert_eq!(proposal_storage.get(1).unwrap().status, ProposalStatus::Approved);

    proposal.reject(now + 20, &mut vec![]);
    assert_eq!(proposal.status, ProposalStatus::Rejected);

    assert!(proposal_storage.mark_as_failed(1).is_ok());
    assert_eq!(proposal_storage.get(1).unwrap().status, ProposalStatus::Failed);
}

#[test]
fn test_weighted_voting_cast_and_tally() {
    let mut vote_storage = VoteStorage::default();
    let voting = WeightedVoting::new(100, 60);

    // Alice votes 40, Bob votes 70
    let alice = 2001;
    let bob = 2002;

    let mut events = vec![];
    assert!(voting.cast_vote(alice, 1, 40, VoteOption::Yes, &mut vote_storage, &mut events).is_ok());
    assert!(voting.cast_vote(bob, 1, 70, VoteOption::No, &mut vote_storage, &mut events).is_ok());

    let (yes, total) = vote_storage.tally_votes(1, VoteOption::Yes);
    assert_eq!(yes, 40);
    assert_eq!(total, 110);

    // Tally (should fail quorum)
    let mut proposal_storage = ProposalStorage::default();
    let mut events2 = vec![];
    let quorum_result = voting.tally(1, &vote_storage, &mut proposal_storage, &mut events2);
    assert!(quorum_result.is_err());
    assert_eq!(quorum_result.unwrap_err(), GovernanceError::QuorumNotMet);
}

#[test]
fn test_double_voting_and_errors() {
    let mut vote_storage = VoteStorage::default();
    let voting = WeightedVoting::new(10, 6);

    let alice = 3001;

    let mut events = vec![];
    assert!(voting.cast_vote(alice, 2, 5, VoteOption::Yes, &mut vote_storage, &mut events).is_ok());

    // Alice tries to vote again
    let result = voting.cast_vote(alice, 2, 5, VoteOption::No, &mut vote_storage, &mut events);
    assert!(matches!(result, Err(GovernanceError::AlreadyVoted)));
}

#[test]
fn test_proposal_config_update() {
    let mut config = GovernanceConfig::default();
    assert_eq!(config.min_voting_period, 60 * 60 * 24);

    let result = config.update("min_voting_period", 1000);
    assert!(result.is_ok());
    assert_eq!(config.min_voting_period, 1000);

    let err = config.update("unknown_param", 1_000_000);
    assert!(matches!(err, Err(GovernanceError::InvalidParameter)));
}
