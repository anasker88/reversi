use crate::rule::*;

const CORNER: u64 = 0x8100000000000081;
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

pub fn negamax(
    depth: u8,
    player_board: u64,
    enemy_board: u64,
    limit: i32,
    turn: u8,
) -> (i32, u64, u64) {
    //println!("node visit");
    let mut best_move: u64 = 0;
    let mut val: i32 = -std::i32::MAX;
    let mut legal = legal_move(player_board, enemy_board);
    if legal == 0 && legal_move(enemy_board, player_board) == 0 {
        val = evaluate_board(player_board, enemy_board, 0);
        (-val, best_move, 1)
    } else if depth == 0 {
        val = evaluate_board(player_board, enemy_board, turn);
        return (-val, best_move, 1);
    } else {
        let mut cur_nodes = 1;
        let new_nodes: u64;
        let mut next_move: u64;
        if legal == 0 {
            (val, _, new_nodes) = negamax(depth - 1, enemy_board, player_board, -limit, turn);
            (-val, best_move, new_nodes + 1)
        } else {
            loop {
                if legal == 0 {
                    break;
                } else {
                    next_move = legal & (!legal + 1);
                }
                legal = legal & !next_move;
                let (next_player_board, next_enemy_board) =
                    next_board(player_board, enemy_board, next_move);
                let (v, _, new_nodes) =
                    negamax(depth - 1, next_enemy_board, next_player_board, -val, turn);
                cur_nodes += new_nodes;
                if v > val {
                    //println!("new best");
                    val = v;
                    best_move = next_move;
                }
                if val >= limit {
                    break;
                }
            }
            (-val, best_move, cur_nodes)
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

        let parameter = if turn < 35 { 30 } else { 10 };
        //前半は打てる手を広げつつ、自分の石を減らす

        (count_stone(legal) as i32 - count_stone(enemy_legal) as i32) * 3 - player_stone as i32
            + enemy_stone as i32
            + parameter * corner_score
    }
}
