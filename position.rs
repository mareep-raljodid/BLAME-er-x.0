use util;

pub struct Position {
    pub current_player: i8,
    pub x_pattern: u16,
    pub o_pattern: u16,
    pub pos_hash: u64,

    rand_hash: Vec<Vec<u64>>,
}

impl Position {
    pub fn new() -> Position {
        let mut rand_hash: Vec<Vec<u64>> = vec![vec![0; 3]; 9];
        for index in 0..9 {
            rand_hash[index][0] = util::generate_random_number();
            rand_hash[index][2] = util::generate_random_number();
        }

        Position {
            current_player: 1,
            o_pattern: 0,
            x_pattern: 0,
            pos_hash: 0,
            rand_hash: rand_hash,
        }
    }

    pub fn is_index_empty(&self, index: u16) -> bool {
        let index_bit_mask = 1 << index;
        (self.x_pattern & index_bit_mask) == 0 &&
        (self.o_pattern & index_bit_mask) == 0
    }

    pub fn change_player(&mut self) {
        self.current_player = -self.current_player;
    }

    pub fn do_move(&mut self, index: u16) {
        self.move_pattern(index);
        self.update_hash(index);
        self.change_player();
    }

    pub fn undo_move(&mut self, index: u16) {
        self.change_player();
        self.update_hash(index);
        self.move_pattern(index);
    }

    fn move_pattern(&mut self, index: u16) {
        if self.current_player > 0 {
            self.x_pattern = util::move_pattern(self.x_pattern, index);
        } else {
            self.o_pattern = util::move_pattern(self.o_pattern, index);
        }
    }

    fn update_hash(&mut self, index: u16) {
        self.pos_hash = self.pos_hash ^ self.rand_hash[index as usize][(self.current_player + 1) as usize];
    }
}

#[cfg(test)]
mod tests {
    use position::Position;

    #[test]
    fn test_do_move_then_undo_move() {
        let mut pos = Position::new();

        assert_eq!(pos.x_pattern, 0);
        assert_eq!(pos.o_pattern, 0);
        assert_eq!(pos.current_player, 1);
        assert!(pos.is_index_empty(0));

        pos.do_move(1);
        assert_eq!(pos.x_pattern, 2);
        assert_eq!(pos.o_pattern, 0);
        assert_eq!(pos.current_player, -1);
        assert!(pos.is_index_empty(0));
        assert!(!pos.is_index_empty(1));

        pos.undo_move(1);
        assert_eq!(pos.x_pattern, 0);
        assert_eq!(pos.o_pattern, 0);
        assert_eq!(pos.current_player, 1);
        assert!(pos.is_index_empty(0));
        assert!(pos.is_index_empty(1));

        pos.do_move(0);
        pos.do_move(3);
        pos.do_move(1);
        pos.do_move(4);
        pos.do_move(2);
        pos.do_move(5);
        assert_eq!(pos.x_pattern, (1 << 0) | (1 << 1) | (1 << 2));
        assert_eq!(pos.o_pattern, (1 << 3) | (1 << 4) | (1 << 5));
        assert_eq!(pos.current_player, 1);
    }

    #[test]
    fn test_hash_update() {
        let mut pos = Position::new();
        assert_eq!(pos.pos_hash, 0);

        pos.do_move(1);
        let hash1 = pos.pos_hash;
        assert!(pos.pos_hash != 0);

        pos.undo_move(1);
        assert_eq!(pos.pos_hash, 0);

        pos.do_move(1);
        assert_eq!(pos.pos_hash, hash1);
    }
}
