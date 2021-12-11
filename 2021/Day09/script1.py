#!/usr/bin/env python

import sys


def get_neighbours():
    pass


with open(sys.argv[1]) as f:
    pgm_data = 'P2\n#heightmap\n'
    hgm = [x.strip() for x in f.readlines()]
    pgm_data += f'{len(hgm)} {len(hgm[0])}\n'
    pgm_data += '10\n'
    empty = ' ' * len(hgm[0])
    width = len(hgm[0])
    height = len(hgm)

    for line in hgm:
        pgm_data += ' '.join(line) + '\n'
    with open(sys.argv[1] + '.pgm', 'w') as g:
        g.write(pgm_data)

    candidates = []

    for y in range(height):
        for x in range(width):
            abort = False
            for ny in range(y-1, y+2):
                if abort:
                    break
                if ny < 0 or ny >= height:
                    continue
                for nx in range(x-1, x + 2):
                    if abort:
                        break
                    if nx < 0 or nx >= width:
                        continue
                    if x == nx and y == ny:
                        continue
                    if hgm[y][x] > hgm[ny][nx]:
                        abort = True
            if not abort:
                candidates.append((x, y))

    # part 1
    ditch_sum = 0
    for (x, y) in candidates:
        ditch_sum += int(hgm[y][x]) + 1

    print(ditch_sum)

    # part 2
    # starting from the low points, crawl around to find the 9s which border the basins
    for (x, y) in candidates:
        