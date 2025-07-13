from itertools import product
from typing import Callable

# class Operators(Enum):
#     PLUS = lambda x, y: x + y
#     TIMES = lambda x, y: x * y


class Operators:
    @staticmethod
    def plus(x, y):
        return x + y

    @staticmethod
    def times(x, y):
        return x * y

    @staticmethod
    def concat(x, y):
        return int(str(x) + str(y))


def read_data(fname: str) -> dict[int, list[int]]:
    """
    Read and split input data in the format:

    190: 10 19
    3267: 81 40 27
    """
    with open(f"src/app/day07/{fname}") as infile:
        data = dict()
        lines = infile.readlines()
        for line in lines:
            if len(line) < 2:
                continue
            (key, values) = line.strip().split(":")
            data[int(key)] = [int(val) for val in values.split()]
    return data


def is_composable(
    test_value: int, numberlist: list[int], operators: list[Callable]
) -> bool:
    """Between every two numbers, an operator from the list should be inserted.
    Permutate all operators.
    """
    for ops in product(operators, repeat=len(numberlist) - 1):
        result = numberlist[0]
        for pos in range(len(ops)):
            result = ops[pos](result, numberlist[pos + 1])

        if result == test_value:
            # A valid combination has been found
            return True

    # No combination has been found
    return False


def add_valid_test_values(data: dict[int, list[int]], operators: list[Callable]) -> int:
    sum = 0
    for key, values in data.items():
        if is_composable(key, values, operators):
            sum += key

    return sum
