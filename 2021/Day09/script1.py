#!/usr/bin/env python

import helper

import sys
import functools


def find_direct_neighbours(center, width, height):
    x, y = center
    all = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
    return [v for v in all if v[0] in range(0, width) and v[1] in range(0, height)]


if __name__ == '__main__':
    hgm = []
    with open(sys.argv[1]) as f:
        hgm = [list(map(int, x.strip())) for x in f.readlines()]
        helper.write_pgm(sys.argv[1] + '.pgm', hgm)

        empty = ' ' * len(hgm[0])
        width = len(hgm[0])
        height = len(hgm)

        candidates = []

        for y in range(height):
            for x in range(width):
                if len([v for v in find_direct_neighbours((x,y), width, height) if hgm[v[1]][v[0]] < hgm[y][x]]) == 0:
                    candidates.append((x, y))

        # part 1
        ditch_sum = 0
        for (x, y) in candidates:
            ditch_sum += int(hgm[y][x]) + 1

        print(ditch_sum)

        # part 2
        # starting from the low points, crawl around to find the 9s which border the basins
        sizes = []
        for center in candidates:
            neighbours = [center]
            # [v for v in  find_direct_neighbours(center, width, height) if int(hgm[v[1][0]] <9)]
            basin = []
            while len(neighbours) > 0:
                basin.append(neighbours.pop(0))
                neighbours += [v for v in find_direct_neighbours(basin[-1], width, height)
                               if int(hgm[v[1]][v[0]]) < 9 and v not in basin and v not in neighbours]

            print(f'basin size of ditch {center} is {len(basin)}')
            sizes.append(len(basin))

        print(f'Result: {functools.reduce(lambda a, b: a * b, sorted(sizes, reverse=True)[:3], 1)}')

