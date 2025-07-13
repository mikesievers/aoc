from app.day08.day08 import Map


def test_map_init():
    map = Map("sample.txt")

    assert map._data.shape == (12, 12)
    assert map._antennas["A"] == [(5, 6), (8, 8), (9, 9)]
    assert (1, 8) in map._antennas["0"]


def test_find_antinodes():
    map = Map("sample.txt")
    assert (2, 0) in map._antinodes
    assert (7, 7) in map._antinodes
