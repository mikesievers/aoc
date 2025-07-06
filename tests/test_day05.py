import pytest

from app.day05.day05 import are_rules_observed, middle_page_sum, read_files


def test_read_files():
    rules, pages = read_files("sample.txt")

    assert len(rules) == 21
    assert rules[0] == (47, 53)
    assert len(pages) == 6

    assert pages[0] == [75, 47, 61, 53, 29]
    assert pages[-1] == [97, 13, 75, 29, 47]


def test_are_rules_observed():
    # samples rules: pages are to be printed in ascending order
    # but 7 before 3
    rules = [
        (3, 4),
        (5, 7),
        (7, 3),
    ]

    page_samples = [
        [3, 4, 5, 7],  # not OK, 3 before 7
        [5, 7, 3, 4],  # OK
        [5, 7, 3],  # OK
    ]

    results = are_rules_observed(rules, page_samples)

    assert results == [False, True, True]


@pytest.mark.parametrize(
    "fname,expected_sum", [("sample.txt", 143), ("input.txt", 4185)]
)
def test_middle_page_sum(fname, expected_sum):
    rules, pages = read_files(fname)
    ok_result_mask = are_rules_observed(rules, pages)
    assert middle_page_sum(ok_result_mask, pages) == expected_sum
