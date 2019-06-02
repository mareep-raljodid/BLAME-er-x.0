mod position;
mod search;
mod evaluator;
mod move_generator;
mod transpos_table;
mod io;
mod util;

use std::time::SystemTime;
use std::io::stdin;

use position::Position;
use search::FindMoveResult;
use io::DoUserMoveResult;

const SECS_TO_NANOS_RATIO: u64 = 1000000000;
const DONE_PATTERN: u16 = 1 | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8);

enum ProcessUserMoveResult {
    ContinueGame,
    EndGame,
    Retry,
}

enum DoComputerMoveResult {
    ContinueGame,
    EndGame,
}

enum ProcessUserSideSelectionResult {
    Ok,
    Err,
}

enum ReadUserInputResult {
    Ok(String),
    Err,
}

fn main() {
    let mut pos = Position::new();
    io::print_board(&pos);
    start_game_loop(&mut pos);
}

fn read_user_input() -> ReadUserInputResult {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => ReadUserInputResult::Ok(input),
        Err(_) => ReadUserInputResult::Err,
    }
}

fn start_game_loop(pos: &mut Position) {
    let mut game_started = false;
    loop {
        if !game_started {
            match process_user_side_selection(pos) {
                ProcessUserSideSelectionResult::Ok => {
                    game_started = true;
                },
                ProcessUserSideSelectionResult::Err => continue,
            }
        }

        println!("Your move: (ex. a1/b2/..)");
        match read_user_input() {
            ReadUserInputResult::Ok(input) => {
                match process_user_move(pos, input) {
                    ProcessUserMoveResult::Retry => continue,
                    ProcessUserMoveResult::ContinueGame => {
                        match do_computer_move(pos) {
                            DoComputerMoveResult::ContinueGame => continue,
                            DoComputerMoveResult::EndGame => break,
                        }
                    },
                    ProcessUserMoveResult::EndGame => break,
                }
            },
            ReadUserInputResult::Err => println!("Please try again"),
        }
    }
}

fn process_user_side_selection(pos: &mut Position) -> ProcessUserSideSelectionResult {
    println!("Select your side: (x/o)");
    match read_user_input() {
        ReadUserInputResult::Ok(input) => {
            match input.trim() {
                "x" => {
                    println!("You have selected to play as X");
                    ProcessUserSideSelectionResult::Ok
                },
                "o" => {
                    println!("You have selected to play as O");
                    do_computer_move(pos);
                    ProcessUserSideSelectionResult::Ok
                },
                _ => {
                    println!("Invalid input, please try again.");
                    ProcessUserSideSelectionResult::Err
                },
            }
        },
        ReadUserInputResult::Err => {
            println!("Please try again");
            ProcessUserSideSelectionResult::Err
        },
    }
}

fn do_computer_move(pos: &mut Position) -> DoComputerMoveResult {
    let search_start_time = SystemTime::now();
    let computer_move = search::find_best_move(pos);
    let search_duration_secs = match SystemTime::now().duration_since(search_start_time) {
        Ok(duration) => (duration.as_secs() * SECS_TO_NANOS_RATIO) as u64 + duration.subsec_nanos() as u64,
        Err(e) => {
            println!("Failed to calculate search duration {}", e);
            0
        },
    };

    match computer_move {
        FindMoveResult::Found (index) => {
            pos.do_move(index);
            io::print_board(&pos);
            println!("Computer moved: {} (took {:.5} secs)", io::get_square_string(index), search_duration_secs as f64 / SECS_TO_NANOS_RATIO as f64);;
            if evaluator::evaluate(&pos).abs() == evaluator::SCORE_BOUNDARY {
                println!("You lost..");
                return DoComputerMoveResult::EndGame;
            }

            if (pos.x_pattern | pos.o_pattern) == DONE_PATTERN {
                println!("Nice draw!");
                return DoComputerMoveResult::EndGame;
            }
        },
        FindMoveResult::None => {
            println!("Nice draw!");
            return DoComputerMoveResult::EndGame;
        },
    }

    DoComputerMoveResult::ContinueGame
}

fn process_user_move(pos: &mut Position, user_move: String) -> ProcessUserMoveResult {
    match io::do_user_move(pos, user_move.as_str()) {
        DoUserMoveResult::Ok => {
            println!("You have moved {}", user_move)
        },
        DoUserMoveResult::Err => {
            println!("Your move is invalid, please try again.");
            return ProcessUserMoveResult::Retry;
        },
    }

    if evaluator::evaluate(&pos).abs() == evaluator::SCORE_BOUNDARY {
        io::print_board(&pos);
        println!("You won!");
        return ProcessUserMoveResult::EndGame;
    }

    if (pos.x_pattern | pos.o_pattern) == DONE_PATTERN {
        io::print_board(&pos);
        println!("Nice draw!");
        return ProcessUserMoveResult::EndGame;
    }

    ProcessUserMoveResult::ContinueGame
}
