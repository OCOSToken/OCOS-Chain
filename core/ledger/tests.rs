//! OCOS-Chain: Ledger Core Tests
//!
//! Unit and integration tests for ledger blocks, transactions, state management, and receipts.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::{
        block::Block, 
        transaction::{SignedTransaction, Transaction}, 
        state::{State, AccountState}, 
        receipt::{Receipt, EventLog},
        block_store::BlockStore,
        tx_pool::TxPool,
        executor::Executor,
        snapshot::Snapshot,
        history::History,
    };

    fn dummy_address(val: u8) -> Vec<u8> { vec![val; 20] }
    fn dummy_tx(sender: u8, recipient: u8, nonce: u64, amount: u128) -> SignedTransaction {
        SignedTransaction {
            sender: dummy_address(sender),
            recipient: dummy_address(recipient),
            nonce,
            amount,
            gas_limit: 1_000,
            hash: vec![nonce as u8; 32],
            signature: vec![],
        }
    }

    #[test]
    fn test_state_account_update() {
        let mut state = State::new();
        let addr = dummy_address(1);
        let acc = AccountState::new(1000);
        state.update_account(addr.clone(), acc);
        assert_eq!(state.get_account(&addr).unwrap().balance, 1000);
    }

    #[test]
    fn test_tx_pool_add_and_evict() {
        let mut pool = TxPool::new(1); // TTL 1s
        let tx = dummy_tx(2, 3, 1, 100);
        pool.add_transaction(tx.clone()).unwrap();
        assert_eq!(pool.len(), 1);
        std::thread::sleep(std::time::Duration::from_secs(2));
        pool.evict_expired();
        assert!(pool.is_empty());
    }

    #[test]
    fn test_block_store_insert_and_rollback() {
        let mut store = BlockStore::new();
        let block1 = Block { header: Default::default(), transactions: vec![] };
        let block2 = Block { header: Default::default(), transactions: vec![] };
        store.insert_block(block1, vec![1;32], 1);
        store.insert_block(block2, vec![2;32], 2);
        assert_eq!(store.len(), 2);
        store.rollback_to_height(1);
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_executor_successful_transfer() {
        let mut state = State::new();
        let sender = dummy_address(10);
        let recipient = dummy_address(20);
        state.update_account(sender.clone(), AccountState::new(2000));
        state.update_account(recipient.clone(), AccountState::new(500));
        let tx = dummy_tx(10, 20, 1, 1000);

        let result = Executor::execute_transaction(&mut state, &tx).unwrap();
        assert_eq!(state.get_account(&sender).unwrap().balance, 2000 - 1000 - 1000);
        assert_eq!(state.get_account(&recipient).unwrap().balance, 500 + 1000);
        assert!(result.1.status);
    }

    #[test]
    fn test_receipt_and_eventlog() {
        let tx_hash = vec![0xab; 32];
        let logs = vec![
            EventLog::new(dummy_address(100), vec![vec![1,2,3]], vec![0x01, 0x02]),
        ];
        let receipt = Receipt::success(tx_hash.clone(), 21000, logs.clone());
        assert!(receipt.status);
        assert_eq!(receipt.tx_hash, tx_hash);
        assert_eq!(receipt.logs.len(), 1);
    }

    #[test]
    fn test_snapshot_restore() {
        let mut state = State::new();
        let addr = dummy_address(55);
        state.update_account(addr.clone(), AccountState::new(999));
        let snapshot = Snapshot::from_state(42, vec![9;32], &state);
        let restored = snapshot.restore_state();
        assert_eq!(restored.get_account(&addr).unwrap().balance, 999);
    }

    #[test]
    fn test_history_add_and_iter() {
        let mut hist = History::new();
        let txs = vec![
            dummy_tx(1,2,1,10),
            dummy_tx(2,3,2,20),
        ];
        hist.add_block(1, vec![0x01;32], txs.clone());
        assert_eq!(hist.get_block_hash(1).unwrap(), &vec![0x01;32]);
        assert_eq!(hist.get_block_txs(1).unwrap().len(), 2);
        let tx_hash = &txs[0].hash;
        assert!(hist.get_tx_by_hash(tx_hash).is_some());
    }
}
