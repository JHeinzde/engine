import chess
import chess.engine
import chess.svg
import chess.pgn

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
        print(board)
        print(chess.pgn.Game.from_board(board))


if __name__ == '__main__':
    main()
