use std::hash::Hash;
use std::ops::BitAnd;

use chess::{BitBoard, Board, CacheTable, ChessMove, Color, EMPTY, MoveGen, Piece, Square};
use chess::Color::White;
use Color::Black;
use crate::Engine::NodeType::{ALL_NODE, CUT_NODE, PV_NODE};


const WHITE_PAWN: [i32; 64] = [
    // Rank 1
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 2
    5, 10, 10, -20, -20, 10, 10, 5,
    // Rank 3
    5, -5, -10, 0, 0, -10, -5, 5,
    // Rank 4
    0, 0, 0, 20, 20, 0, 0, 0,
    // Rank 5
    5, 5, 10, 25, 25, 10, 5, 5,
    // Rank 6
    10, 10, 20, 30, 30, 20, 10, 10,
    // Rank 7
    50, 50, 50, 50, 50, 50, 50, 50,
    // Rank 8
    0, 0, 0, 0, 0, 0, 0, 0,
];

const WHITE_ROOK: [i32; 64] = [
    // Rank 1
    0, 0, 0, 5, 5, 0, 0, 0,
    // Rank 2
    -5, 0, 0, 0, 0, 0, 0, -5,
    // Rank 3
    -5, 0, 0, 0, 0, 0, 0, -5,
    // Rank 4
    -5, 0, 0, 0, 0, 0, 0, -5,
    // Rank 5
    -5, 0, 0, 0, 0, 0, 0, -5,
    // Rank 6
    -5, 0, 0, 0, 0, 0, 0, -5,
    // Rank 7
    5, 10, 10, 10, 10, 10, 10, 5,
    // Rank 8
    0, 0, 0, 0, 0, 0, 0, 0,
];


const WHITE_KNIGHT: [i32; 64] = [
    // Rank 1
    -50, -40, -30, -30, -30, -30, -40, -50,
    // Rank 2
    -40, -20, 0, 5, 5, 0, -20, -40,
    // Rank 3
    -30, 5, 10, 15, 15, 10, 5, -30,
    // Rank 4
    -30, 0, 15, 20, 20, 15, 0, -30,
    // Rank 5
    -30, 5, 15, 20, 20, 15, 5, -30,
    // Rank 6
    -30, 0, 10, 15, 15, 10, 0, -30,
    // Rank 7
    -40, -20, 0, 0, 0, 0, -20, -40,
    // Rank 8
    -50, -40, -30, -30, -30, -30, -40, -50,
];

const WHITE_BISHOP: [i32; 64] = [
    // Rank 1
    -20, -10, -10, -10, -10, -10, -10, -20,
    // Rank 2
    -10, 5, 0, 0, 0, 0, 5, -10,
    // Rank 3
    -10, 10, 10, 10, 10, 10, 10, -10,
    // Rank 4
    -10, 0, 10, 10, 10, 10, 0, -10,
    // Rank 5
    -10, 5, 5, 10, 10, 5, 5, -10,
    // Rank 6
    -10, 0, 5, 10, 10, 5, 0, -10,
    // Rank 7
    -10, 0, 0, 0, 0, 0, 0, -10,
    // Rank 8
    -20, -10, -10, -10, -10, -10, -10, -20,
];


const WHITE_QUEEN: [i32; 64] = [
    // Rank 1
    -20, -10, -10, -5, -5, -10, -10, -20,
    // Rank 2
    -10, 0, 5, 0, 0, 0, 0, -10,
    // Rank 3
    -10, 5, 5, 5, 5, 5, 0, -10,
    // Rank 4
    0, 0, 5, 5, 5, 5, 0, -5,
    // Rank 5
    -5, 0, 5, 5, 5, 5, 0, -5,
    // Rank 6
    -10, 0, 5, 5, 5, 5, 0, -10,
    // Rank 7
    -10, 0, 0, 0, 0, 0, 0, -10,
    // Rank 8
    -20, -10, -10, -5, -5, -10, -10, -20,
];


