import numpy as np
import pytest

from app.day02.day02 import count_safe_rows, is_row_safe, read_report

SAMPLE_FILE = "src/app/day02/sample.txt"
REPORT_FILE = "src/app/day02/input.txt"


def test_d02p1_read_report():
    first_line = [7, 6, 4, 2, 1]
    report = read_report(SAMPLE_FILE)
    assert len(report) == 6
    assert report[0] == first_line


@pytest.mark.parametrize(
    "row,expected", [([1, 2, 3], True), ([1, 4, 5, 7], True), ([1, 5, 6, 7], False)]
)
def test_s02p1_is_row_safe(row, expected):
    assert is_row_safe(row) == expected


@pytest.mark.parametrize("report_file,expected", [(SAMPLE_FILE, 2), (REPORT_FILE, 213)])
def test_count_safe_rows(report_file, expected):
    report = read_report(report_file)
    assert count_safe_rows(report) == expected
