//! OCOS-Chain: Virtual Memory Module
//!
//! This module provides a safe, sandboxed linear memory abstraction used by the VM engine.
//! It simulates WASM-style memory with bounds checking and zeroed pages, ensuring
//! isolation and deterministic state transitions.

const DEFAULT_MEMORY_SIZE: usize = 64 * 1024; // 64 KB default

/// A simple linear memory model with bounded access
#[derive(Debug, Clone)]
pub struct VmMemory {
    data: Vec<u8>,
}

impl VmMemory {
    /// Create a new memory instance with a given size (zeroed)
    pub fn new(size: usize) -> Self {
        VmMemory {
            data: vec![0u8; size],
        }
    }

    /// Create memory with default size (64KB)
    pub fn default() -> Self {
        Self::new(DEFAULT_MEMORY_SIZE)
    }

    /// Read bytes from memory
    pub fn read(&self, offset: usize, length: usize) -> Result<Vec<u8>, MemoryError> {
        if offset + length > self.data.len() {
            return Err(MemoryError::OutOfBounds);
        }
        Ok(self.data[offset..offset + length].to_vec())
    }

    /// Write bytes to memory
    pub fn write(&mut self, offset: usize, input: &[u8]) -> Result<(), MemoryError> {
        if offset + input.len() > self.data.len() {
            return Err(MemoryError::OutOfBounds);
        }
        self.data[offset..offset + input.len()].copy_from_slice(input);
        Ok(())
    }

    /// Zero out the entire memory
    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    /// Return a reference to the internal memory buffer
    pub fn buffer(&self) -> &[u8] {
        &self.data
    }

    /// Return total memory size
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

/// Memory operation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    OutOfBounds,
}
