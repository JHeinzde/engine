import asyncio
import multiprocessing

import chess
import sys

from Engine import Engine  # Assuming you have an Engine class in the Engine module


class UCIProtocolHandler:
    def __init__(self, inp, out, pool):
        self.inp = inp
        self.out = out
        self.board = chess.Board()
        self.debug = False
        self.engine = Engine(self.board, pool)

    async def get_streams(self):
        loop = asyncio.get_event_loop()
        reader = asyncio.StreamReader()
        protocol = asyncio.StreamReaderProtocol(reader)
        await loop.connect_read_pipe(lambda: protocol, sys.stdin)
        w_transport, w_protocol = await loop.connect_write_pipe(asyncio.streams.FlowControlMixin, sys.stdout)
        writer = asyncio.StreamWriter(w_transport, w_protocol, reader, loop)
        return writer, reader

    async def write(self, data):
        self.out.write(data.encode())
        await self.out.drain()

    async def handle_uci(self):
        await self.write("id name KekChess 1.0\n")
        await self.write("id author Jonathan Heinz\n")
        await self.write("uciok\n")

    async def handle_debug(self, argument):
        if "on" in argument:
            self.debug = True
        elif "off" in argument:
            self.debug = False
        else:
            raise RuntimeError("Invalid UCI argument encountered!")

    async def handle_isready(self):
        await self.write("readyok\n")

    async def handle_setoption(self):
        # Currently no options can be set
        pass

    async def handle_position(self, argument):
        if b'startpos' in argument:
            self.board = chess.Board()
        if b'fen' in argument:
            fen = argument.split(' ')[1]
            self.board = self.board.set_fen(fen)
        if b'moves' in argument:
            moves = argument.split(b'moves ')[1]
            moves = moves.split(b' ')
            for move in moves:
                self.board.push_uci(move.decode('latin-1'))

    async def handle_go(self):
        self.engine.board = self.board
        move, eval = self.engine.analyze_concurrent()
        await self.write(f"info eval {eval}\n")
        await self.write(f"bestmove {move.uci()}\n")

    async def handle_stop(self):
        # We currently do not support stopping
        pass

    async def handle_ponderhit(self):
        # We currently have no pondering
        pass

    async def handle_quit(self):
        # Quit the engine process
        exit(0)

    async def listen(self):
        out, inp = await self.get_streams()
        self.out = out
        self.inp = inp
        while True:
            line = await self.inp.readline()
            command, *arguments = line.strip().split(b' ')
            argument = b' '.join(arguments).strip()

            if command == b"uci":
                await self.handle_uci()
            elif command == b"debug":
                await self.handle_debug(argument)
            elif command == b"isready":
                await self.handle_isready()
            elif command == b"setoption":
                await self.handle_setoption()
            elif command == b"position":
                await self.handle_position(argument)
            elif command == b"go":
                await self.handle_go()
            elif command == b"stop":
                await self.handle_stop()
            elif command == b"ponderhit":
                await self.handle_ponderhit()
            elif command == b"quit":
                await self.handle_quit()


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    with  multiprocessing.Pool(10) as pool:
        handler = UCIProtocolHandler(sys.stdin, sys.stdout, pool)
        loop.run_until_complete(handler.listen())
