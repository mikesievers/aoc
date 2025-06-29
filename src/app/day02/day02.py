import numpy as np


def read_report(fname: str) -> list[list[int]]:
    data = []
    with open(fname, "r") as f:
        for line in f.readlines():
            data.append([int(n) for n in line.split()])
    return data


def is_row_safe(row: list, active_problem_dampener: bool = False) -> bool:
    """
    Determine whether a row of levels is safe.
    It is safe when:
    1. The row of levels is safe
    2. The row without any single level is safe

    Args:
        row: The row to be inspected
        active_problem_dampener: tolerate the levels, if removing one level
            results in a safe row
    """
    if _perform_row_safeness_calculation(row):
        return True

    for drop_idx in range(len(row)):
        row_copy = row.copy()
        row_copy.pop(drop_idx)
        if _perform_row_safeness_calculation(row_copy):
            return True

    return False


def _perform_row_safeness_calculation(row):
    """
    Determine whether a row of levels is safe.
    It is safe when:
    1. Levels are always only decreasing or decreasing
    2. Following levels differ at least by 1, at most by 3

    """
    VALID_DELTAS_PLUS = (1, 2, 3)
    VALID_DELTAS_MINUS = (-1, -2, -3)
    deltas = [row[n + 1] - row[n] for n in range(len(row) - 1)]
    valid_positive_deltas = [delta in VALID_DELTAS_PLUS for delta in deltas]
    valid_negative_deltas = [delta in VALID_DELTAS_MINUS for delta in deltas]
    return all(valid_positive_deltas) or all(valid_negative_deltas)


def count_safe_rows(report: np.array, active_problem_dampener=False) -> int:
    return sum(
        [
            is_row_safe(row, active_problem_dampener=active_problem_dampener)
            for row in report
        ]
    )
