//! OCOS-Chain: Syscall Handler for VM
//!
//! Provides a unified interface for host functions accessible from within
//! smart contracts and DAO logic. Implements state access, logging, balance
//! queries, and inter-contract communication.

use std::collections::HashMap;

/// Simulated key-value contract storage (for demonstration)
type Storage = HashMap<Vec<u8>, Vec<u8>>;

/// A trait defining system calls available to contracts
pub trait Syscall {
    fn log(&mut self, topic: &str, data: &[u8]);
    fn read_storage(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn write_storage(&mut self, key: &[u8], value: &[u8]);
    fn get_balance(&self, address: &[u8]) -> u128;
    fn emit_event(&mut self, name: &str, payload: &[u8]);
    fn call_contract(&mut self, address: &[u8], input: &[u8]) -> Result<Vec<u8>, SyscallError>;
}

/// Default implementation of SyscallHandler
#[derive(Default)]
pub struct SyscallHandler {
    pub storage: Storage,
    pub logs: Vec<(String, Vec<u8>)>,
    pub events: Vec<(String, Vec<u8>)>,
}

impl Syscall for SyscallHandler {
    fn log(&mut self, topic: &str, data: &[u8]) {
        self.logs.push((topic.to_string(), data.to_vec()));
    }

    fn read_storage(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.get(key).cloned()
    }

    fn write_storage(&mut self, key: &[u8], value: &[u8]) {
        self.storage.insert(key.to_vec(), value.to_vec());
    }

    fn get_balance(&self, address: &[u8]) -> u128 {
        // Placeholder: In production, query on-chain state
        1_000_000
    }

    fn emit_event(&mut self, name: &str, payload: &[u8]) {
        self.events.push((name.to_string(), payload.to_vec()));
    }

    fn call_contract(&mut self, address: &[u8], input: &[u8]) -> Result<Vec<u8>, SyscallError> {
        // Placeholder: In production, perform real contract call
        Ok(vec![42]) // Dummy return
    }
}

/// Errors returned by syscall functions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyscallError {
    InvalidTarget,
    Revert,
    OutOfGas,
    Unknown,
}
