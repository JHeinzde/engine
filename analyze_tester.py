import Engine
import chess
import multiprocessing


if __name__ == '__main__':
    pool = multiprocessing.Pool(10)
    manager = multiprocessing.Manager()
    board = chess.Board()
    engine = Engine.Engine(board, pool, manager)

    print(engine.analyze_concurrent())
