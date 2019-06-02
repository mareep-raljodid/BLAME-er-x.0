extern crate rand;

pub fn move_pattern(pattern: u16, index: u16) -> u16 {
    pattern ^ (1 << index)
}

pub fn generate_random_number() -> u64 {
	rand::random::<u64>()
}

#[cfg(test)]
mod tests {
    use util;

    #[test]
    fn test_move_pattern() {
        assert_eq!(1 << 0 | 1 << 1, util::move_pattern(1, 1));
    }
}
