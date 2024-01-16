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
    board = chess.Board()#chess.Board("8/2R3pk/8/8/6P1/P1K5/8/8 w - - 3 73")
    while not board.is_game_over():
        result = engine.play(board, chess.engine.Limit(depth=4))
        board.push(result.move)
        print(board)
        print(chess.pgn.Game.from_board(board))


if __name__ == '__main__':
    main()
