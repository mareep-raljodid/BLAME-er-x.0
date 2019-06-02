use position::Position;

const BIT_MASK_LIST: &[(u16, u16)] = &[
    (0, 1 << 0),
    (1, 1 << 1),
    (2, 1 << 2),
    (3, 1 << 3),
    (4, 1 << 4),
    (5, 1 << 5),
    (6, 1 << 6),
    (7, 1 << 7),
    (8, 1 << 8),
];

pub fn generate_move_list(position: &Position) -> Vec<u16> {
    let mut move_list:Vec<u16> = Vec::new();
    for index in 0..BIT_MASK_LIST.len() {
        let bit_mask = BIT_MASK_LIST[index];
        if (position.x_pattern & bit_mask.1) != 0 {
            continue;
        }

        if (position.o_pattern & bit_mask.1) != 0 {
            continue;
        }

        move_list.push(bit_mask.0);
    }

    move_list
}

#[cfg(test)]
mod tests {
    use position::Position;
    use move_generator;

    #[test]
    fn test_generate_move_list() {
        let mut pos = Position::new();

        let move_list = move_generator::generate_move_list(&pos);
        assert_eq!(move_list.len(), 9);

        pos.do_move(1);
        let move_list1 = move_generator::generate_move_list(&pos);
        assert_eq!(move_list1.len(), 8);

        pos.do_move(2);
        pos.do_move(3);
        pos.do_move(8);
        let move_list1 = move_generator::generate_move_list(&pos);
        assert_eq!(move_list1.len(), 5);
    }
}
