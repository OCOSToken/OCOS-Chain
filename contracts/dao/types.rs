/// DAO on-chain configuration (sample)
#[derive(Debug, Clone)]
pub struct DaoConfig {
    pub quorum: u64,                 // Minimum votes for proposal approval
    pub voting_period_secs: u64,     // Voting window in seconds
    pub min_stake: u128,             // Minimum stake to participate
    pub max_active_proposals: u32,   // Max proposals a member can create concurrently
    pub emergency_mode: bool,        // If true, DAO runs in restricted mode (pause, emergency powers)
    // Extend with more governance parameters as needed
}
