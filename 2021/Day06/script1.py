#!/usr/bin/env python

import sys

if __name__ == "__main__":
    # sort the fishes into equivalency classes mod 7
    with open(sys.argv[1]) as f:
        numbers = map(int, f.read().strip().split(','))
        classes = [0 for x in range(7)]
        for number in numbers:
            classes[number] += 1

        newborns = [0 for x in range(9)]
        for day in range(1, 256):
            animal_class = day % 7

            now_adults = newborns.pop(0)
            classes[animal_class] += now_adults

            animals_in_class = classes[animal_class]
            newborns.append(animals_in_class)

        print(f'Day {day} {sum(classes) + sum(newborns)}')
        sys.stdin.readline()