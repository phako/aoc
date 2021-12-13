#!/usr/bin/env python3

import re
import sys


class State:
    COORDINATES = 0
    FOLDING_INSTRUCTIONS = 1

    def __init__(self):
        self.state = State.COORDINATES

    def next(self):
        self.state = State.FOLDING_INSTRUCTIONS

    def __eq__(self, other):
        if isinstance(other, int):
            return self.state == other
        elif isinstance(other, State):
            return self.state == other.state
        else:
            raise TypeError()


def print_field(field, width, height):
    for y in range(height):
        for x in range(width):
            print('#' if (x,y) in field else '.', end='')
        print()


if __name__ == "__main__":
    with open(sys.argv[1]) as f:
        state = State()
        coordinates = []
        folding = []
        width = 0
        height = 0

        for line in f:
            if len(line.strip()) == 0:
                state.next()
                continue

            if state == State.COORDINATES:
                x, y = map(int, line.strip().split(',', 2))
                width = max(x, width)
                height = max(y, height)
                coordinates.append((x, y))
            else:
                axis, fold = line.strip().split('=', 2)
                folding.append((axis[-1], int(fold)))

        width += 1
        height += 1

        print(f"Field is {width} x {height}")

        # do the first fold
        fold = folding.pop(0)
        folded = None

        if fold[0] == 'x':
            folded = [(2 * fold[1] - x, y) if x > fold[1] else (x,y) for x, y in coordinates]
            width -= fold[1]
        else:
            folded = [(x, 2 * fold[1] - y) if y > fold[1] else (x,y) for x, y in coordinates]
            height -= fold[1]

        # print_field(folded, width, height)

        print(len(set(folded)))

        # fold everything
        while folding:
            # do the first fold
            fold = folding.pop(0)

            if fold[0] == 'x':
                folded = [(2 * fold[1] - x, y) if x > fold[1] else (x, y) for x, y in folded]
                width -= fold[1]
            else:
                folded = [(x, 2 * fold[1] - y) if y > fold[1] else (x, y) for x, y in folded]
                height -= fold[1]

        print(f"Field is now {width} x {height}")
        print_field(folded, width, height)
