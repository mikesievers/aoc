from app.day10.day10 import Map


def test_read_map():
    map = Map("sample.txt")
    assert map is not None
    assert map._map[1, 2] == 1
    assert map._map[-1, -2] == 3
    assert map._map[-5, -3] == 8


# def test_find_viable_neighbors():
#     map = read_map("sample.txt")

#     assert find_viable_neighbors()
