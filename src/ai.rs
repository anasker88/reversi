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
    return mask;
}

pub fn negamax(
    nodes: u64,
    player_board: u64,
    enemy_board: u64,
    limit: i32,
    turn: u8,
) -> (i32, u64) {
    //println!("node visit");
    let mut best_move: u64 = 0;
    let mut val: i32 = -std::i32::MAX;
    if nodes == 0
        || (legal_move(player_board, enemy_board) == 0
            && legal_move(enemy_board, player_board) == 0)
    {
        let v = evaluate_board(player_board, enemy_board, turn);

        return (-v, best_move);
    }

    let legal = legal_move(player_board, enemy_board);
    if legal == 0 {
        (val, _) = negamax(nodes, enemy_board, player_board, -limit, turn);
        return (-val, best_move);
    }
    let next_nodes = nodes / count_stone(legal);
    let mut next_move: u64 = 1;
    while next_move != 0 {
        //println!("{}", infer_move(next_move));
        if next_move & legal == 0 {
            next_move <<= 1;
            continue;
        }
        let (next_player_board, next_enemy_board) =
            next_board(player_board, enemy_board, next_move);
        let (v, _) = negamax(next_nodes, next_enemy_board, next_player_board, -val, turn);
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

fn evaluate_board(player_board: u64, enemy_board: u64, turn: u8) -> i32 {
    let player_stone = count_stone(player_board);
    let enemy_stone = count_stone(enemy_board);
    let legal = legal_move(player_board, enemy_board);
    let enemy_legal = legal_move(enemy_board, player_board);
    //終局評価関数。勝ちなら無限大、負けなら負の無限大
    if legal == 0 && enemy_legal == 0 {
        if player_stone > enemy_stone {
            return std::i32::MAX / (2 + enemy_stone as i32);
        } else if player_stone < enemy_stone {
            return -std::i32::MAX / (2 + player_stone as i32);
        } else {
            return 0;
        }
    }
    let corner_score =
        count_stone(player_board & CORNER) as i32 - count_stone(enemy_board & CORNER) as i32;

    let parameter = 30;
    //前半は打てる手を広げつつ、自分の石を減らす
    if turn <= 35 {
        return count_stone(legal) as i32 - count_stone(enemy_legal) as i32 - player_stone as i32
            + enemy_stone as i32
            + parameter * corner_score;
    } else {
        return count_stone(legal) as i32 - count_stone(enemy_legal) as i32 + 20 * corner_score;
    }
}
