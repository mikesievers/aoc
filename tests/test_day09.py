import pytest

from app.day09.day09 import (
    calculate_checksum,
    compact_disk,
    defragment_disk,
    parse_diskmap,
    read_diskmap,
)


def test_read_diskmap():
    map = read_diskmap("sample.txt")

    assert map[0] == "2"
    assert map[-2] == "0"


def test_parse_diskmap():
    disk = parse_diskmap(read_diskmap("sample.txt"))

    assert disk[0] == 0
    assert disk[5] == 1
    assert disk[8] == -1
    assert disk[-3] == 8
    assert disk[-2] == 9
    assert disk[-1] == 9


def test_compact_disk():
    disk = parse_diskmap(read_diskmap("sample.txt"))
    cd = compact_disk(disk)

    assert cd[1] == 0
    assert cd[2] == 9
    assert cd[4] == 8
    assert cd[4] == 8


@pytest.mark.parametrize(
    "fname,expected_sum", [("sample.txt", 1928), ("input.txt", 6415184586041)]
)
def test_calculate_checksum(fname, expected_sum):
    disk = parse_diskmap(read_diskmap(fname))
    cd = compact_disk(disk)
    assert calculate_checksum(cd) == expected_sum


# Test part 2
# Defragment by moving files (highest ID first) to the left most possible
# continuous area of free space


def test_defragment_disk():
    disk = parse_diskmap(read_diskmap("sample.txt"))
    df = defragment_disk(disk)

    assert df[1] == 0
    assert df[2] == 9
    assert df[4] == 2
    assert df[5] == 1
    assert df[8] == 7
    assert df[11] == -1
    assert df[12] == 4


@pytest.mark.parametrize(
    "fname,expected_sum", [("sample.txt", 2858), ("input.txt", 6436819084274)]
)
def test_calculate_checksum_defrag(fname, expected_sum):
    disk = parse_diskmap(read_diskmap(fname))
    df = defragment_disk(disk)
    assert calculate_checksum(df) == expected_sum
