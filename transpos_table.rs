pub struct TransposTable {
    index_base: usize,
    table: Vec<TransposTableEntry>,
}

impl TransposTable {
    pub fn new(size: usize) -> TransposTable {
        TransposTable {
            index_base: size - 1,
            table: vec![TransposTableEntry::new(); size],
        }
    }

    pub fn insert(&mut self, hash_key: u64, x_pattern: u16, o_pattern: u16, player: i8, depth: u32, score: i32) {
        let hash_address = hash_key as usize & self.index_base;
        let entry = &mut self.table[hash_address];
        if entry.key != hash_key {
            entry.update(hash_key, x_pattern, o_pattern, player, depth, score);
            return;
        }

        if entry.depth > depth {
            return;
        }

        entry.update(hash_key, x_pattern, o_pattern, player, depth, score);
    }

    pub fn find(&self, hash_key: u64, x_pattern: u16, o_pattern: u16, player: i8, depth: u32) -> TransposTableSearchResult {
        let hash_address = hash_key as usize & self.index_base;

        let entry = &self.table[hash_address];
        if (entry.key != hash_key)
        || (entry.player != player)
        || (entry.x_pattern != x_pattern)
        || (entry.o_pattern != o_pattern)
        || (entry.depth < depth) {
            return TransposTableSearchResult::None;
        }

        TransposTableSearchResult::Found(entry.score)
    }
}

#[derive (Clone)]
struct TransposTableEntry {
    key: u64,
    x_pattern: u16,
    o_pattern: u16,
    player: i8,
    depth: u32,
    score: i32,
}

impl TransposTableEntry {
    fn new() -> TransposTableEntry {
        TransposTableEntry {
            key: 0,
            x_pattern: 0,
            o_pattern: 0,
            player: 0,
            depth: 0,
            score: 0,
        }
    }

    fn update(&mut self, key: u64, x_pattern: u16, o_pattern: u16, player: i8, depth: u32, score: i32) {
        self.key = key;
        self.x_pattern = x_pattern;
        self.o_pattern = o_pattern;
        self.player = player;
        self.depth = depth;
        self.score = score;
    }
}

pub enum TransposTableSearchResult {
    Found (i32),
    None,
}


#[cfg(test)]
mod tests {
    use transpos_table::TransposTable;
    use transpos_table::TransposTableSearchResult;

    #[test]
    fn test_insert_and_search() {
        let mut table = TransposTable::new(4096);
        table.insert(91, 1000, 900, 1, 0, 100);

        match table.find(91, 1000, 900, 1, 0) {
            TransposTableSearchResult::Found(score) => assert_eq!(score, 100),
            TransposTableSearchResult::None => assert!(false),
        }

        match table.find(91, 1000, 900, 1, 1) {
            TransposTableSearchResult::Found(_) => assert!(false),
            TransposTableSearchResult::None => assert!(true),
        }

        match table.find(91, 200, 900, 1, 0) {
            TransposTableSearchResult::Found(_) => assert!(false),
            TransposTableSearchResult::None => assert!(true),
        }

        match table.find(91, 1000, 800, 1, 0) {
            TransposTableSearchResult::Found(_) => assert!(false),
            TransposTableSearchResult::None => assert!(true),
        }

        table.insert(1000001, 200, 199, -1, 6, 199);

        match table.find(1000001, 200, 199, -1, 6) {
            TransposTableSearchResult::Found(score) => assert_eq!(score, 199),
            TransposTableSearchResult::None => assert!(false),
        }
    }
}
