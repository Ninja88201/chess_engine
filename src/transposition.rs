use std::collections::HashMap;

use chess_lib::Move;

pub enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}
pub struct TTEntry {
    pub depth: i32,
    pub flag: TTFlag,
    pub score: i32,
    pub best_move: Option<Move>
}
pub struct TranspositionTable(HashMap<u64, TTEntry>);
impl TranspositionTable {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: u64) -> Option<&TTEntry> {
        self.0.get(&key)
    }

    pub fn insert(&mut self, key: u64, entry: TTEntry) {
        self.0.insert(key, entry);
    }
}