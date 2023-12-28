mod Engine;

use std::io::{self, BufRead, Write};
use chess::Board;
use std::str::FromStr;
use crate::Engine::analyze;

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
            self.chess_board = Board::from_str(fen).unwrap();
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
        let (bmove, score) = analyze(8, self.chess_board.clone());

        println!("info score {}", score);
        println!("bestmove {}", bmove.to_string())
    }
}

fn main() {
    let mut uci_handler = UciHandler::new();

    uci_handler.run();

    //let (mov, score) = analyze(8, Board::default());
    //println!("info {}", score);
    //println!("bestmove {}", mov.to_string())
}