import pytest

from app.day05.day05 import (
    are_rules_observed,
    make_sequence_correct,
    middle_page_sum,
    read_files,
)


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


@pytest.mark.parametrize(
    "incorrect_seq, correct_seq",
    [
        ([75, 97, 47, 61, 53], [97, 75, 47, 61, 53]),
        ([61, 13, 29], [61, 29, 13]),
        ([97, 13, 75, 29, 47], [97, 75, 47, 29, 13]),
    ],
)
def test_correct_sequence(incorrect_seq, correct_seq):
    rules, _ = read_files("sample.txt")
    assert make_sequence_correct(rules, incorrect_seq) == correct_seq


@pytest.mark.parametrize(
    "fname,expected_sum", [("sample.txt", 123), ("input.txt", 4480)]
)
def test_corrected_middle_page_sum(fname, expected_sum):
    rules, pages = read_files(fname)
    ok_result_mask = are_rules_observed(rules, pages)
    bad_result_mask = [not mask_value for mask_value in ok_result_mask]
    bad_pages = [pages[i] for (i, _) in enumerate(pages) if bad_result_mask[i] is True]

    corrected_pages = [make_sequence_correct(rules, page_seq) for page_seq in bad_pages]
    assert (
        middle_page_sum([True for _ in range(len(corrected_pages))], corrected_pages)
        == expected_sum
    )
