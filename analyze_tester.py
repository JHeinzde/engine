import Engine
import chess

board = chess.Board()
engine = Engine.Engine(board)

print(engine.analyze())
