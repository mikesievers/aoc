"""
Day 4 requires detecting word in various directions:
- horizontal
- vertical
- diagonal
- forward + backward

Note:
- Words will be found at least twice (from beginning and from end)
- 2 or more words may begin/end in one point
  - It might be better to identify words by (start,end) tuples collected in a set

The possible directions to check (o is the center):

       N
      WoE
       S

To avoid duplicates, only half the directions will be checked:


       N  NE

       o  E

          SE

"""

from enum import Enum

import numpy as np


class XY_INC(Enum):
    """Increments in X, Y for each direction"""

    N = (-1, 0)
    NE = (-1, 1)
    E = (0, 1)
    SE = (1, 1)


def read_grid(fname: str) -> np.array:
    data = []
    with open(fname, "r") as f:
        lines = f.readlines()
        for line in lines:
            data.append(list(line.strip()))

    return np.array(data)


def get_start_and_end(
    grid: np.array, check_position: tuple[int, int], check_direction: XY_INC
) -> tuple[tuple[int, int], tuple[int, int]]:
    """
    If "XMAS" is found starting or ending at this grid point,
    return start and end point.
    Else, return None, None
    """

    XMAS = list("XMAS")

    y0 = check_position[0]
    x0 = check_position[1]
    # The grid is (row, column), i.e. (y, x)
    check_character = grid[y0, x0]
    # Ignore obviously impossible situations
    if check_character not in ["X", "S"]:
        return None, None

    delta_y = check_direction.value[0]
    delta_x = check_direction.value[1]
    characters = [grid[y0 + delta_y * n, x0 + delta_x * n] for n in range(4)]

    end_position = y0 + delta_y * 3, x0 + delta_x * 3
    if characters == XMAS:
        return check_position, end_position
    if list(reversed(characters)) == XMAS:
        return end_position, check_position

    return None, None


def possible_directions(
    grid: np.ndarray, check_position: tuple[int, int]
) -> list[XY_INC]:
    """Return the possible directions for a search
    Ensure no search is done over the edges of the grid
    """
    check_y = check_position[0]
    check_x = check_position[1]

    y_max = grid.shape[0] - 1
    x_max = grid.shape[1] - 1

    directions = []
    if check_y >= 3:
        directions.append(XY_INC.N)
    if check_y >= 3 and check_x <= x_max - 3:
        directions.append(XY_INC.NE)
    if check_x <= x_max - 3:
        directions.append(XY_INC.E)
    if check_y <= y_max - 3 and check_x <= x_max - 3:
        directions.append(XY_INC.SE)

    return directions


def count_occurrences(fname: str) -> int:
    grid = read_grid(fname)

    occurrences = []
    for y in range(grid.shape[0]):
        for x in range(grid.shape[1]):
            for direction in possible_directions(grid=grid, check_position=(y, x)):
                (start, end) = get_start_and_end(
                    grid=grid, check_position=(y, x), check_direction=direction
                )
                if start and end:
                    occurrences.append((start, end))

    return len(occurrences)


def count_occurrences_x(fname: str) -> int:
    grid = read_grid(fname)

    occurrences = 0
    for y in range(1, grid.shape[0] - 1):
        for x in range(1, grid.shape[1] - 1):
            if is_x_mas(grid, check_pos=(x, y)):
                occurrences += 1

    return occurrences


def is_x_mas(grid: np.ndarray, check_pos: tuple[int, int]) -> bool:
    y0 = check_pos[0]
    x0 = check_pos[1]

    center = grid[y0, x0]
    if center != "A":
        return False

    char_nw = grid[y0 - 1, x0 - 1]
    char_ne = grid[y0 - 1, x0 + 1]
    char_se = grid[y0 + 1, x0 + 1]
    char_sw = grid[y0 + 1, x0 - 1]

    if list(sorted([char_nw, center, char_se])) != list("AMS"):
        return False
    if list(sorted([char_ne, center, char_sw])) != list("AMS"):
        return False

    # All impossible situations eliminated, we must have an X-MAS:
    return True
