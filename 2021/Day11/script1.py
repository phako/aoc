#!/usr/bin/env python3

import helper
import sys

from typing import Tuple


def increase(data: list[list[int]]) -> None:
    for line in data:
        for x in range(len(line)):
            line[x] += 1


def find_neighbours(location: Tuple[int, int], width: int, height: int) -> list[Tuple[int, int]]:
    result = []
    for y in range(location[1] - 1, location[1] + 1 + 1):
        if y < 0 or y >= height:
            continue
        for x in range(location[0] - 1, location[0] + 1 + 1):
            if x < 0 or x >= width:
                continue
            if (x, y) == location:
                continue
            result.append((x, y))

    return result


# this will dump the steps to PGM files, which can be viewed or converted into a video with GStreamer:
# gst-launch-1.0 multifilesrc location="step%05d.pgm" ! typefind ! avdec_pgm ! videoconvert ! \
# videoscale sharpen=1 method=nearest-neighbour ! video/x-raw,width=800,height=800 ! \
# theoraenc ! oggmux ! filesink location=octopus.ogv
if __name__ == '__main__':
    data, width, height = helper.read_int_matrix(sys.argv[1])
    print(f'Data dimension: {width} x {height}')

    flashed = 0
    visited = []
    step = 0
    while len(visited) < width * height:
        # part 1

        # Increase energy levels by one
        increase(data)

        # Find all coordinates with initial energy level larger than 9
        flashing = helper.find_in_data(data, lambda x: x > 9)

        visited = []
        while len(flashing) > 0:
            current = flashing.pop(0)
            visited.append(current)
            to_visit = find_neighbours(current, width, height)
            for x, y in to_visit:
                data[y][x] += 1
                if data[y][x] > 9:
                    if (x, y) not in visited and (x, y) not in flashing:
                        flashing.append((x, y))

        # Reset all that flashed
        for x, y in visited:
            data[y][x] = 0

        flashed += len(visited)

        helper.write_pgm(f'step{step:05d}.pgm', data)

        if step == 99:
            print(f'Flashed after {step + 1} steps: {flashed}')
        step += 1

    print(f'All flashing at the same time after step {step}')
