#!/usr/bin/env python

import sys


def get_digits_with_length(data, length):
    return list(filter(lambda x: len(x) == length, inputs[line] + outputs[line]))


with open(sys.argv[1]) as f:
    inputs = []
    outputs = []
    for line in f:
        inp, outp = line.strip().split(' | ')
        inputs.append(inp.split(' '))
        outputs.append(outp.split(' '))

    # Part 1
    all_outputs = sum(outputs, [])
    unique = list(filter(lambda x: len(x) in (2, 4, 3, 7), all_outputs))
    print(len(unique))

    # Part 2

    # Step 1: Find the top segment
    # This is done by subtracting 1 from 7
    sum = 0

    for line in range(len(inputs)):
        digits = list(map(sorted, inputs[line] + outputs[line]))
        one = set(get_digits_with_length(digits, 2)[0])
        seven = set(get_digits_with_length(digits, 3)[0])
        four = set(get_digits_with_length(digits, 4)[0])
        eight = set(get_digits_with_length(digits, 7)[0])

        top = seven - one

        # Step 2:
        # Figure out the nine.
        # for that,
        #  - add the top to 4
        #  - find all candidates with length 6
        #  - those that differ from the merged character by 1 must be 9, as A is not included
        matcher = four | top

        candidates = list(map(set, get_digits_with_length(digits, 6)))
        nine = list(filter(lambda x: len(x - matcher) == 1, candidates))[0]
        bottom = nine - matcher

        # bottom left is easy, that's 8 - 9
        bottom_left = eight - nine

        # Filter out from the candidates of length six those, that only one bit in common with one
        # Those are the sixes
        six = list(filter(lambda x: len(x & one) == 1, candidates))[0]

        # We know top right now, that is the only difference between 8 and 6
        top_right = eight - six

        # With that we know bottom right, since that is what is left in the 1
        bottom_right = one - top_right

        # and five is a six with bottom_left removed
        five = six - bottom_left

        candidates = list(map(set, get_digits_with_length(digits, 5)))

        # candidates is either 2, 3 or 5
        # the one with which has two differences from 5 is a two
        two = list(filter(lambda x: len(five - x) == 2, candidates))[0]

        # three has only one difference from five
        three = list(filter(lambda x: len(five - x) == 1, candidates))[0]

        # top left is what you get if you remove all of three from 5
        top_left = five - three

        # and zero is all but middle
        zero = top | bottom | top_right | top_left | bottom_right | bottom_left

        # and just for completeness, middle is 8 - 0
        middle = eight - zero

        decoder = {
            ''.join(sorted(zero)): '0',
            ''.join(sorted(one)): '1',
            ''.join(sorted(two)): '2',
            ''.join(sorted(three)): '3',
            ''.join(sorted(four)): '4',
            ''.join(sorted(five)): '5',
            ''.join(sorted(six)): '6',
            ''.join(sorted(seven)): '7',
            ''.join(sorted(eight)): '8',
            ''.join(sorted(nine)): '9'}

        sum += int(''.join([decoder[''.join(sorted(x))] for x in outputs[line]]))
    print(sum)
