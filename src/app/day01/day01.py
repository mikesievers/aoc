import numpy as np


# Part 1
def read_input(fname: str) -> np.array:
    # raise Exception(f"{os.listdir('src/app/day01')}")
    with open(fname, "r") as f:
        data = []
        for line in f:
            data.append(list(map(int, line.split())))

    return np.array(data)


def sort_lists(lists: np.array) -> np.array:
    return np.column_stack((sorted(lists[:, 0]), sorted(lists[:, 1])))


def add_distances(lists: np.array) -> int:
    return np.sum(np.abs(lists[:, 0] - lists[:, 1]))


def run_analysis(fname: str) -> int:
    return add_distances(sort_lists(read_input(fname)))


# Part 2
def similarity_index(lists: np.array) -> int:
    """Similarity index.
    1. For each of number in the left column, create a slot in a dict with value 0
    2. For each number in the right column, increment the dict by the number, if a slot exists
    3. For each row in the left column, add the dict value for that number
    """
    left_column = lists[:, 0]
    right_column = lists[:, 1]
    # Establish the keys
    # val_dict = {f"{val}": 0 for val in left_column}
    val_dict = {val: 0 for val in left_column}

    for val in list(right_column):
        if val in val_dict:
            val_dict[val] += val

    return sum([val_dict[val] for val in left_column])