const WHITE_KING: [i32; 64] = [
    // Rank 1
    20, 30, 10, 0, 0, 10, 30, 20,
    // Rank 2
    20, 20, 0, 0, 0, 0, 20, 20,
    // Rank 3
    -10, -20, -20, -20, -20, -20, -20, -10,
    // Rank 4
    -20, -30, -30, -40, -40, -30, -30, -20,
    // Rank 5
    -30, -40, -40, -50, -50, -40, -40, -30,
    // Rank 6
    -30, -40, -40, -50, -50, -40, -40, -30,
    // Rank 7
    -30, -40, -40, -50, -50, -40, -40, -30,
    // Rank 8
    -30, -40, -40, -50, -50, -40, -40, -30,
];

const FLIP: [usize; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
    8, 9, 10, 11, 12, 13, 14, 15,
    0, 1, 2, 3, 4, 5, 6, 7
];

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
struct TranspositionEntry {
    mov: Option<ChessMove>,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
    score: Option<i32>,
    node_type: NodeType,
    depth: u16,
}


#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
enum NodeType {
    PV_NODE,
    ALL_NODE,
    CUT_NODE,
}


pub struct Engine {
    pub pos_counter: i32,
    pub cut_off_counter: i32,
    transposition_table: CacheTable<TranspositionEntry>,
    repeat_table: CacheTable<u16>,
}


const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;

const KING_VALUE: i32 = 20000;

const PIECE_VALUES: [i32; 6] = [
    PAWN_VALUE,
    KNIGHT_VALUE,
    BISHOP_VALUE,
    ROOK_VALUE,
    QUEEN_VALUE,
    KING_VALUE
];

pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0i32;

    let white_pieces = board.color_combined(White);
    let black_pieces = board.color_combined(Black);

    let mut white_pawns = white_pieces.bitand(board.pieces(Piece::Pawn));
    let mut white_knights = white_pieces.bitand(board.pieces(Piece::Knight));
    let mut white_bishops = white_pieces.bitand(board.pieces(Piece::Bishop));
    let mut white_rooks = white_pieces.bitand(board.pieces(Piece::Rook));
    let mut white_queen = white_pieces.bitand(board.pieces(Piece::Queen));
    let mut white_king = white_pieces.bitand(board.pieces(Piece::King));

    let mut black_pawns = black_pieces.bitand(board.pieces(Piece::Pawn));
    let mut black_knights = black_pieces.bitand(board.pieces(Piece::Knight));
    let mut black_bishops = black_pieces.bitand(board.pieces(Piece::Bishop));
    let mut black_rooks = black_pieces.bitand(board.pieces(Piece::Rook));
    let mut black_queen = black_pieces.bitand(board.pieces(Piece::Queen));
    let mut black_king = black_pieces.bitand(board.pieces(Piece::King));

    let white_pawns_count = white_pawns.popcnt() as i32;
    let white_knights_count = white_knights.popcnt() as i32;
    let white_bishops_count = white_bishops.popcnt() as i32;
    let white_rooks_count = white_rooks.popcnt() as i32;
    let white_queen_count = white_queen.popcnt() as i32;

    let black_pawns_count = black_pawns.popcnt() as i32;
    let black_knights_count = black_knights.popcnt() as i32;
    let black_bishops_count = black_bishops.popcnt() as i32;
    let black_rooks_count = black_rooks.popcnt() as i32;
    let black_queen_count = black_queen.popcnt() as i32;

    score = (white_pawns_count - black_pawns_count) * PAWN_VALUE + (white_bishops_count - black_bishops_count) * BISHOP_VALUE +
        (white_knights_count - black_knights_count) * KNIGHT_VALUE + (white_rooks_count - black_rooks_count) * ROOK_VALUE +
        (white_queen_count - black_queen_count) * QUEEN_VALUE;


    score += iterate_bitboard(&mut white_pawns, |square: Square| WHITE_PAWN[square.to_index()]);
    score += iterate_bitboard(&mut black_pawns, |square: Square| -WHITE_PAWN[FLIP[square.to_index()]]);
    score += iterate_bitboard(&mut white_bishops, |square: Square| WHITE_BISHOP[square.to_index()]);
    score += iterate_bitboard(&mut black_bishops, |square: Square| -WHITE_BISHOP[FLIP[square.to_index()]]);
    score += iterate_bitboard(&mut black_knights, |square: Square| -WHITE_KNIGHT[FLIP[square.to_index()]]);
    score += iterate_bitboard(&mut white_knights, |square: Square| WHITE_KNIGHT[square.to_index()]);
    score += iterate_bitboard(&mut white_rooks, |square: Square| WHITE_ROOK[square.to_index()]);
    score += iterate_bitboard(&mut black_rooks, |square: Square| -WHITE_ROOK[FLIP[square.to_index()]]);
    score += iterate_bitboard(&mut white_queen, |square: Square| WHITE_QUEEN[square.to_index()]);
    score += iterate_bitboard(&mut black_queen, |square: Square| -WHITE_QUEEN[FLIP[square.to_index()]]);
    score += iterate_bitboard(&mut white_king, |square: Square| WHITE_KING[square.to_index()]);
    score += iterate_bitboard(&mut black_king, |square: Square| -WHITE_KING[FLIP[square.to_index()]]);


    return score;
}

