use reversi::ai::*;
// use reversi::testplay;

use reversi::rule::*;
use std::env;
use std::io::*;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

enum ClientState {
    Start,
    WaitForMatch,
    MyMove,
    OpponentMove,
    WaitForAck,
    End,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let argc: usize = args.len();
    let mut host_name: String = "localhost".to_string();
    let mut port = 3000;
    let mut player_name = "T_A".to_string();
    for i in 1..argc {
        match args.get(i).unwrap().as_str() {
            "-H" => {
                host_name = args.get(i + 1).unwrap().to_string();
            }
            "-p" => {
                port = args.get(i + 1).unwrap().parse().unwrap();
            }
            "-n" => {
                player_name = args.get(i + 1).unwrap().to_string();
            }
            _ => {}
        }
    }
    println!("Host name :{:?}", host_name);
    println!("Port number :{:?}", port);
    println!("Player name :{:?}", player_name);
    let host_and_port = format!("{}:{}", host_name, port);
    let mut addrs = host_and_port.to_socket_addrs().unwrap();

    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
        match TcpStream::connect(addr) {
            Err(_) => {
                println!("Connection NG.");
            }
            Ok(stream) => {
                println!("Connection Ok.");
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
                let mut input: Vec<String>;
                let mut state: ClientState = ClientState::Start;
                let mut black: u64 = 0;
                let mut white: u64 = 0;
                let mut turn: u8 = 0; //Passの時は進めないでカウント
                let mut i_am_black: bool = true;
                let mut time_left: i32;
                loop {
                    match state {
                        ClientState::Start => {
                            write_something(&mut writer, format!("OPEN {}\n", player_name));
                            state = ClientState::WaitForMatch;
                        }
                        ClientState::WaitForMatch => {
                            input = read_something(&mut reader);
                            println!("{:?}", input);
                            match input.get(0).unwrap().as_str() {
                                "START" => {
                                    turn = 1;
                                    black = 0x0000000810000000; //黒い石の初期配置
                                    white = 0x0000001008000000; //白い石の初期配置
                                    match input.get(1).unwrap().as_str() {
                                        "BLACK" => {
                                            state = ClientState::MyMove;
                                            i_am_black = true;
                                            println!("I'm Black");
                                        }
                                        "WHITE" => {
                                            state = ClientState::OpponentMove;
                                            println!("I'm white");
                                            i_am_black = false;
                                        }
                                        _ => panic!(),
                                    }
                                    time_left = input.get(3).unwrap().parse().unwrap();
                                    println!("Time left : {} ms", time_left);
                                }
                                "BYE" => {
                                    state = ClientState::End;
                                }
                                _ => panic!(),
                            }
                        }
                        ClientState::MyMove => {
                            let mut my_board = if i_am_black { black } else { white };
                            let mut opponent_board = if i_am_black { white } else { black };
                            //let legal = legal_move(my_board, opponent_board);
                            //現状を表示
                            println!("Turn {}", turn);
                            //print_board(black, white, legal);
                            println!("My Stone : {}", count_stone(my_board));
                            println!("Enemy Stone : {}", count_stone(opponent_board));
                            //手を取得
                            let my_move = ai_play(my_board, opponent_board, turn);
                            if my_move != 0 {
                                turn += 1;
                            }
                            let my_move_as_s = infer_move(my_move);
                            let msg = format!("MOVE {}\n", my_move_as_s);
                            write_something(&mut writer, msg); //ここで送信
                            println!("My Move : {}", my_move_as_s);
                            (my_board, opponent_board) =
                                next_board(my_board, opponent_board, my_move);
                            //盤面を更新
                            black = if i_am_black { my_board } else { opponent_board };
                            white = if i_am_black { opponent_board } else { my_board };
                            //状態遷移
                            state = ClientState::WaitForAck;
                        }
                        ClientState::OpponentMove => {
                            //手を取得
                            let input = read_something(&mut reader);
                            println!("{:?}", input);
                            match input.get(0).unwrap_or(&"None".to_string()).as_str() {
                                "MOVE" => {
                                    let mut my_board = if i_am_black { black } else { white };
                                    let mut opponent_board = if i_am_black { white } else { black };
                                    // let legal = legal_move(opponent_board, my_board);
                                    //現状を表示
                                    println!("Turn {}", turn);
                                    // print_board(black, white, legal);
                                    println!("My Stone : {}", count_stone(my_board));
                                    println!("Enemy Stone : {}", count_stone(opponent_board));
                                    let opponent_move_as_s = input.get(1).unwrap();
                                    let opponent_move = encode_move(opponent_move_as_s.to_string());
                                    if opponent_move != 0 {
                                        turn += 1;
                                    }
                                    println!("Enemy Move : {}", opponent_move_as_s);
                                    (opponent_board, my_board) =
                                        next_board(opponent_board, my_board, opponent_move);
                                    //盤面を更新
                                    black = if i_am_black { my_board } else { opponent_board };
                                    white = if i_am_black { opponent_board } else { my_board };
                                    //状態遷移
                                    state = ClientState::MyMove;
                                }
                                "END" => {
                                    if input.get(1).unwrap().as_str() == "WIN" {
                                        println!("I WIN!");
                                    } else if input.get(1).unwrap().as_str() == "LOSE" {
                                        println!("I LOSE...");
                                        // return;
                                    }
                                    println!("My Stone : {}", input.get(2).unwrap());
                                    println!("Enemy Stone : {}", input.get(3).unwrap());
                                    println!("Reason : {}", input.get(4).unwrap());
                                    state = ClientState::WaitForMatch;
                                }
                                "None" => state = ClientState::OpponentMove,
                                _ => panic!(),
                            }
                        }
                        ClientState::WaitForAck => {
                            let input = read_something(&mut reader);
                            println!("{:?}", input);
                            match input.get(0).unwrap().as_str() {
                                "ACK" => {
                                    time_left = input.get(1).unwrap().parse().unwrap();
                                    println!("Time left : {} ms", time_left);
                                    //状態遷移
                                    state = ClientState::OpponentMove;
                                }
                                "END" => {
                                    if input.get(1).unwrap().as_str() == "WIN" {
                                        println!("I WIN!");
                                    } else if input.get(1).unwrap().as_str() == "LOSE" {
                                        println!("I LOSE...");
                                        // return;
                                    }
                                    println!("My Stone : {}", input.get(2).unwrap());
                                    println!("Enemy Stone : {}", input.get(3).unwrap());
                                    println!("Reason : {}", input.get(4).unwrap());
                                    state = ClientState::WaitForMatch
                                }
                                _ => panic!(),
                            }
                        }
                        ClientState::End => break,
                    }
                }
            }
        }
    } else {
        eprintln!("Invalid Host:Port Number");
    }
}

fn read_something<'a>(reader: &mut BufReader<&TcpStream>) -> Vec<String> {
    let mut msg = String::new();
    reader.read_line(&mut msg).expect("RECEIVE FAILURE!!!");
    msg.split_whitespace().map(|s| s.to_string()).collect()
}

fn write_something(writer: &mut BufWriter<&TcpStream>, comment: String) {
    writer.write(comment.as_bytes()).expect("SEND FAILURE!!!");
    writer.flush().unwrap();
}
