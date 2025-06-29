import numpy as np
import pytest

from app.day01.day01 import add_distances, read_input, run_analysis, sort_lists


def test_read_input():
    # GIVEN
    fname = "src/app/day01/sample.txt"

    # WHEN
    data = read_input(fname)

    # THEN
    assert [data[2, 0], data[2, 1]] == [2, 5]
    assert [data[-1, 0], data[-1, 1]] == [3, 3]


def test_sort_lists():
    input = np.array([[3, 2], [1, 0], [8, 2]])
    expected = np.array([[1, 0], [3, 2], [8, 2]])

    assert (sort_lists(input) == expected).all()


def test_add_distances():
    input = np.array([[1, 0], [3, 2], [8, 2]])
    assert add_distances(input) == 1 + 1 + 6


@pytest.mark.parametrize("fname,expected", [("sample.txt", 11), ("input.txt", 1834060)])
def test_run_analysis(fname, expected):
    assert run_analysis("src/app/day01/" + fname) == expected
