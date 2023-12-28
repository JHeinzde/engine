
from multiprocessing import freeze_support

import chess

PIECE_VALUES = {
    chess.PAWN: 1.0,
    chess.BISHOP: 3.0,
    chess.KNIGHT: 3.0,
    chess.ROOK: 5.0,
    chess.QUEEN: 9.0,
    chess.KING: 0.0
}


class Engine:
    def __init__(self, board: chess.Board, pool, manager):
        self.board = board
        self.pool = pool
        self.manager = manager

    def analyze_concurrent(self):
        if self.board.turn == chess.WHITE:
            maximize = True
        else:
            maximize = False

        best_move = None
        best_score = float('-inf') if maximize else float('inf')

        pos_map = {} #self.manager.dict()
        depth = 5

        moves = list(self.board.legal_moves)
        tasks = [(move, depth, maximize, self.board.copy(), pos_map) for move in moves]

        for args in tasks:
            move, score  = call_analyze(args)
            if maximize and score >= best_score:
                best_score = score
                best_move = move
            elif not maximize and score <= best_score:
                best_score = score
                best_move = move
        return best_move, best_score


def evaluate(board: chess.Board):
    if outcome := board.outcome():
        if outcome.winner == chess.BLACK:
            return -1000.0
        if outcome.winner == chess.WHITE:
            return 1000.0
        if outcome.result == '1/2-1/2':
            return 0.0

    score = 0.0

    for (square, piece) in board.piece_map().items():
        if piece.color == chess.WHITE:
            score += PIECE_VALUES[piece.piece_type]
        else:
            score -= PIECE_VALUES[piece.piece_type]

        #if piece.piece_type == chess.PAWN:
        #    file_index = chess.square_file(square)
        #    rank_index = chess.square_rank(square)
        #    # Isolated Pawn Check
        #    if not any(board.piece_at(neighbor) for neighbor in chess.SquareSet(chess.BB_FILES[file_index]) & chess.SquareSet(chess.BB_RANKS[rank_index])):
        #        isolated_pawns += 1 if piece.color == chess.WHITE else -1
#
        #    # Doubled Pawn Check
        #    if len(chess.SquareSet(board.pieces(piece.piece_type, piece.color) & chess.BB_FILES[rank_index])) > 1:
        #        doubled_pawns +=  1 if piece.color == chess.WHITE else -1
#
        #    # Blocked Pawn Check
        #    if board.piece_at(square + 8) is not None:  # Check if there's a piece in front of the pawn
        #        blocked_pawns += 1 if piece.color == chess.WHITE else -1

    #white_mobility = len(list(board.legal_moves))
    #board.turn = chess.BLACK
    #black_mobility = len(list(board.legal_moves))
    #board.turn = chess.WHITE
    ##score -= 0.5 * (doubled_pawns + blocked_pawns + isolated_pawns)
    #score += 0.1 * (white_mobility - black_mobility)

    return score


def call_analyze(args):
    move, depth, maximize, board, pos_map = args
    board.push(move)
    return move, alpha_beta(depth, float('-inf'), float('inf'), maximize, board, pos_map)


def alpha_beta(depth, alpha, beta, maximize, board: chess.Board, pos_map):
    # If we already have an evaluation for the position return that instead
    if bhash := hash_board(board) in pos_map.keys():
        return pos_map[bhash]

    if depth == 0 or board.is_game_over():
        score = evaluate(board)
        pos_map[bhash] = score
        return evaluate(board)
    if maximize:
        score = float('-inf')
        for move in list(board.legal_moves):
            board.push(move)
            score = max(score, alpha_beta(depth - 1, alpha, beta, False, board, pos_map))
            if score > beta:
                board.pop()
                break
            alpha = max(alpha, score)
            board.pop()
        pos_map[bhash] = score
        return score
    else:
        score = float('inf')
        for move in list(board.legal_moves):
            board.push(move)
            score = min(score, alpha_beta(depth - 1, alpha, beta, True, board, pos_map))
            if score < alpha:
                board.pop()
                break
            beta = min(beta, score)
            board.pop()
        pos_map[bhash] = score
        return score


# Converts fen into hashmap key
def hash_board(board: chess.Board):
    hash = board.pawns
    hash ^= board.rooks
    hash ^= board.knights
    hash ^= board.bishops
    hash ^= board.queens
    hash ^= board.kings
    hash ^= board.castling_rights
    hash ^= board.ep_square if board.ep_square else 0
    hash ^= board.turn
    return hash

if __name__ == '__main__':
    freeze_support()
