import numpy as np
import pytest

from app.day01.day01 import read_input, similarity_index


def test_similarity_index():
    left = [3, 4, 2, 1, 3, 3]
    right = [4, 3, 5, 3, 9, 3]
    lists = np.column_stack((left, right))

    assert similarity_index(lists) == 31


@pytest.mark.parametrize(
    "fname,expected", [("sample.txt", 31), ("input.txt", 21607792)]
)
def test_day01_part2(fname, expected):
    lists = read_input(f"src/app/day01/{fname}")
    assert similarity_index(lists) == expected
