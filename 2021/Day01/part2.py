#!/usr/bin/env python

import sys

if __name__ == '__main__':
    last_number = None
    increased = 0
    window = []
    with open(sys.argv[1]) as f:
        for line in f:
            number = int(line)
            window.append(number)
            if len(window) < 3:
                continue

            number = window[0] + window[1] + window[2]
            window.pop(0)

            if last_number is None or last_number >= number:
                pass
            else:
                increased = increased + 1

            last_number = number

    print(f'Increased measurements: {increased}')
