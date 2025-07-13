from itertools import combinations

import numpy as np


class Map:
    _data = np.ndarray
    _antennas = dict
    _antinodes = set[tuple[int, int]]

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
        self._antinodes = set()
        assert isinstance(self._data, np.ndarray)
        (x_max, y_max) = self._data.shape
        assert isinstance(self._antennas, dict)

        for _, antenna_locations in self._antennas.items():
            for a1, a2 in combinations(antenna_locations, 2):
                delta_y = a1[0] - a2[0]
                delta_x = a1[1] - a2[1]

                cand1_y = a1[0] + delta_y
                cand1_x = a1[1] + delta_x

                cand2_y = a2[0] - delta_y
                cand2_x = a2[1] - delta_x

                # Add the two antinode candidates if they are on the map
                if (
                    (cand1_x >= 0)
                    and (cand1_y >= 0)
                    and (cand1_x < x_max)
                    and (cand1_y < y_max)
                ):
                    self._antinodes.add((cand1_y, cand1_x))

                if (
                    (cand2_x >= 0)
                    and (cand2_y >= 0)
                    and (cand2_x < x_max)
                    and (cand2_y < y_max)
                ):
                    self._antinodes.add((cand2_y, cand2_x))