#[inline]
fn iterate_bitboard(bb: &mut BitBoard, f: fn(Square) -> i32) -> i32 {
    let mut score = 0;
    for i in bb {
        score += f(i);
    }

    return score;
}

impl Engine {
    pub fn new() -> Engine {
        return Engine {
            pos_counter: 0,
            cut_off_counter: 0,
            transposition_table: CacheTable::new(33554432, TranspositionEntry { mov: None, lower_bound: None, upper_bound: None, score: None, node_type: ALL_NODE, depth: 0 }),
            repeat_table: CacheTable::new(33554432, 0u16),
        };
    }


    pub fn iterative_deepening(&mut self, depth: u16, board: Board) -> (ChessMove, i32, Vec<ChessMove>) {
        let mut first_guess = 0;
        let mut best_move = None;
        let mut variation = Vec::new();

        let maximize = if board.side_to_move() == White {
            true
        } else {
            false
        };

        for d in 1..depth + 1 {
            (first_guess, best_move, variation) = self.mtdf(board, first_guess, depth, maximize);
            //println!("{:?}", best_move);
        }

        return (best_move.unwrap(), first_guess, variation);
    }

    fn mtdf(&mut self, board: Board, f: i32, depth: u16, maximize: bool) -> (i32, Option<ChessMove>, Vec<ChessMove>) {
        let mut score = f;
        let mut mov = None;
        let mut lower_bound = i32::MIN;
        let mut upper_bound = i32::MAX;
        let mut beta = 0;
        let mut variation = Vec::new();
        self.repeat_table =  CacheTable::new(33554432, 0u16);


        while lower_bound < upper_bound {
            if score == lower_bound {
                beta = score + 100;
            } else {
                beta = score;
            }

            (score, mov, variation) = self.alpha_beta(depth, beta - 100, beta, maximize, board);
            if score < beta {
                upper_bound = score;
            } else {
                lower_bound = score;
            }
        }

        return (score, mov, variation);
    }

