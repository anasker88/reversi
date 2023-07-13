use crate::rule::*;

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
    return mask;
}

pub fn negamax(depth: u8, player_board: u64, enemy_board: u64, limit: i32) -> (i32, u64) {
    //println!("node visit");
    let mut best_move: u64 = 0;
    let mut val: i32 = -std::i32::MAX;
    if depth == 0 {
        let v = evaluate_board(player_board, enemy_board);

        return (-v, best_move);
    }

    let legal = legal_move(player_board, enemy_board);
    if legal == 0 {
        (val, _) = negamax(depth - 1, enemy_board, player_board, -limit);
        return (-val, best_move);
    }
    let mut next_move: u64 = 1;
    while next_move != 0 {
        //println!("{}", infer_move(next_move));
        if next_move & legal == 0 {
            next_move <<= 1;
            continue;
        }
        let (next_player_board, next_enemy_board) =
            next_board(player_board, enemy_board, next_move);
        let (v, _) = negamax(depth - 1, next_enemy_board, next_player_board, -val);
        if v > val {
            //println!("new best");
            val = v;
            best_move = next_move;
        }
        if val >= limit {
            break;
        }
        next_move <<= 1;
    }
    return (-val, best_move);
}

fn evaluate_board(player_board: u64, enemy_board: u64) -> i32 {
    let player_stone = count_stone(player_board);
    let enemy_stone = count_stone(enemy_board);
    if legal_move(player_board, enemy_board) == 0 && legal_move(enemy_board, player_board) == 0 {
        if player_stone > enemy_stone {
            return std::i32::MAX / 2;
        } else if player_stone < enemy_stone {
            return -std::i32::MAX / 2;
        } else {
            return 0;
        }
    }
    //let legal = legal_move(player_board, enemy_board);
    return player_stone as i32 - enemy_stone as i32;
}
