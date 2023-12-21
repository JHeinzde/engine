import chess
import multiprocessing

PIECE_VALUES = {
    chess.PAWN: 1.0,
    chess.BISHOP: 3.0,
    chess.KNIGHT: 3.0,
    chess.ROOK: 5.0,
    chess.QUEEN: 9.0,
    chess.KING: 0.0
}


class Engine:
    def __init__(self, board):
        self.board = board

    def evaluate(self):
        if outcome := self.board.outcome():
            if outcome.winner == chess.BLACK:
                return -1000.0
            if outcome.winner == chess.WHITE:
                return 1000.0
            if outcome.result == '1/2-1/2':
                return 0.0

        score_white = 0.0
        score_black = 0.0

        center_squares = [chess.E4, chess.D4, chess.E5, chess.D5]
        safe_squares_white = [chess.G1, chess.H1, chess.C1, chess.B1, chess.A1]
        safe_squares_black = [chess.G8, chess.H8, chess.C8, chess.B8, chess.A8]

        for (square, piece) in self.board.piece_map().items():
            if piece.color == chess.WHITE:
                score_white += PIECE_VALUES[piece.piece_type]
                if square in center_squares:
                    score_white += 0.1 * PIECE_VALUES[piece.piece_type]
                score_white += 0.05 * len(self.board.attacks(square))
            else:
                score_black += PIECE_VALUES[piece.piece_type]
                if square in center_squares:
                    score_black += 0.1 * PIECE_VALUES[piece.piece_type]
                score_black += 0.05 * len(self.board.attacks(square))
        if self.board.king(chess.WHITE) in safe_squares_white:
            score_white += 0.5
        if self.board.king(chess.BLACK) in safe_squares_black:
            score_black += 0.5
        if self.board.king(chess.WHITE) not in safe_squares_white and not self.board.has_castling_rights(chess.WHITE):
            score_white -= 0.75
        if self.board.king(chess.BLACK) not in safe_squares_white and not self.board.has_castling_rights(chess.BLACK):
            score_black -= 0.75
        return score_white - score_black

    def analyze(self):
        if self.board.turn == chess.WHITE:
            maximize = True
        else:
            maximize = False

        best_move = None
        best_score = float('-inf') if maximize else float('inf')

        for move in self.board.legal_moves:
            self.board.push(move)
            score = self.alpha_beta(6, float('-inf'), float('inf'), not maximize)
            if maximize and score >= best_score:
                best_score = score
                best_move = move
            elif not maximize and score <= best_score:
                best_score = score
                best_move = move
            self.board.pop()
        return best_move, best_score

    def alpha_beta(self, depth, score_white, score_black, maximize):
        if depth == 0 or self.board.is_game_over():
            return self.evaluate()
        if maximize:
            score = float('-inf')
            for move in list(self.board.legal_moves):
                self.board.push(move)
                score = max(score, self.alpha_beta(depth - 1, score_white, score_black, False))
                if score > score_black:
                    self.board.pop()
                    break
                score_white = max(score_white, score)
                self.board.pop()
            return score
        else:
            score = float('inf')
            for move in list(self.board.legal_moves):
                self.board.push(move)
                score = min(score, self.alpha_beta(depth - 1, score_white, score_black, True))
                if score < score_white:
                    self.board.pop()
                    break
                score_black = min(score_black, score)
                self.board.pop()
            return score
