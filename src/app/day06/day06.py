from __future__ import annotations

from enum import StrEnum

import numpy as np

BASEPATH = "src/app/day06/"
PLAYER_UP = "^"


class Orientation(StrEnum):
    UP = "^"
    RIGHT = ">"
    DOWN = "v"
    LEFT = "<"


class Fields(StrEnum):
    EMPTY = "."
    OBSTRUCTION = "#"
    WALKED = "X"
    VOID = "O"


class Player:
    _x: int
    _y: int
    _orientation: Orientation

    def __init__(self, pos: tuple[int, int], orientation: Orientation):
        (self._y, self._x) = pos
        self._orientation = orientation

    def get_pos(self) -> tuple[int, int]:
        return (self._y, self._x)

    def get_orientation(self) -> Orientation:
        return self._orientation

    def move(self):
        if self._orientation == Orientation.UP:
            self._y -= 1
        if self._orientation == Orientation.RIGHT:
            self._x += 1
        if self._orientation == Orientation.DOWN:
            self._y += 1
        if self._orientation == Orientation.LEFT:
            self._x -= 1

    def turn_right(self):
        match self._orientation:
            case Orientation.UP:
                self._orientation = Orientation.RIGHT
            case Orientation.RIGHT:
                self._orientation = Orientation.DOWN
            case Orientation.DOWN:
                self._orientation = Orientation.LEFT
            case Orientation.LEFT:
                self._orientation = Orientation.UP


class Board:
    """A game board consisting of a grid."""

    _cells: np.ndarray
    _walked_cells: np.ndarray
    _player: Player
    _is_over: bool

    @staticmethod
    def from_file(fname: str) -> Board:
        with open(BASEPATH + fname) as infile:
            lines = [list(line.strip()) for line in infile.readlines()]

        # Set up the cells
        board = Board()
        board._is_over = False
        board._cells = np.array(lines)
        board._walked_cells = np.full(board._cells.shape, Fields.EMPTY)

        # find the player position and remember it,
        # Mark the player position on the board with a period
        (y_max, x_max) = board._cells.shape
        for y in range(y_max):
            for x in range(x_max):
                if board._cells[y, x] == Orientation.UP:
                    board._player = Player((y, x), Orientation.UP)
                    board._cells[y, x] = Fields.EMPTY
                    board._walked_cells[y, x] = Fields.WALKED

        return board

    def shape(self) -> tuple[int, int]:
        return self._cells.shape

    def make_move(self) -> None:
        # Look ahead
        next_field = self.look_ahead()

        # It's over when we fall off the edge
        if next_field == Fields.VOID:
            self._is_over = True
            return

        # Turn right until you see the light:
        while next_field == Fields.OBSTRUCTION:
            self._player.turn_right()
            next_field = self.look_ahead()

        # move player
        self._player.move()

        # mark field as walked
        (y_new, x_new) = self._player.get_pos()
        self._walked_cells[y_new, x_new] = Fields.WALKED

    def look_ahead(self) -> str:
        dir = self._player.get_orientation()
        match dir:
            case Orientation.UP:
                delta_y = -1
                delta_x = 0
            case Orientation.DOWN:
                delta_y = +1
                delta_x = 0
            case Orientation.RIGHT:
                delta_x = +1
                delta_y = 0
            case Orientation.LEFT:
                delta_x = -1
                delta_y = 0

        (pos_y, pos_x) = self._player.get_pos()
        (next_y, next_x) = (pos_y + delta_y, pos_x + delta_x)

        if (
            (next_x < 0 or next_y < 0)
            or (next_x >= self._cells.shape[1])
            or (next_y >= self._cells.shape[0])
        ):
            return Fields.VOID

        return self._cells[next_y, next_x]

    def count_walked(self) -> int:
        nr_walked_fields = 0
        y_max, x_max = self._walked_cells.shape
        for y in range(y_max):
            for x in range(x_max):
                if self._walked_cells[y, x] == Fields.WALKED:
                    nr_walked_fields += 1
        return nr_walked_fields
