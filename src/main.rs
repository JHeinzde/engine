#![feature(iter_collect_into)]

use std::io::{self, BufRead};
use std::ops::Neg;
use std::str::FromStr;

use chess::Board;

mod Engine;

struct UciHandler {
    chess_board: Board,
}

impl UciHandler {
    fn new() -> Self {
        // Initialize your UCI handler here
        UciHandler {
            chess_board: Board::default()
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
            let pos_moves = parts.iter().position(|&s| s == "moves").unwrap();
            for i in pos_moves + 1..parts.len() {
                let uci_move = *parts.get(i).unwrap();
                let parsed_move = chess::ChessMove::from_str(uci_move).unwrap();
                self.chess_board = self.chess_board.make_move_new(parsed_move);
            }
        }
    }

    // Example method to handle the "go" command
    fn handle_go_command(&self, parts: Vec<&str>) {
        let mut engine = Engine::Engine::new();
        let (bmove, score, mut variation) = engine
            .iterative_deepening(5, self.chess_board.clone());


        variation.reverse();
        let mut s_var = String::new();

        for mov in variation {
            s_var.push_str(&*mov.to_string());
            s_var.push_str(" ")
        }


        println!("info score {}", score);
        println!("info variation {}", s_var);
        println!("info visited nodes {}", engine.pos_counter);
        println!("info pruning operations {}", engine.cut_off_counter);
        println!("bestmove {}", bmove.to_string())
    }
}

fn main() {
    let mut uci_handler = UciHandler::new();

    //uci_handler.run();

    let mut engine = Engine::Engine::new();


    let (mov, score, mut variation) = engine.iterative_deepening(6, Board::from_str("3qr2k/pbpp2pp/1p5N/3Q2b1/2P1P3/P7/1PP2PPP/R4RK1 w - - 0 1").unwrap());
    println!("info {}", score);
    println!("bestmove {}", mov.to_string())
}