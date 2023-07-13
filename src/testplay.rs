use crate::ai::*;
use crate::rule::*;

#[derive(Debug, PartialEq)]
enum GameState {
    Continue,
    NoMove,
    BlackBreakingRule,
    WhiteBreakingRule,
}

pub fn testplay() {
    let mut black: u64 = 0x0000001008000000; // 黒い石の配置
    let mut white: u64 = 0x0000000810000000; // 白い石の配置
    let mut black_playing = true;
    let mut game_state: GameState = GameState::Continue;
    while game_state == GameState::Continue {
        if black_playing {
            println!("Black's turn");
            let legal = legal_move(black, white);
            print_board(black, white, legal);
            print_stonenum(black, white);
            if legal == 0 {
                println!("Black passed");
            } else {
                let next_move = human_play();
                println!("Black played");
                //有効な手か調べる
                if judge_move(black, white, next_move) == false {
                    game_state = GameState::BlackBreakingRule;
                    break;
                }
                (black, white) = next_board(black, white, next_move);
            }
        } else {
            println!("White's turn");
            let legal = legal_move(white, black);
            print_board(black, white, legal);
            print_stonenum(black, white);
            if legal == 0 {
                println!("White passed");
            } else {
                let next_move = human_play();
                println!("White played");
                if judge_move(white, black, next_move) == false {
                    //有効な手か調べる
                    game_state = GameState::WhiteBreakingRule;
                    break;
                }
                (white, black) = next_board(white, black, next_move);
            }
        }
        //終局か調べる
        if no_move(black, white) {
            game_state = GameState::NoMove;
        }
        //プレイヤー交代
        black_playing = !black_playing;
    }
    match game_state {
        GameState::BlackBreakingRule => {
            println!("Black broke the rule");
            println!("White won!");
        }
        GameState::WhiteBreakingRule => {
            println!("White broke the rule");
            println!("Black won!");
        }
        GameState::NoMove => {
            let black_stone = count_stone(black);
            let white_stone = count_stone(white);
            println!("Black: {}", black_stone);
            println!("White: {}", white_stone);
            if black_stone > white_stone {
                println!("Black won!");
            } else if black_stone < white_stone {
                println!("white won!");
            } else {
                println!("Draw!");
            }
        }
        _ => (),
    }
}