    fn sort_moves(&mut self, board: &Board, move_gen: &mut MoveGen, cache_entry: Option<TranspositionEntry>) -> (Vec<ChessMove>) {
        let mut captures = Vec::new();
        let mut res_val = Vec::new();
        let color_captures = if board.side_to_move() == White {
            Black
        } else {
            White
        };

        if cache_entry != None && cache_entry.unwrap().mov != None {
            move_gen.remove_move(cache_entry.unwrap().mov.unwrap());
            res_val.push(cache_entry.unwrap().mov.unwrap())
        }

        move_gen.set_iterator_mask(*board.color_combined(color_captures));
        move_gen.collect_into(&mut captures);

        captures.sort_by(|mov1, mov2| {
            (PIECE_VALUES[board.piece_on(mov1.get_dest()).unwrap().to_index()]
                - PIECE_VALUES[board.piece_on(mov1.get_source()).unwrap().to_index()])
                .cmp(
                    &(PIECE_VALUES[board.piece_on(mov2.get_dest()).unwrap().to_index()]
                        - PIECE_VALUES[board.piece_on(mov2.get_source()).unwrap().to_index()])
                )
        });

        captures.reverse();

        move_gen.set_iterator_mask(!EMPTY);
        let mut rest_moves = Vec::new();
        move_gen.collect_into(&mut rest_moves);

        res_val.append(&mut captures);
        res_val.append(&mut rest_moves);

        return res_val;
    }

    fn alpha_beta(&mut self, depth: u16, mut alpha: i32, mut beta: i32, maximize: bool, board: Board)
        -> (i32, Option<ChessMove>, Vec<ChessMove>) {
        let entry = self.transposition_table.get(board.get_hash());
        self.pos_counter += 1;
        if entry != None {
            let actual_entry = entry.unwrap();
            match actual_entry.lower_bound {
                Some(score) => {
                    if score >= beta {
                        self.cut_off_counter += 1;
                        return (score, actual_entry.mov, vec![]);
                    } else {
                        alpha = alpha.max(score);
                    }
                }
                None => {}
            }

            match actual_entry.upper_bound {
                Some(score) => {
                    if score <= alpha {
                        self.cut_off_counter += 1;
                        return (score, actual_entry.mov, vec![]);
                    } else {
                        beta = beta.min(score);
                    }
                }
                None => {}
            }
        }

        let mut move_gen = MoveGen::new_legal(&board);
        match move_gen.len() {
            0 => {
                return if board.checkers().0 == 0 {
                    (0, None, vec![])
                } else {
                    if board.side_to_move() == White {
                        //println!("{}", board.to_string());
                        (-10_000_000, None, vec![])
                    } else {
                        //println!("{}", board.to_string());
                        (10_000_000, None, vec![])
                    }
                };
            }
            _ => {} // else do nothing
        }

        let r_table_entry = self.repeat_table.get(board.get_hash()).or(Some(0));

        if r_table_entry == Some(2u16) {
            return (0, None, vec![]);
        }

        self.repeat_table.add(board.get_hash(), r_table_entry.or(Some(0)).unwrap() + 1);

        let moves = self.sort_moves(&board, &mut move_gen, entry);
        let mut moves_iter = moves.iter();


        //println!("{:?}", entry);

        if entry != None && entry.unwrap().node_type == PV_NODE {
            println!("pv move {}", entry.unwrap().mov.unwrap());
        }

        if depth == 0 {
            let score = evaluate(&board); //quiesce_search(alpha, beta, &board);
            return (score, None, vec![]);
        }


        let mut score = 0;
        let mut best_move: Option<ChessMove> = None;
        let mut mov = moves_iter.next();
        let mut mov_stack = Vec::new();


        if maximize {
            score = i32::MIN;
            let mut a = alpha;
            while (score < beta) && (mov != None) {
                let (tmp_score, _, move_stack) = self.alpha_beta(depth - 1, a, beta, false, board.make_move_new(*mov.unwrap()));
                if tmp_score > score {
                    score = tmp_score;
                    best_move = Some(*mov.unwrap());
                    mov_stack = move_stack;
                }
                a = a.max(score);
                mov = moves_iter.next();
            }
        } else {
            score = i32::MAX;
            let mut b = beta;
            while (score > alpha) && (mov != None) {
                let (tmp_score, _, move_stack) = self.alpha_beta(depth - 1, alpha, b, true, board.make_move_new(*mov.unwrap()));
                if tmp_score < score {
                    score = tmp_score;
                    best_move = Some(*mov.unwrap());
                    mov_stack = move_stack;
                }
                b = b.min(score);
                mov = moves_iter.next();
            }
        }

        if score <= alpha {
            self.cut_off_counter += 1;
            self.transposition_table.add(board.get_hash(),
                            TranspositionEntry { mov: best_move, lower_bound: None, upper_bound: Some(score), score: None, node_type: ALL_NODE, depth })
        }
        if score > alpha && score < beta {
            self.transposition_table.add(board.get_hash(),
                            TranspositionEntry { mov: best_move, lower_bound: Some(score), upper_bound: Some(score), score: Some(score), node_type: PV_NODE, depth })
        }
        if score >= beta {
            self.cut_off_counter += 1;
            self.transposition_table.add(board.get_hash(),
                            TranspositionEntry { mov: best_move, lower_bound: Some(score), upper_bound: None, score: None, node_type: CUT_NODE, depth })
        }
        mov_stack.push(best_move.unwrap());
        return (score, best_move, mov_stack);
    }
}

