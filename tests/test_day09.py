from app.day09.day09 import compact_disk, parse_diskmap, read_diskmap


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


# NEXT: test checksum
