#![feature(iter_collect_into)]

use std::io::{self, BufRead};

use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use chess::{Board, ChessMove};
use chess::Color::White;

mod Engine;

struct UciHandler {
    chess_board: Board,
    time_white: f64,
    time_black: f64,
    moves_made: u16,
    time_force: f64,
}

impl UciHandler {
    fn new() -> Self {
        // Initialize your UCI handler here
        UciHandler {
            chess_board: Board::default(),
            time_white: 0.0,
            time_black: 0.0,
            moves_made: 0,
            time_force: 0.0
        }
    }

    fn run(&mut self) {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines().map(|l| l.unwrap());

        loop {
            if let Some(command) = lines.next() {
                self.handle_command(&command);
            }
        }
    }

    fn handle_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts[0] {
            "uci" => self.uci(),
            "isready" => self.isready(),
            "ucinewgame" => {} // Do nothing on ucinewgame.
            "position" => self.handle_position_command(parts),
            "go" => self.handle_go_command(parts),
            "quit" => std::process::exit(0),
            _ => println!("Unknown command: {}", command),
        }
    }

    fn uci(&self) {
        // Print UCI identification information
        println!("id name KekChess");
        println!("id author Jonathan Heinz");
        println!("uciok");
    }

    fn isready(&self) {
        println!("readyok");
    }

    fn handle_position_command(&mut self, parts: Vec<&str>) {
        if parts.contains(&"startpos") {
            self.chess_board = Board::default()
        }
        if parts.contains(&"fen") {
            let pos_fen = parts.iter().position(|&s| s == "fen").unwrap();
            let fen = *parts.get(pos_fen + 1).unwrap();
            let fen = fen.to_owned() + " " + *parts.get(pos_fen + 2).unwrap();
            let fen = fen + " " + *parts.get(pos_fen + 3).unwrap();
            let fen = fen + " " + *parts.get(pos_fen + 4).unwrap();
            let fen = fen + " " + *parts.get(pos_fen + 5).unwrap();
            let fen = fen + " " + *parts.get(pos_fen + 5).unwrap();
            self.chess_board = Board::from_str(&fen).unwrap();
        }

        if parts.contains(&"moves") {
            self.moves_made = 0;
            let pos_moves = parts.iter().position(|&s| s == "moves").unwrap();
            for i in pos_moves + 1..parts.len() {
                let uci_move = *parts.get(i).unwrap();

                let parsed_move = chess::ChessMove::from_str(uci_move).unwrap();
                self.chess_board = self.chess_board.make_move_new(parsed_move);
                self.moves_made += 1;
            }
        }
    }

    // Example method to handle the "go" command
    fn handle_go_command(&mut self, parts: Vec<&str>) {
        if parts.contains(&"btime") {
            let pos_btime = parts.iter().position(|&s| s == "btime").unwrap();
            let time = *parts.get(pos_btime + 1).unwrap();
            self.time_black = time.parse().unwrap();
            self.time_black /= 1000.0;
        }

        if parts.contains(&"wtime") {
            let pos_wtime = parts.iter().position(|&s| s == "wtime").unwrap();
            let time = *parts.get(pos_wtime + 1).unwrap();
            self.time_white = time.parse().unwrap();
            self.time_white /= 1000.0;
        }

        if parts.contains(&"movetime") {
            let pos_movtime = parts.iter().position(|&s| s == "movetime").unwrap();
            let time = *parts.get(pos_movtime + 1).unwrap();
            self.time_force = time.parse().unwrap();
        }

        let mut engine = Engine::Engine::new();
        let mut time_slice = 10.0;

        if self.chess_board.side_to_move() == White && self.time_white != 0.0 {
            if self.moves_made != 60 {
                time_slice = self.time_white / (60 - self.moves_made) as f64;
            } else {
                time_slice = self.time_white / (150 - self.moves_made) as f64;
            }
        } else if self.time_black != 0.0 {
            if self.moves_made != 60 {
                time_slice = self.time_black / (60 - self.moves_made) as f64;
            } else {
                time_slice = self.time_black / (150 - self.moves_made) as f64;
            }
        } else if self.time_force != 0.0 {
            time_slice = self.time_force / 1000.0
        }

        let (tx, rx) = mpsc::channel();
        let (tx_cancle, rx_cancle) = mpsc::channel();
        let board  = self.chess_board.clone();
        let _ = thread::spawn(move || {
            engine.iterative_deepening(board, tx, rx_cancle);
        });

        let mut score = 0;
        let mut best_move = ChessMove::default();
        let mut nodes = 0u32;
        let mut depth = 0u16;

        println!("info timeslice {time_slice}");


        while time_slice > 0.0 {
            let inst_now = Instant::now();
            let res = rx.recv_timeout(Duration::from_secs_f64(time_slice));
            if res.is_ok() {
                (score, best_move, depth, nodes) = res.unwrap();
            }

            let inst_after = Instant::now();

            let duration = inst_after.duration_since(inst_now).as_secs_f64();
            time_slice = time_slice - duration;
        }

        let _ = tx_cancle.send(()); // cancel search


        println!("info score {} nodes {nodes} depth {depth}", score);
        println!("bestmove {}", best_move.to_string())
    }
}

fn main() {
    let mut uci_handler = UciHandler::new();

    uci_handler.run();

    //let mut engine = Engine::Engine::new();
//
//
    //let (mov, score, variation) = engine.iterative_deepening(6, Board::default());
//
    //for mov in variation {
    //    println!("{mov}");
    //}
//
    //println!("info {}", score);
    //println!("bestmove {}", mov.to_string())
}