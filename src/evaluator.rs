use position::Position;

pub const SCORE_BOUNDARY:i32 = 100;
pub const CLOSE_WIN_REWARD:i32 = 10;

const WINNING_PATTERN_LIST: &[u16] = &[
    (1 << 0) | (1 << 1) | (1 << 2),
    (1 << 3) | (1 << 4) | (1 << 5),
    (1 << 6) | (1 << 7) | (1 << 8),

    (1 << 0) | (1 << 3) | (1 << 6),
    (1 << 1) | (1 << 4) | (1 << 7),
    (1 << 2) | (1 << 5) | (1 << 8),

    (1 << 0) | (1 << 4) | (1 << 8),
    (1 << 2) | (1 << 4) | (1 << 6),
];

const CLOSE_TO_WIN_PATTERN_PAIR_LIST: &[(u16, u16)] = &[
    ((1 << 0) | (1 << 1),  (1 << 2)),
    ((1 << 0) | (1 << 2),  (1 << 1)),
    ((1 << 1) | (1 << 2),  (1 << 0)),

    ((1 << 3) | (1 << 4),  (1 << 5)),
    ((1 << 3) | (1 << 5),  (1 << 4)),
    ((1 << 4) | (1 << 5),  (1 << 3)),

    ((1 << 6) | (1 << 7),  (1 << 8)),
    ((1 << 6) | (1 << 8),  (1 << 7)),
    ((1 << 7) | (1 << 8),  (1 << 6)),

    ((1 << 0) | (1 << 3),  (1 << 6)),
    ((1 << 0) | (1 << 6),  (1 << 3)),
    ((1 << 3) | (1 << 6),  (1 << 0)),

    ((1 << 1) | (1 << 4),  (1 << 7)),
    ((1 << 1) | (1 << 7),  (1 << 4)),
    ((1 << 4) | (1 << 7),  (1 << 1)),

    ((1 << 2) | (1 << 5),  (1 << 8)),
    ((1 << 2) | (1 << 8),  (1 << 5)),
    ((1 << 5) | (1 << 8),  (1 << 2)),

    ((1 << 0) | (1 << 4),  (1 << 8)),
    ((1 << 0) | (1 << 8),  (1 << 4)),
    ((1 << 4) | (1 << 8),  (1 << 0)),

    ((1 << 2) | (1 << 4),  (1 << 6)),
    ((1 << 2) | (1 << 6),  (1 << 4)),
    ((1 << 4) | (1 << 6),  (1 << 2)),
];

pub fn evaluate(position: &Position) -> i32 {
    for pattern in WINNING_PATTERN_LIST {
        if position.x_pattern & pattern == pattern.to_owned() {
            return SCORE_BOUNDARY;
        }

        if position.o_pattern & pattern == pattern.to_owned() {
            return -SCORE_BOUNDARY;
        }
    }

    let mut score = 0;
    for pattern_pair in CLOSE_TO_WIN_PATTERN_PAIR_LIST {
        let close_win_pattern = pattern_pair.0;
        let defend_pattern = pattern_pair.1;
        if (position.x_pattern & close_win_pattern == close_win_pattern.to_owned())
        && position.o_pattern & defend_pattern == 0 {
            score = score + CLOSE_WIN_REWARD;
        }

        if (position.o_pattern & close_win_pattern == close_win_pattern.to_owned())
        && position.x_pattern & defend_pattern == 0 {
            score = score - CLOSE_WIN_REWARD;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use position::Position;
    use evaluator;

    #[test]
    fn test_evaluate_win() {
        let mut pos = Position::new();
        assert_eq!(evaluator::evaluate(&pos), 0);

        pos.do_move(0);
        pos.do_move(3);
        pos.do_move(1);
        pos.do_move(5);
        pos.do_move(2);
        assert_eq!(evaluator::evaluate(&pos), 100);
    }

    #[test]
    fn test_evaluate_close_to_win() {
        let mut pos = Position::new();
        assert_eq!(evaluator::evaluate(&pos), 0);

        pos.do_move(0);
        pos.do_move(3);
        pos.do_move(1);
        assert_eq!(evaluator::evaluate(&pos), 10);
        pos.do_move(5);
        assert_eq!(evaluator::evaluate(&pos), 0);
    }
}
