const HORIZONTAL_MASK: u64 = 0x7E7E7E7E7E7E7E7E;
const VERTICAL_MASK: u64 = 0x00FFFFFFFFFFFF00;
const ALLSIDE_MASK: u64 = 0x007E7E7E7E7E7E00;
// const EMPTY: u64 = 0;
const SHIFTS: [i8; 8] = [1, -1, 7, -7, 8, -8, 9, -9];

//shiftの方向に対応するmaskを取得
fn get_mask(shift: i8) -> u64 {
    match shift.abs() {
        1 => HORIZONTAL_MASK,
        7 => ALLSIDE_MASK,
        8 => VERTICAL_MASK,
        9 => ALLSIDE_MASK,
        _ => 0,
    }
}

//shift方向へのlegal moveを発見
fn find_legal_move(player_board: u64, enemy_board: u64, shift: i8) -> u64 {
    let mask: u64 = get_mask(shift);
    let masked_board: u64 = enemy_board & mask;
    let mut tmp: u64 = player_board;
    for i in 0..6 {
        if shift > 0 {
            let shifted_tmp = tmp >> shift;
            tmp = if i == 0 {
                shifted_tmp & masked_board
            } else {
                (shifted_tmp & masked_board) | tmp
            };
        } else {
            let shifted_tmp = tmp << (-shift);
            tmp = if i == 0 {
                shifted_tmp & masked_board
            } else {
                (shifted_tmp & masked_board) | tmp
            };
        }
    }
    let empty_board = !(player_board | enemy_board);
    if shift > 0 {
        tmp = tmp >> shift;
    } else {
        tmp = tmp << (-shift);
    }
    tmp & empty_board
}
//legal moveのbitを立てる
pub fn legal_move(player_board: u64, enemy_board: u64) -> u64 {
    let mut ret: u64 = 0;
    for shift in SHIFTS {
        ret |= find_legal_move(player_board, enemy_board, shift);
    }
    ret
}

fn find_reverse_stones(player_board: u64, enemy_board: u64, next_move: u64, shift: i8) -> u64 {
    let mask: u64 = get_mask(shift);
    let masked_enemy = enemy_board & mask;
    let mut tmp = if shift > 0 {
        next_move >> shift
    } else {
        next_move << -shift
    };

    let mut reverse_tmp = tmp & masked_enemy;
    while (tmp & masked_enemy) != 0 {
        tmp = if shift > 0 {
            tmp >> shift
        } else {
            tmp << -shift
        };
        reverse_tmp |= tmp & masked_enemy;
    }
    if (player_board & tmp) != 0 {
        reverse_tmp
    } else {
        0
    }
}

pub fn next_board(player_board: u64, enemy_board: u64, next_move: u64) -> (u64, u64) {
    let mut reverse: u64 = 0;
    for shift in SHIFTS {
        reverse |= find_reverse_stones(player_board, enemy_board, next_move, shift);
    }
    (player_board | next_move | reverse, enemy_board & !reverse)
}

pub fn judge_move(player_board: u64, enemy_board: u64, next_move: u64) -> bool {
    if count_stone(next_move) != 1 {
        //動かしている石は一つだけか
        false
    } else {
        let legal = legal_move(player_board, enemy_board);
        if (legal & next_move) == 0 {
            //legal_moveか
            false
        } else {
            true
        }
    }
}

pub fn no_move(black: u64, white: u64) -> bool {
    (legal_move(black, white) == 0) && (legal_move(white, black) == 0)
}
pub fn count_stone(v: u64) -> u64 {
    let mut count = (v & 0x5555555555555555) + ((v >> 1) & 0x5555555555555555);
    count = (count & 0x3333333333333333) + ((count >> 2) & 0x3333333333333333);
    count = (count & 0x0f0f0f0f0f0f0f0f) + ((count >> 4) & 0x0f0f0f0f0f0f0f0f);
    count = (count & 0x00ff00ff00ff00ff) + ((count >> 8) & 0x00ff00ff00ff00ff);
    count = (count & 0x0000ffff0000ffff) + ((count >> 16) & 0x0000ffff0000ffff);
    (count & 0x00000000ffffffff) + ((count >> 32) & 0x00000000ffffffff)
}
pub fn print_stonenum(black: u64, white: u64) {
    let black_stone = count_stone(black);
    let white_stone = count_stone(white);
    println!("Black: {}", black_stone);
    println!("White: {}", white_stone);
}

pub fn print_board(black: u64, white: u64, legal: u64) {
    let mut mask: u64 = 1 << 63;
    println!("  A B C D E F G H");
    for row in 0..8 {
        print!("{} ", row + 1);
        for _ in 0..8 {
            let b = (black & mask) != 0;
            let w = (white & mask) != 0;
            let n = (legal & mask) != 0;

            let square = match (b, w, n) {
                (true, _, _) => "●", // 黒い石
                (_, true, _) => "○", // 白い石
                (_, _, true) => "X",
                _ => ".", // 空きマス
            };
            print!("{} ", square);

            mask >>= 1;
        }
        println!();
    }
}
pub fn infer_move(my_move: u64) -> String {
    let mut count: u8 = 0;
    let mut mask: u64 = 1 << 63;
    while (my_move & mask == 0) && count < 64 {
        mask >>= 1;
        count += 1;
    }
    let column = (count % 8 + 'A' as u8) as u8 as char;
    let line = (count / 8 + '1' as u8) as char;
    if count == 64 {
        "PASS".to_string()
    } else {
        format!("{}{}", column, line)
    }
}

pub fn encode_move(my_move: String) -> u64 {
    match my_move.as_str() {
        "PASS" => 0,
        decoded_move => {
            let move_as_vec = decoded_move.chars().collect::<Vec<char>>();
            let shift = (move_as_vec.get(0).unwrap().clone() as u8 - 'A' as u8)
                + (move_as_vec.get(1).unwrap().clone() as u8 - '1' as u8) * 8;
            1 << (63 - shift)
        }
    }
}
