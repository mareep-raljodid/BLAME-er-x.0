use position::Position;
use evaluator;
use move_generator;
use transpos_table::TransposTable;
use transpos_table::TransposTableSearchResult;

pub enum FindMoveResult {
    Found (u16),
    None,
}

pub fn find_best_move(position: &mut Position) -> FindMoveResult {
    let score_side_flag = position.current_player as i32;
    let low_bound = -evaluator::SCORE_BOUNDARY;
    let high_bound = evaluator::SCORE_BOUNDARY;

    let mut best_move = FindMoveResult::None;

    let mut best_score = low_bound * score_side_flag;
    let move_list = move_generator::generate_move_list(position);
    if move_list.len() == 0 {
        return FindMoveResult::None;
    }

    let mut table = TransposTable::new(10000);
    let search_depth = move_list.len() as u32 - 1;
    for m in move_list {
        position.do_move(m);
        let score = search_alpha_beta(position, &mut table, search_depth, high_bound * score_side_flag, low_bound * score_side_flag);
        position.undo_move(m);

        if score * score_side_flag >= best_score * score_side_flag {
            best_score = score;
            best_move = FindMoveResult::Found(m);
        }
    }

    best_move
}

fn search_alpha_beta(position: &mut Position, table: &mut TransposTable, depth: u32, low_bound: i32, high_bound: i32) -> i32 {
    match table.find(position.pos_hash, position.x_pattern, position.o_pattern, position.current_player, depth) {
        TransposTableSearchResult::Found(score) => { return score; },
        _ => {},
    }

    let score_side_flag = position.current_player as i32;
    let current_pos_score = evaluator::evaluate(position);

    if (current_pos_score.abs() == evaluator::SCORE_BOUNDARY) || (depth == 0) {
        table.insert(position.pos_hash, position.x_pattern, position.o_pattern, position.current_player, depth, current_pos_score);
        return current_pos_score;
    }

    let move_list = move_generator::generate_move_list(position);
    if move_list.len() == 0 {
        table.insert(position.pos_hash, position.x_pattern, position.o_pattern, position.current_player, depth, current_pos_score);
        return current_pos_score;
    }

    let mut best_score = low_bound;
    for m in move_list {
        position.do_move(m);
        let score = search_alpha_beta(position, table, depth - 1, high_bound, best_score);
        position.undo_move(m);

        if score * score_side_flag >= high_bound * score_side_flag {
            table.insert(position.pos_hash, position.x_pattern, position.o_pattern, position.current_player, depth, score);
            return high_bound;
        }

        if score * score_side_flag > best_score * score_side_flag {
            best_score = score;
        }
    }

    table.insert(position.pos_hash, position.x_pattern, position.o_pattern, position.current_player, depth, best_score);
    best_score
}

#[cfg(test)]
mod tests {
    use position::Position;
    use transpos_table::TransposTable;
    use search;

    #[test]
    fn test_search_alpha_beta() {
        let mut pos = Position::new();
        let mut transpos_table = TransposTable::new(4096);
        pos.do_move(0);
        pos.do_move(1);
        pos.do_move(4);
        assert_eq!(100, search::search_alpha_beta(&mut pos, &mut transpos_table, 5, 100, -100));
    }

    #[test]
    fn test_find_best_move() {
        let mut pos = Position::new();
        pos.do_move(0);
        pos.do_move(1);
        match search::find_best_move(&mut pos) {
            search::FindMoveResult::Found(index) => {
                assert_eq!(6, index);
            },
            search::FindMoveResult::None => {
                assert!(false);
            }
        }
    }
}
