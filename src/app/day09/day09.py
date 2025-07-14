"""
He shows you the disk map (your puzzle input) he's already generated. For example:

2333133121414131402

The disk map uses a dense format to represent the layout of files and free space on the disk. The digits alternate between indicating the length of a file and the length of free space.

So, a disk map like 12345 would represent a one-block file, two blocks of free space, a three-block file, four blocks of free space, and then a five-block file. A disk map like 90909 would represent three nine-block files in a row (with no free space between them).

Each file on disk also has an ID number based on the order of the files as they appear before they are rearranged, starting with ID 0. So, the disk map 12345 has three files: a one-block file with ID 0, a three-block file with ID 1, and a five-block file with ID 2. Using one character for each block where digits are the file ID and . is free space, the disk map 12345 represents these individual blocks:

0..111....22222

The first example above, 2333133121414131402, represents these individual blocks:

00...111...2...333.44.5555.6666.777.888899
"""

import numpy as np


def read_diskmap(fname: str) -> str:
    with open(f"src/app/day09/{fname}") as f:
        return f.readline().strip()


def parse_diskmap(diskmap: str) -> np.ndarray:
    """
    Parse the diskmap.
    The dismap is a string, alternatingly denoting the length of a file and the length of free blocks.
    """
    disk = np.array([int(0)] * int(diskmap[0]), dtype=np.int16)
    for idx, digit in enumerate(diskmap):
        if idx == 0:  # We have already used the first entry
            continue
        if idx % 2 == 0:  # This is the length of a file
            file_id = int(idx // 2)
            new_entry = np.array([file_id] * int(digit), dtype=np.int16)
        else:
            new_entry = np.array([-1] * int(digit), dtype=np.int16)

        disk = np.concat([disk, new_entry])
        # Every position divisible by 2 is a file length
        # And length of free space otherwise
    return disk


def compact_disk(disk: np.ndarray) -> np.ndarray:
    """
    Move from both the left and the right of the array.
    Position the right index on the first non-empty sector.
    Position the left index on the first empty sector.
    Swap the two.
    Repeat until the left index is >= the right index.
    """
    idx_l = 0
    idx_r = len(disk) - 1

    cd = disk.copy()

    while True:
        while cd[idx_l] != -1:
            idx_l += 1
        while cd[idx_r] == -1:
            idx_r -= 1
        if idx_l >= idx_r:
            break

        cd[idx_l] = cd[idx_r]
        cd[idx_r] = -1

    return cd


def calculate_checksum(cd: np.ndarray) -> int:
    sum = int(0)
    for idx in range(cd.shape[0]):
        block_id = cd[idx]
        if block_id > 0:
            sum += int(block_id) * int(idx)

    return sum
