use std::hash::Hash;
use std::ops::BitAnd;

use chess::{BitBoard, Board, CacheTable, ChessMove, Color, MoveGen, Piece, Square};
use chess::Color::White;
use Color::Black;


const WHITE_PAWN: [i32; 64] = [
    // Rank 1
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 2
    30, 30, 30, -20, -20, 30, 30, 30,
    // Rank 3
    30, 30, 30, -10, -10, -20, 5, 5,
    // Rank 4
    10, 10, 50, 60, 60, 10, 5, 5,
    // Rank 5
    10, 10, 40, 50, 50, 40, 10, 10,
    // Rank 6
    20, 20, 30, 30, 30, 30, 20, 20,
    // Rank 7
    80, 80, 80, 80, 80, 80, 80, 80,
    // Rank 8
    0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_PAWN: [i32; 64] = [
    // Rank 1
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 2
    -80, -80, -80, -80, -80, -80, -80, -80,
    // Rank 3
    -20, -20, -30, -30, -30, -30, -20, -20,
    // Rank 4
    -10, -10, -40, -50, -50, -40, -10, -10,
    // Rank 5
    -10, -10, -50, -60, -60, -10, -5, -5,
    // Rank 6
    -30, -30, -30, 10, 10, 20, -5, -5,
    // Rank 7
    -30, -30, -30, 20, 20, -30, -30, -30,
    // Rank 8
    0, 0, 0, 0, 0, 0, 0, 0,
];

const WHITE_ROOK: [i32; 64] = [
    // Rank 1
    -30, -30, 10, 30, 30, 10, -30, -30,
    // Rank 2
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 3
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 4
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 5
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 6
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 7
    20, 20, 20, 20, 20, 20, 20, 20,
    // Rank 8
    0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_ROOK: [i32; 64] = [
    // Rank 1
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 2
    -20, -20, -20, -20, -20, -20, -20, -20,
    // Rank 3
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 4
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 5
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 6
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 7
    0, 0, 0, 0, 0, 0, 0, 0,
    // Rank 8
    30, 30, -10, -30, -30, -10, 30, 30,
];

const WHITE_KNIGHT: [i32; 64] = [
    // Rank 1
    -30, -30, -30, -30, -30, -30, -30, -30,
    // Rank 2
    -30, -30, 10, -10, -10, 10, -30, -30,
    // Rank 3
    -30, -30, 20, 25, 25, 20, -30, -30,
    // Rank 4
    -30, -30, 30, 50, 50, 30, -30, -30,
    // Rank 5
    -30, -30, 30, 50, 50, 30, -30, -30,
    // Rank 6
    -30, -30, 20, 30, 30, 20, -30, -30,
    // Rank 7
    -30, -30, 10, 20, 20, 10, -30, -30,
    // Rank 8
    -15, -15, -15, -10, -10, -15, -15, -15,
];

const BLACK_KNIGHT: [i32; 64] = [
    // Rank 1
    15, 15, 15, 10, 10, 15, 15, 15,
    // Rank 2
    30, 30, -10, -20, -20, -10, 30, 30,
    // Rank 3
    30, 30, -20, -30, -30, -20, 30, 30,
    // Rank 4
    30, 30, -30, -50, -50, -30, 30, 30,
    // Rank 5
    30, 30, -30, -50, -50, -30, 30, 30,
    // Rank 6
    30, 30, -20, -25, -25, -20, 30, 30,
    // Rank 7
    30, 30, -10, 10, 10, -10, 30, 30,
    // Rank 8
    30, 30, 30, 30, 30, 30, 30, 30,
];

const WHITE_BISHOP: [i32; 64] = [
    // Rank 1
    -30, -30, -30, -30, -30, -30, -30, -30,
    // Rank 2
    -10, 40, 20, 10, 10, -30, 40, -10,
    // Rank 3
    -30, 20, -10, 30, 30, -10, 20, -30,
    // Rank 4
    -30, -30, 30, 30, 30, 30, -30, -30,
    // Rank 5
    -30, 30, 10, 50, 50, 10, 30, -30,
    // Rank 6
    -30, -30, 50, 10, 10, 50, -30, -30,
    // Rank 7
    -30, -30, 10, 20, 20, 10, -30, -30,
    // Rank 8
    -20, -20, -20, -20, -20, -20, -20, -20,
];

const BLACK_BISHOP: [i32; 64] = [
    // Rank 1
    20, 20, 20, 20, 20, 20, 20, 20,
    // Rank 2
    30, 30, -10, -20, -20, -10, 30, 30,
    // Rank 3
    30, 30, -50, -10, -10, -50, 30, 30,
    // Rank 4
    30, 30, -30, -30, -30, -30, 30, 30,
    // Rank 5
    30, -30, -10, -50, -50, -10, 30, 30,
    // Rank 6
    30, -20, 10, -30, -30, 10, -20, 30,
    // Rank 7
    10, -40, -20, -10, -10, 30, -40, 10,
    // Rank 8
    30, 30, 30, 30, 30, 30, 30, 30,
];

const WHITE_QUEEN: [i32; 64] = [
    // Rank 1
    -30, -30, -30, -30, -30, -30, -30, -30,
    // Rank 2
    -30, -30, -30, -30, -30, -30, -30, -30,
    // Rank 3
    -30, -30, 30, 30, 30, 30, -30, -30,
    // Rank 4
    -30, -30, 40, 40, 40, 40, -30, -30,
    // Rank 5
    -30, -30, 40, 40, 40, 40, -30, -30,
    // Rank 6
    -30, -30, 30, 30, 30, 30, -30, -30,
    // Rank 7
    -30, -30, 30, 30, 30, 30, -30, -30,
    // Rank 8
    -30, -30, -30, -30, -30, -30, -30, -30,
];

const BLACK_QUEEN: [i32; 64] = [
    // Rank 1
    30, 30, 30, 30, 30, 30, 30, 30,
    // Rank 2
    30, 30, -30, -30, -30, -30, 30, 30,
    // Rank 3
    30, 30, -30, -30, -30, -30, 30, 30,
    // Rank 4
    30, 30, -40, -40, -40, -40, 30, 30,
    // Rank 5
    30, 30, -40, -40, -40, -40, 30, 30,
    // Rank 6
    30, 30, -30, -30, -30, -30, 30, 30,
    // Rank 7
    30, 30, -30, -30, -30, -30, 30, 30,
    // Rank 8
    30, 30, 30, 30, 30, 30, 30, 30,
];

const WHITE_KING: [i32; 64] = [
    // Rank 1
    20, 30, 20, -30, -30, 20, 30, 20,
    // Rank 2
    -10, -10, -30, -30, -30, -30, -30, -30,
    // Rank 3
    -50, -50, -50, -50, -50, -50, -50, -50,
    // Rank 4
    -50, -50, -50, -50, -50, -50, -50, -50,
    // Rank 5
    -50, -50, -50, -50, -50, -50, -50, -50,
    // Rank 6
    -50, -50, -50, -50, -50, -50, -50, -50,
    // Rank 7
    -50, -50, -50, -50, -50, -50, -50, -50,
    // Rank 8
    -50, -50, -50, -50, -50, -50, -50, -50,
];

const BLACK_KING: [i32; 64] = [
    // Rank 1
    50, 50, 50, 50, 50, 50, 50, 50,
    // Rank 2
    10, 10, 30, 30, 30, 30, 30, 30,
    // Rank 3
    50, 50, 50, 50, 50, 50, 50, 50,
    // Rank 4
    50, 50, 50, 50, 50, 50, 50, 50,
    // Rank 5
    50, 50, 50, 50, 50, 50, 50, 50,
    // Rank 6
    50, 50, 50, 50, 50, 50, 50, 50,
    // Rank 7
    50, 50, 50, 50, 50, 50, 50, 50,
    // Rank 8
    -20, -30, -20, 30, 30, -20, -30, -20,
];

enum NodeType {
    EXACT,
    ALL,
    CUT
}

struct TranspositionEntry {
    score: i32,
    mov: ChessMove,
    node_type: NodeType,
    depth: u16
}


fn evaluate(board: &Board) -> i32 {
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

    score = (white_pawns_count - black_pawns_count) * 100 + (white_bishops_count - black_bishops_count) * 320 +
        (white_knights_count - black_knights_count) * 290 + (white_rooks_count - black_rooks_count) * 500 +
        (white_queen_count - black_queen_count) * 900;
    

    score += iterate_bitboard(&mut white_pawns, |square: Square| WHITE_PAWN[square.to_index()]);
    score += iterate_bitboard(&mut black_pawns, |square: Square| BLACK_PAWN[square.to_index()]);
    score += iterate_bitboard(&mut white_bishops, |square: Square| WHITE_BISHOP[square.to_index()]);
    score += iterate_bitboard(&mut black_bishops, |square: Square| BLACK_BISHOP[square.to_index()]);
    score += iterate_bitboard(&mut black_knights, |square: Square| BLACK_KNIGHT[square.to_index()]);
    score += iterate_bitboard(&mut white_knights, |square: Square| WHITE_KNIGHT[square.to_index()]);
    score += iterate_bitboard(&mut white_rooks, |square: Square| WHITE_ROOK[square.to_index()]);
    score += iterate_bitboard(&mut black_rooks, |square: Square| BLACK_ROOK[square.to_index()]);
    score += iterate_bitboard(&mut white_queen, |square: Square| WHITE_QUEEN[square.to_index()]);
    score += iterate_bitboard(&mut black_queen, |square: Square| BLACK_QUEEN[square.to_index()]);
    score += iterate_bitboard(&mut white_king, |square: Square| WHITE_KING[square.to_index()]);
    score += iterate_bitboard(&mut black_king, |square: Square| BLACK_KING[square.to_index()]);


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

pub fn analyze(depth: u16, board: Board) -> (ChessMove, i32) {
    let mut cache_table = CacheTable::new(1048576, 0i32);

    let mut maximize = true;

    if board.side_to_move() == White {
        maximize = true;
    } else {
        maximize = false;
    }

    let mut best_score = if board.side_to_move() == White { i32::MIN } else { i32::MAX };
    let mut best_move: Option<ChessMove> = None;

    for mov in MoveGen::new_legal(&board) {
        let score = alpha_beta(depth - 1, i32::MIN, i32::MAX, !maximize, board.make_move_new(mov), &mut cache_table);
        if maximize {
            if score >= best_score {
                best_score = score;
                best_move = Some(mov)
            }
        } else {
            if score <= best_score {
                best_score = score;
                best_move = Some(mov);
            }
        }
    }

    return (best_move.unwrap(), best_score);
}

fn alpha_beta(depth: u16, mut alpha: i32, mut beta: i32, maximize: bool, board: Board, cache_table: &mut CacheTable<i32>) -> i32 {
    // If we already looked at the position return the score
    if cache_table.get(board.get_hash()) != None {
        return cache_table.get(board.get_hash()).unwrap();
    }
    let mut move_gen = MoveGen::new_legal(&board);
    match move_gen.len() {
        0 => {
            return if board.checkers().0 == 0 {
                0
            } else {
                if board.side_to_move() == White {
                    i32::MIN
                } else {
                    i32::MAX
                }
            };
        }
        _ => {} // else do nothing
    }
    if depth == 0 {
        let score = evaluate(&board);
        cache_table.add(board.get_hash(), score);
        return score;
    }
    return if maximize {
        let mut score = i32::MIN;
        for mov in move_gen {
            score = score.max(alpha_beta(depth - 1, alpha, beta, false, board.make_move_new(mov), cache_table));
            cache_table.add(board.get_hash(), score);
            if score >= beta {
                return beta;
            }
            alpha = alpha.max(score)
        }
        cache_table.add(board.get_hash(), score);
        score
    } else {
        let mut score = i32::MAX;
        for mov in move_gen {
            score = score.min(alpha_beta(depth - 1, alpha, beta, true, board.make_move_new(mov), cache_table));
            cache_table.add(board.get_hash(), score);
            if score <= alpha {
                return alpha;
            }
            beta = beta.min(score)
        }
        cache_table.add(board.get_hash(), score);
        score
    };
}
