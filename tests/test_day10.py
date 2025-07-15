from app.day10.day10 import Map


def test_read_map():
    map = Map("sample.txt")
    assert map is not None
    assert map._map[1, 2] == 1
    assert map._map[-1, -2] == 3
    assert map._map[-5, -3] == 8


def test_follow_trail():
    map = Map("sample.txt")

    map.follow_trail(y=1, x=1)

    expected = set()
    expected.add((0, 1))
    assert map._trails[(1, 1)] == expected

    map.follow_trail(y=0, x=1)
    assert map._trails[(0, 1)] == expected

    map.follow_trail(y=0, x=4)
    assert len(map._trails[(0, 4)]) == 6
