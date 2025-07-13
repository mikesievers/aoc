import numpy as np


class Map:
    _data = np.ndarray
    _antennas = dict
    _antinodes = list[tuple[int, int]]

    def __init__(self, fname: str):
        with open(f"src/app/day08/{fname}") as f:
            # Read all lines, but be sure to avoid worst cases like CR/LF on a line
            lines = [list(line.strip()) for line in f.readlines() if len(line) > 2]
            self._data = np.array(lines)
            self.find_antennas()
            self.find_antinodes()

    def find_antennas(self):
        self._antennas = dict()
        assert isinstance(self._data, np.ndarray)
        (max_y, max_x) = self._data.shape
        for y in range(max_y):
            for x in range(max_x):
                curr_pos = self._data[y, x]
                if np.char.isalnum(curr_pos):
                    if not self._antennas.get(curr_pos):
                        self._antennas[curr_pos] = []

                    self._antennas[curr_pos].append((int(y), int(x)))

    def find_antinodes(self):
        self._antinodes = []
