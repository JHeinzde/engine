import chess
import chess.engine
import chess.svg

import logging

# Enable debug logging.
logging.basicConfig(level=logging.DEBUG)

engine = chess.engine.SimpleEngine.popen_uci(
    ["/Users/jonathan/Projekte/chess-engine/target/debug/chess-engine"])


def main():
    board = chess.Board()
    while not board.is_game_over():
        result = engine.play(board, chess.engine.Limit(depth=4))
        board.push(result.move)
        #svg = chess.svg.board(
        #    board,
        #    squares=chess.SquareSet(chess.BB_DARK_SQUARES & chess.BB_FILE_B),
        #    size=350)
        print(board)


if __name__ == '__main__':
    main()
