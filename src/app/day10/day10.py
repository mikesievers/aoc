"""
goal: find trails
  - start at trailhead ('0')
  - via vertical and horizontal neighbors that are larger by 1
  - to trail end ('9')
  - then score the trail heads (nr of 9s reachable from the 0) and add the score


Thoughts:
- The map spans a directed graph
- Nodes are connected by +1 edges
- Multiple paths from one trailhead can lead to the same end
-

Procedure:
- Inspect each cell for being a trailhead
- find all viable neighbors and follow them recursively until a '9'
- add the position of each 9 to a set related to the trailhead

"""

from enum import Enum

import numpy as np


class Direction:
    UP = np.array([-1, 0])
    RIGHT = np.array([0, 1])
    DOWN = np.array([1, 0])
    LEFT = np.array([0, -1])


class Map:
    _map: np.ndarray
    _trails: dict[tuple[int, int], set[tuple[int, int]]]

    def __init__(self, fname):
        with open(f"src/app/day10/{fname}") as f:
            lines = [list(line.strip()) for line in f.readlines() if len(line) > 2]
        self._map = np.array(lines, dtype=np.int8)
        self._trails = dict()

    def follow_trail(self, y: int, x: int, orig_y: int = -1, orig_x: int = -1) -> None:
        """
        Follow the trail starting at y, x by finding all possible next
        directions and following them in turn.
        If the current position is a '9', remember it as a trail end.
        """
        # Remember the origin
        if orig_y < 0:
            (orig_y, orig_x) = (y, x)

        # Remember the trail end
        if self._map[y, x] == 9:
            if self._trails.get((orig_y, orig_x), None) is None:
                self._trails[(orig_y, orig_x)] = set()
            self._trails[(orig_y, orig_x)].add((y, x))
            return

        position = np.array([y, x])
        current_height = self._map[y, x]
        # Check which directions are viable
        possible_directions = []
        # Up
        position_up = position + Direction.UP
        if (
            self.get_map_value_if_valid(position_up[0], position_up[1])
            == current_height + 1
        ):
            possible_directions.append(tuple(position_up))
        # Right
        position_right = position + Direction.RIGHT
        if (
            self.get_map_value_if_valid(position_right[0], position_right[1])
            == current_height + 1
        ):
            possible_directions.append(tuple(position_right))
        # Down
        position_down = position + Direction.DOWN
        if (
            self.get_map_value_if_valid(position_down[0], position_down[1])
            == current_height + 1
        ):
            possible_directions.append(tuple(position_down))
        # Left
        position_left = position + Direction.LEFT
        if (
            self.get_map_value_if_valid(position_left[0], position_left[1])
            == current_height + 1
        ):
            possible_directions.append(tuple(position_left))

        # Recurse
        for direction in possible_directions:
            self.follow_trail(direction[0], direction[1], orig_y, orig_x)
        return

    def get_map_value_if_valid(self, y: int, x: int) -> int:
        """
        Return map value at indicated position if it is a valid map position
        Return -1 otherwise
        """
        (y_max, x_max) = self._map.shape
        if y >= 0 and y < y_max and x >= 0 and x < x_max:
            return self._map[y, x]
        return -1

    def follow_all_trails(self):
        (y_max, y_min) = self._map.shape
        for y in range(y_max):
            for x in range(y_min):
                if self._map[y, x] == 0:
                    self.follow_trail(y, x)

    def sum_score(self):
        score = 0
        for _, trail_ends in self._trails.items():
            score += len(trail_ends)
        return score
