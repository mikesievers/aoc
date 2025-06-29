import pytest

from app.day04.day04 import (
    XY_INC,
    count_occurrences,
    count_occurrences_x,
    get_start_and_end,
    possible_directions,
    read_grid,
)

FNAME_SAMPLE = "src/app/day04/sample.txt"
FNAME_INPUT = "src/app/day04/input.txt"


def test_read_grid():
    grid = read_grid(FNAME_SAMPLE)
    assert grid[0, 3] == "S"
    assert grid[2, 4] == "X"
    assert grid[-2, -5] == "X"


@pytest.mark.parametrize(
    "check_pos,check_dir,expected_start,expected_end",
    [
        ((9, 9), XY_INC.N, (9, 9), (6, 9)),
        ((5, 0), XY_INC.NE, (5, 0), (2, 3)),
        ((4, 0), XY_INC.E, (4, 0), (4, 3)),
        ((6, 0), XY_INC.SE, (9, 3), (6, 0)),
    ],
)
def test_get_start_and_end(check_pos, check_dir, expected_start, expected_end):
    grid = read_grid(FNAME_SAMPLE)

    (start, end) = get_start_and_end(
        grid=grid, check_position=check_pos, check_direction=check_dir
    )

    assert start == expected_start
    assert end == expected_end


@pytest.mark.parametrize(
    "check_pos,expected",
    [
        ((0, 0), [XY_INC.E, XY_INC.SE]),
        ((9, 0), [XY_INC.N, XY_INC.NE, XY_INC.E]),
        ((0, 9), []),
        ((9, 9), [XY_INC.N]),
        ((3, 6), [XY_INC.N, XY_INC.NE, XY_INC.E, XY_INC.SE]),
        ((2, 7), []),
    ],
)
def test_possible_directions(check_pos, expected):
    grid = read_grid(FNAME_SAMPLE)

    assert possible_directions(grid=grid, check_position=check_pos) == expected


@pytest.mark.parametrize("fname, expected", [(FNAME_SAMPLE, 18), (FNAME_INPUT, 2573)])
def test_count_occurrences(fname, expected):
    assert count_occurrences(fname) == expected


@pytest.mark.parametrize("fname, expected", [(FNAME_SAMPLE, 9), (FNAME_INPUT, 1850)])
def test_count_occurrences_x(fname, expected):
    assert count_occurrences_x(fname) == expected
