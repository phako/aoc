#!/usr/bin/env python

import sys

# iterative approach
def part_iter(lines):
    up = lines
    down = lines
    position = 0

    while len(up) > 1 or len(down) > 1:
        # Filter oxgen, keep the items that have the most values in a certain position
        if len(up) > 1:
            zeros = [x for x in up if x[position] == '0']
            ones = [x for x in up if x[position] == '1']
            # if equal, keep the ones 
            up = zeros if len(zeros) > len(ones) else ones

        # filter co2, keep the items that have the least values in a certain position
        if len(down) > 1:
            zeros = [x for x in down if x[position] == '0']
            ones= [x for x in down if x[position] == '1']
            # if equal, keep the zeros
            down = zeros if len(zeros) <= len(ones) else ones

        position += 1

    return up, down

# recursive approach, breaks for the big input
def part(lines, position):
    zero = []
    ones = []

    if len(lines) == 1:
        return lines, lines

    for line in lines:
        if line[position] == '0':
            zero.append(line)
        else:
            ones.append(line)

    if len(zero) > len(ones):
        (up, _) = part(zero, position + 1)
        (_, down) = part(ones, position + 1)
    else:
        (up, _) = part(ones, position + 1)
        (_, down) = part(zero, position + 1)

    return up, down

if __name__ == '__main__':
    with open(sys.argv[1]) as f:
        lines = f.readlines()
        count = len(lines)

        a, b = part_iter(lines)
        oxygen = int(a[0].strip(), 2)
        co2 = int(b[0].strip(), 2)

        print(f'Ox: {oxygen}, co2: {co2}, all: {oxygen * co2}')

