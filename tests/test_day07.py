import pytest

from app.day07.day07 import Operators, add_valid_test_values, is_composable, read_data


@pytest.fixture
def simple_operators():
    return [Operators.times, Operators.plus]


@pytest.fixture
def complex_operators():
    return [Operators.times, Operators.plus, Operators.concat]


def test_read_data():
    data = read_data("sample.txt")
    assert data[190] == [10, 19]
    assert data[292] == [11, 6, 16, 20]


def test_is_composable(simple_operators):
    data = read_data("sample.txt")
    assert is_composable(3267, data[3267], simple_operators)
    assert not is_composable(161011, data[161011], simple_operators)


@pytest.mark.parametrize(
    "fname, expected_sum", [("sample.txt", 3749), ("input.txt", 12940396350192)]
)
def test_add_valid_test_values(fname, expected_sum, simple_operators):
    data = read_data(fname)
    assert add_valid_test_values(data, simple_operators) == expected_sum


def test_is_composable_concat(complex_operators):
    data = read_data("sample.txt")
    assert is_composable(156, data[156], complex_operators)
    assert is_composable(7290, data[7290], complex_operators)


# Part 2
@pytest.mark.parametrize(
    "fname, expected_sum", [("sample.txt", 11387), ("input.txt", 106016735664498)]
)
def test_add_valid_test_values_complex(fname, expected_sum, complex_operators):
    data = read_data(fname)
    assert add_valid_test_values(data, complex_operators) == expected_sum