struct SortedMoveGen {
    move_gen: MoveGen,
    total_size: usize,
    iterated_size: usize,
}

impl SortedMoveGen {
    fn new(move_gen: MoveGen, board: &Board) -> SortedMoveGen {
        let len = move_gen.len();

        let mut s_mov_gen = SortedMoveGen {
            move_gen,
            total_size: len,
            iterated_size: 0,
        };

        let color_captures = if board.side_to_move() == White {
            Black
        } else {
            White
        };

        s_mov_gen.move_gen.set_iterator_mask(*board.color_combined(color_captures));

        return s_mov_gen;
    }
}

impl Iterator for SortedMoveGen {
    type Item = ChessMove;
    fn next(&mut self) -> Option<ChessMove> {
        let mut tmp_move = self.move_gen.next();

        // Set new values for iterating
        if self.iterated_size < self.total_size && tmp_move == None {
            self.move_gen.set_iterator_mask(!EMPTY);
            tmp_move = self.move_gen.next();
        }

        self.iterated_size += 1;

        return tmp_move;
    }
}

fn get_capture_moves(board: &Board) -> Vec<ChessMove> {
    let mut move_gen = MoveGen::new_legal(board);
    let mut ret_val = Vec::new();
    let color_captures = if board.side_to_move() == White {
        Black
    } else {
        White
    };

    move_gen.set_iterator_mask(*board.color_combined(color_captures));
    move_gen.collect_into(&mut ret_val);

    ret_val.sort_by(|mov1, mov2| {
        (PIECE_VALUES[board.piece_on(mov1.get_dest()).unwrap().to_index()]
            - PIECE_VALUES[board.piece_on(mov1.get_source()).unwrap().to_index()])
            .cmp(
                &(PIECE_VALUES[board.piece_on(mov2.get_dest()).unwrap().to_index()]
                    - PIECE_VALUES[board.piece_on(mov2.get_source()).unwrap().to_index()])
            )
    });

    ret_val.reverse();

    return ret_val;
}


fn quiesce_search(mut alpha: i32, mut beta: i32, board: &Board) -> i32 {
    let standing_pat = evaluate(board);
    if standing_pat >= beta {
        return beta;
    }
    if alpha < standing_pat {
        alpha = standing_pat;
    }

    let c_moves = get_capture_moves(board);
    let mut score = 0;

    if c_moves.len() == 0 {
        return standing_pat;
    }

    for mov in c_moves {
        score = -quiesce_search(beta, alpha, &board.make_move_new(mov));

        if score >= beta {
            return beta;
        }
        if score < alpha {
            alpha = score
        }
    }


    return alpha;
}
