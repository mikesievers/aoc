import pytest

from app.day06.day06 import Board, Fields, Orientation, Player


@pytest.fixture
def sample_board():
    return Board.from_file("sample.txt")


@pytest.fixture
def real_board():
    return Board.from_file("input.txt")


def test_init_board(sample_board):
    assert sample_board.shape() == (10, 10)

    assert sample_board._cells[0, 4] == Fields.OBSTRUCTION
    assert sample_board._cells[-1, -4] == Fields.OBSTRUCTION
    # Ensure the player position is now marked with a period:
    assert sample_board._cells[6, 4] == Fields.EMPTY
    # Ensure the player position is remembered
    assert sample_board._player.get_pos() == (6, 4)
    # ... as well as the orientation
    assert sample_board._player.get_orientation() == Orientation.UP
    # Ensure the first sqare is marked as "trodden"
    assert sample_board._walked_cells[6, 4] == Fields.WALKED


def test_make_move(sample_board):
    sample_board.make_move()
    assert sample_board._player.get_pos() == (5, 4)

    assert (6, 4, Orientation.UP) in sample_board._past_moves
    assert (5, 4, Orientation.UP) in sample_board._past_moves

    sample_board._player._orientation = Orientation.RIGHT
    sample_board.make_move()
    assert sample_board._player.get_pos() == (5, 5)

    assert (5, 4, Orientation.UP) in sample_board._past_moves
    assert (5, 5, Orientation.RIGHT) in sample_board._past_moves

    sample_board._player._orientation = Orientation.DOWN
    sample_board.make_move()
    assert sample_board._player.get_pos() == (6, 5)

    sample_board._player._orientation = Orientation.LEFT
    sample_board.make_move()
    assert sample_board._player.get_pos() == (6, 4)


def test_look_ahead(sample_board):
    next_char = sample_board.look_ahead()
    assert next_char == Fields.EMPTY

    for _ in range(5):
        sample_board.make_move()
    assert sample_board.look_ahead() == Fields.OBSTRUCTION


def test_turn_right():
    p1 = Player((1, 1), Orientation.UP)

    p1.turn_right()
    assert p1.get_orientation() == Orientation.RIGHT
    p1.turn_right()
    assert p1.get_orientation() == Orientation.DOWN
    p1.turn_right()
    assert p1.get_orientation() == Orientation.LEFT
    p1.turn_right()
    assert p1.get_orientation() == Orientation.UP


def test_count_walked(sample_board):
    while not sample_board._is_over:
        sample_board.make_move()

    assert sample_board.count_walked() == 41


def test_part_1(real_board):
    while not real_board._is_over:
        real_board.make_move()

    assert real_board.count_walked() == 5409


def test_set_obstacle(sample_board):
    sample_board.set_obstacle(-2, 1)
    while not sample_board._is_over:
        sample_board.make_move()
    assert sample_board.is_loop


@pytest.mark.parametrize(
    "fname,expected_nr_possibilities", [("sample.txt", 6), ("input.txt", 2022)]
)
def test_possibilities(fname, expected_nr_possibilities):
    nr_possibilities = 0
    board = Board.from_file(fname)

    (y_max, x_max) = board.shape()

    # for y in [8]:  # range(y_max):
    #    for x in [1]:  # range(x_max):
    for y in range(y_max):
        for x in range(x_max):
            board = Board.from_file(fname)
            if (y, x) != board._player.get_pos():
                board.set_obstacle(y, x)
            while not board._is_over:
                board.make_move()
            if board._is_loop:
                nr_possibilities += 1
                print(f"{(y, x) =}")

    assert nr_possibilities == expected_nr_possibilities
