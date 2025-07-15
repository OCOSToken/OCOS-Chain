//! OCOS-Chain: Ledger Executor Module
//!
//! Responsible for deterministic execution of blocks and transactions,
//! state transition application, gas accounting, and receipt generation.

use crate::ledger::{block::Block, transaction::SignedTransaction, state::{State, AccountState, StateUpdate}, receipt::{Receipt, EventLog}};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct ExecutionResult {
    pub state_update: StateUpdate,
    pub receipts: Vec<Receipt>,
    pub gas_used: u64,
    pub errors: Vec<ExecutionError>,
}

#[derive(Debug)]
pub enum ExecutionError {
    InvalidSignature,
    InsufficientBalance,
    InvalidNonce,
    ContractError(String),
    Unknown,
}

/// Executor: Processes blocks & txs, applies results to state
pub struct Executor;

impl Executor {
    /// Execute a block (returns cumulative result)
    pub fn execute_block(
        state: &mut State,
        block: &Block,
    ) -> ExecutionResult {
        let mut state_update = StateUpdate { account_deltas: vec![], storage_deltas: vec![] };
        let mut receipts = vec![];
        let mut gas_used = 0;
        let mut errors = vec![];

        // Example: iterate all transactions in the block
        for tx in &block.transactions {
            match Self::execute_transaction(state, tx) {
                Ok((delta, receipt, gas)) => {
                    state_update.account_deltas.push((tx.sender.clone(), delta));
                    receipts.push(receipt);
                    gas_used += gas;
                }
                Err(err) => {
                    errors.push(err);
                }
            }
        }

        ExecutionResult {
            state_update,
            receipts,
            gas_used,
            errors,
        }
    }

    /// Execute a single transaction (returns account delta, receipt, gas used)
    pub fn execute_transaction(
        state: &mut State,
        tx: &SignedTransaction,
    ) -> Result<(AccountDelta, Receipt, u64), ExecutionError> {
        // Simple transfer logic for demo
        let sender_addr = &tx.sender;
        let recipient_addr = &tx.recipient;
        let amount = tx.amount;
        let gas = tx.gas_limit;

        // Check signature (pseudo)
        // if !verify_sig(tx) { return Err(ExecutionError::InvalidSignature); }

        // Check sender account
        let sender_account = state.get_account_mut(sender_addr)
            .ok_or(ExecutionError::InvalidSignature)?;

        if sender_account.balance < amount + gas as u128 {
            return Err(ExecutionError::InsufficientBalance);
        }

        if tx.nonce != sender_account.nonce + 1 {
            return Err(ExecutionError::InvalidNonce);
        }

        // Update sender
        sender_account.balance -= amount + gas as u128;
        sender_account.nonce += 1;

        // Update recipient
        let recipient_account = state.get_account_mut(recipient_addr)
            .unwrap_or_else(|| {
                state.update_account(recipient_addr.clone(), AccountState::new(0));
                state.get_account_mut(recipient_addr).unwrap()
            });
        recipient_account.balance += amount;

        // (Smart contract calls, event logs, fee burning, etc. can be added here.)

        // Prepare receipt
        let receipt = Receipt {
            tx_hash: tx.hash.clone(),
            status: true,
            gas_used: gas,
            logs: vec![EventLog {
                address: recipient_addr.clone(),
                topics: vec![],
                data: vec![],
            }],
        };

        // Delta for audit/state trace
        let delta = AccountDelta::BalanceChange(amount as i128);

        Ok((delta, receipt, gas))
    }
}
