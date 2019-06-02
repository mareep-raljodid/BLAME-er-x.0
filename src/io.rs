use position::Position;

const O_CHAR: char = 'O';
const X_CHAR: char = 'X';
const EMPTY_CHAR: char = '+';

pub enum DoUserMoveResult {
    Ok,
    Err,
}

pub enum GetIndexCodeResult {
    Ok(u16),
    Err,
}

pub fn get_square_string(index_code: u16) -> String {
    match index_code {
        0 => "a1".to_owned(),
        1 => "b1".to_owned(),
        2 => "c1".to_owned(),
        3 => "a2".to_owned(),
        4 => "b2".to_owned(),
        5 => "c2".to_owned(),
        6 => "a3".to_owned(),
        7 => "b3".to_owned(),
        8 => "c3".to_owned(),
        _ => panic!("invalid square index code {}", index_code),
    }
}

pub fn get_index_code(square_string: &str) -> GetIndexCodeResult {
    match square_string.trim() {
        "a1" => GetIndexCodeResult::Ok(0),
        "b1" => GetIndexCodeResult::Ok(1),
        "c1" => GetIndexCodeResult::Ok(2),
        "a2" => GetIndexCodeResult::Ok(3),
        "b2" => GetIndexCodeResult::Ok(4),
        "c2" => GetIndexCodeResult::Ok(5),
        "a3" => GetIndexCodeResult::Ok(6),
        "b3" => GetIndexCodeResult::Ok(7),
        "c3" => GetIndexCodeResult::Ok(8),
        _ => GetIndexCodeResult::Err,
    }
}

fn convert_piece_to_symbol(pos: &Position, index: u16) -> char {
    let index_bit_mask = 1 << index;
    if pos.x_pattern & index_bit_mask != 0 {
        return X_CHAR;
    }

    if pos.o_pattern & index_bit_mask != 0 {
        return O_CHAR;
    }

    EMPTY_CHAR
}

fn generate_board_string(pos: &Position) -> String {
    let get_piece_char = |square_string| match get_index_code(square_string) {
        GetIndexCodeResult::Ok(index) => convert_piece_to_symbol(pos, index),
        GetIndexCodeResult::Err => panic!("invalid system state"),
    };

    let row3_string = format!("3 {} {} {}", get_piece_char("a3"), get_piece_char("b3"), get_piece_char("c3"));
    let row2_string = format!("2 {} {} {}", get_piece_char("a2"), get_piece_char("b2"), get_piece_char("c2"));
    let row1_string = format!("1 {} {} {}", get_piece_char("a1"), get_piece_char("b1"), get_piece_char("c1"));
    let colum_notation_string = "  a b c";

    format!("{}\n{}\n{}\n{}\n", row3_string, row2_string, row1_string, colum_notation_string).to_owned()
}

pub fn print_board(position: &Position) {
    println!("{}", generate_board_string(position));
}

pub fn do_user_move(position: &mut Position, square_string: &str) -> DoUserMoveResult {
    match get_index_code(square_string) {
        GetIndexCodeResult::Ok(index) => {
            if !position.is_index_empty(index) {
                return DoUserMoveResult::Err
            }

            position.do_move(index);
            DoUserMoveResult::Ok
        },
        GetIndexCodeResult::Err => DoUserMoveResult::Err,
    }
}

#[cfg(test)]
mod tests {
    use position::Position;
    use io;

    #[test]
    fn test_generate_board_string() {
        let mut pos = Position::new();
        assert_eq!("3 + + +\n2 + + +\n1 + + +\n  a b c\n", io::generate_board_string(&pos));
        pos.do_move(0);
        pos.do_move(1);
        assert_eq!("3 + + +\n2 + + +\n1 X O +\n  a b c\n", io::generate_board_string(&pos));
    }

    #[test]
    fn test_do_user_move() {
        let mut pos = Position::new();
        assert_eq!("3 + + +\n2 + + +\n1 + + +\n  a b c\n", io::generate_board_string(&pos));
        io::do_user_move(&mut pos, "a1");
        assert_eq!("3 + + +\n2 + + +\n1 X + +\n  a b c\n", io::generate_board_string(&pos));
    }
}
