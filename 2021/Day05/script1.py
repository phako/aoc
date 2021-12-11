#!/usr/bin/env python

import sys
import re

with open(sys.argv[1]) as f:
    dim_x = 0
    dim_y = 0
    lines = []
    for l in f:
        x1, y1, x2, y2 = map(int, re.match(r'(\d+),(\d+) -> (\d+),(\d+)', l.strip()).groups())
        # Skip all lines not horizontal or vertical
        if x1 != x2 and y1 != y2:
            continue

        # Get grid dimensions
        dim_x = max((x1, x2, dim_x))
        dim_y = max((y1, y2, dim_y))

        lines.append(((x1, y1), (x2, y2)))

    raster = [[0 for x in range(dim_x + 1)] for y in range(dim_y + 1)]
    print(lines)
    print(raster)

    overlap = 0

    for line in lines:
        print(line)
        for y in range(line[0][1], line[1][1] + 1):
            for x in range(line[0][0], line[1][0] + 1):
                print(x,y)
                raster[y][x] += 1
                if raster[y][x] == 2:
                    overlap += 1

    print(f'overlap: {overlap}')
