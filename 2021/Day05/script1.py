#!/usr/bin/env python

import sys
import re


class Matrix:
    def __init__(self, skip = True):
        self.raster = None
        self.dim_x = 0
        self.dim_y = 0
        self.lines = []
        self.skip = skip
        self.overlap = 0

    def add_line(self, l):
        x1, y1, x2, y2 = map(int, re.match(r'(\d+),(\d+) -> (\d+),(\d+)', l.strip()).groups())
        # Skip all lines not horizontal or vertical
        if x1 != x2 and y1 != y2:
            if self.skip:
                return
        else:
            # swap lines
            if x2 < x1:
                x1, x2 = x2, x1

            if y2 < y1:
                y1, y2 = y2, y1

        # Get grid dimensions
        self.dim_x = max((x1, x2, self.dim_x))
        self.dim_y = max((y1, y2, self.dim_y))

        self.lines.append(((x1, y1), (x2, y2)))

    def fill(self):
        self.raster = [[0 for x in range(self.dim_x + 1)] for y in range(self.dim_y + 1)]
        for line in self.lines:
            if line[0][0] == line[1][0] or line[0][1] == line[1][1]:
                for y in range(line[0][1], line[1][1] + 1):
                    for x in range(line[0][0], line[1][0] + 1):
                        self.raster[y][x] += 1
                        if self.raster[y][x] == 2:
                            self.overlap += 1
            else:
                y = line[0][1]
                j = y
                x = line[0][0]
                k = x
                y2 = line[1][1]
                x2 = line[1][0]
                dx = int((x2 - x) / abs(x2 - x))
                dy = int((y2 - y) / abs(y2 - y))

                while j != line[1][1] + dy:
                    self.raster[j][k] += 1
                    if self.raster[j][k] == 2:
                        self.overlap += 1
                    k += dx
                    j += dy


    def __str__(self):
        str = ''
        for row in self.raster:
            for value in row:
                str += f'{"." if value == 0 else value}\t'
            str += '\n'

        return str


if __name__ == '__main__':
    with open(sys.argv[1]) as f:
        m = Matrix()
        n = Matrix(skip = False)

        for line in f:
            m.add_line(line)
            n.add_line(line)

        m.fill()
        n.fill()
        print(m.overlap, n.overlap)
        #print(n)
