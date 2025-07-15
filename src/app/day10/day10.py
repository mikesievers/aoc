"""
goal: find trails
  - start at trailhead ('0')
  - via vertical and horizontal neighbors that are larger by 1
  - to trail end ('9')
  - then score the trail heads (nr of 9s reachable from the 0) and add the score


Thoughts:
- The map spans a directed graph
- Nodes are connected by +1 edges
- Multiple paths from one trailhead can lead to the same end
-

Procedure:
- Inspect each cell for being a trailhead
- find all viable neighbors and follow them recursively until a '9'
- add the position of each 9 to a set related to the trailhead

"""

import numpy as np


class Map:
    _map: np.ndarray

    def __init__(self, fname):
        with open(f"src/app/day10/{fname}") as f:
            lines = [list(line.strip()) for line in f.readlines() if len(line) > 2]
        self._map = np.array(lines, dtype=np.int8)
