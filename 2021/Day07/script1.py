#!/usr/bin/env python

import sys
import matplotlib.pyplot as plt

with open(sys.argv[1]) as f:
    positions = list(map(int, f.read().strip().split(',')))
    print(positions)
    left = min(positions)
    right = max(positions)
    histogram = [0 for x in range(right + 1)]

    for crab in positions:
        histogram[crab] += 1

    # find bin which roughly splits the field in half
    x = 0
    crabs_til_x = 0
    fuel = 0
    while crabs_til_x < len(positions) / 2:
        crabs_til_x += histogram[x]
        x += 1

    x -= 1

    for j in range(len(histogram)):
        fuel += abs(x - j) * histogram[j]
        if j == x:
            print(f'fuel at point {fuel}')

    print(f"Fuel for position {x}: {fuel}")


    cost = [0 for x in histogram]
    for x in range(len(cost)):
        for j in range(len(cost)):
            d = abs(x-j)
            # c = d
            c = d * (d + 1) / 2
            cost[x] += c * histogram[j]

    print(min(cost))

    fig, ax = plt.subplots()
    ax.plot([x for x in range(len(histogram))], cost)
    plt.show()
