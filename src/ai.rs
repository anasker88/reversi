use crate::rule::*;

const CORNER: u64 = 0x8100000000000081;
const NEXT_TO_CORNER: u64 = 0x42c300000000c342;
pub const END_SEARCH: u8 = 16;
// const NEXT_TO_CORNER: u64 = 0x42c300000000c342;
pub fn human_play() -> u64 {
    let mut buf = String::new(); //A
    std::io::stdin().read_line(&mut buf).unwrap();
    println!("{}", buf);
    if let Some(column) = buf.chars().nth(0) {
        if let Some(line) = buf.chars().nth(1) {
            if (column as u8 >= 'A' as u8) && (line as u8 >= '1' as u8) {
                let shift = (column as u8 - 'A' as u8) + (line as u8 - '1' as u8) * 8;
                return 1 << (63 - shift);
            }
        }
    }
    println!("invalid input");
    panic!();
}
pub fn random(legal: u64) -> u64 {
    let mut mask = 1;
    while mask & legal == 0 && mask != 0 {
        mask <<= 1;
    }
    mask
}

pub fn ai_play(player_board: u64, enemy_board: u64, turn: u8, time_left: u64) -> u64 {
    //手を取得
    let mut score = 0;
    let mut best_moves = Vec::new();
    let mut former_best_moves: Vec<u64> = Vec::new();
    let mut nodes;
    if turn < 60 - END_SEARCH {
        let max_depth = if turn <= 20 {
            9
        } else if turn <= 35 {
            9
        } else {
            11
        };
        let nodes_limit = if turn <= 35 { 4000000 } else { 6000000 };
        for depth in 1..max_depth + 1 {
            former_best_moves = best_moves;
            (score, best_moves, nodes) = negamax(
                depth,
                player_board,
                enemy_board,
                1000000,
                former_best_moves.clone(),
                turn,
                nodes_limit * 10,
            );
            if score.abs() != 1234 {
                println!("depth:{} Score:{} visited nodes:{}", depth, -score, nodes);
            }
            if nodes > nodes_limit {
                break;
            }
            // print!("Move:");
            // for my_move in best_moves.clone() {
            //     print!(" {}", infer_move(my_move));
            // }
            // println!();
        }
        if score.abs() == 1234 {
            former_best_moves.pop().unwrap()
        } else {
            best_moves.pop().unwrap()
        }
    } else {
        (score, best_moves, nodes) = negamax(
            100,
            player_board,
            enemy_board,
            1000000,
            best_moves.clone(),
            turn,
            std::cmp::min(time_left * 6000, 180000000),
        );
        if score.abs() == 1234 {
            println!("Yomikiri failed! Nodes: {}", nodes);
        } else {
            println!("depth:Inf Score:{} visited nodes:{}", -score, nodes);
            // print!("Move:");
            // for my_move in best_moves.clone() {
            //     print!(" {}", infer_move(my_move));
            // }
            // println!();
        }
        if score <= 0 {
            best_moves.pop().unwrap()
        } else {
            let max_depth = if turn < 57 {
                std::cmp::min(11, 60 - turn - 2)
            } else {
                1
            };
            let nodes_limit = 5000000;
            for depth in 1..max_depth + 1 {
                former_best_moves = best_moves;
                (score, best_moves, nodes) = negamax(
                    depth,
                    player_board,
                    enemy_board,
                    1000000,
                    former_best_moves.clone(),
                    turn,
                    nodes_limit * 10,
                );
                if score.abs() != 1234 {
                    println!("depth:{} Score:{} visited nodes:{}", depth, -score, nodes);
                }
                if nodes > nodes_limit {
                    break;
                }
            }
            if score.abs() == 1234 {
                former_best_moves.pop().unwrap()
            } else {
                best_moves.pop().unwrap()
            }
        }
    }
}

