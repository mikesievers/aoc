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
            self._antennas = self.find_antennas(self._data)
            self.find_antinodes()

    @staticmethod
    def find_antennas(data: np.ndarray) -> dict[str, tuple[int, int]]:
        antennas = dict()
        (max_y, max_x) = data.shape
        for y in range(max_y):
            for x in range(max_x):
                curr_pos = data[y, x]
                if np.char.isalnum(curr_pos):
                    if not antennas.get(curr_pos):
                        antennas[curr_pos] = []

                    antennas[curr_pos].append((int(y), int(x)))
        return antennas

    def find_antinodes(self):
        self._antinodes = []