fn negamax(
    depth: u8,
    player_board: u64,
    enemy_board: u64,
    limit: i32,
    mut former_best_moves: Vec<u64>,
    turn: u8,
    node_limit: u64,
) -> (i32, Vec<u64>, u64) {
    // if turn > 3 {
    //     panic!();
    // }
    //println!("node visit");
    let mut current_best_moves = Vec::new();
    let mut val: i32 = -1000000;
    let mut legal = legal_move(player_board, enemy_board);
    if legal == 0 && legal_move(enemy_board, player_board) == 0 {
        val = evaluate_board(player_board, enemy_board, 0);
        (-val, vec![0, 0], 1)
    } else if depth == 0 {
        val = evaluate_board(player_board, enemy_board, turn);
        return (-val, current_best_moves, 1);
    } else {
        let mut cur_nodes = 1;
        let new_nodes: u64;
        let mut next_move: u64;
        if legal == 0 {
            let popped_move = former_best_moves.pop();
            if popped_move != None {
                if popped_move.unwrap() != 0 {
                    println!("Something went wrong!");
                }
            }
            (val, current_best_moves, new_nodes) = negamax(
                depth - 1,
                enemy_board,
                player_board,
                if limit == 1000000 { limit } else { -limit },
                former_best_moves,
                turn,
                node_limit,
            );
            current_best_moves.push(0);
            (-val, current_best_moves, new_nodes + 1)
        } else {
            let popped_move = former_best_moves.pop();
            let mut first_loop = true;
            if popped_move == None {
                next_move = legal & (!legal + 1);
            } else {
                next_move = popped_move.unwrap();
                // println!("Popped! move:{:?} depth:{}", infer_move(next_move), depth);
            }
            loop {
                if next_move & legal == 0 {
                    println!("Invalid move");
                }
                let (next_player_board, next_enemy_board) =
                    next_board(player_board, enemy_board, next_move);
                let (v, mut best_moves, new_nodes) = negamax(
                    depth - 1,
                    next_enemy_board,
                    next_player_board,
                    -val,
                    if first_loop {
                        former_best_moves.clone()
                    } else {
                        Vec::new()
                    },
                    turn,
                    node_limit,
                );
                cur_nodes += new_nodes;
                if cur_nodes > node_limit {
                    return (1234, Vec::new(), cur_nodes);
                }
                if v > val {
                    //println!("new best");
                    val = v;
                    best_moves.push(next_move);
                    current_best_moves = best_moves;
                }
                if val >= limit {
                    break;
                }
                first_loop = false;
                legal = legal & !next_move;
                if legal == 0 {
                    break;
                } else {
                    next_move = legal & (!legal + 1);
                }
            }
            (-val, current_best_moves, cur_nodes)
        }
    }
}

fn evaluate_board(player_board: u64, enemy_board: u64, turn: u8) -> i32 {
    //println!("evaluation called");
    //panic!();
    let player_stone = count_stone(player_board);
    let enemy_stone = count_stone(enemy_board);
    //終局評価関数。勝ちなら無限大、負けなら負の無限大 turn0が終局
    if turn == 0 {
        if player_stone > enemy_stone {
            100000
        } else if player_stone < enemy_stone {
            -100000
        } else {
            0
        }
    } else {
        let legal = legal_move(player_board, enemy_board);
        let enemy_legal = legal_move(enemy_board, player_board);
        let corner_score =
            count_stone(player_board & CORNER) as i32 - count_stone(enemy_board & CORNER) as i32;
        let _next_to_corner_score = count_stone(enemy_board & NEXT_TO_CORNER) as i32
            - count_stone(player_board & NEXT_TO_CORNER) as i32;

        let parameter = if turn < 35 { 30 } else { 10 };
        //前半は打てる手を広げつつ、自分の石を減らす

        if turn < 50 {
            (count_stone(legal) as i32 - count_stone(enemy_legal) as i32) * 3
                + (10 / (count_stone(enemy_legal) + 1) as i32
                    - 10 / (count_stone(legal) + 1) as i32)
                + parameter * corner_score
                + parameter * _next_to_corner_score / 7
                - if turn <= 35 {
                    player_stone as i32 - enemy_stone as i32
                } else {
                    0
                }
        } else {
            player_stone as i32 - enemy_stone as i32
        }
    }
}
