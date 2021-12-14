#!/usr/bin/env python3

import sys


def expand(rules, input, histogram):
    result = ''
    while len(input) > 1:
        pair = input[0:2]
        input = input[1:]
        if pair in rules:
            result += pair[0] + rules[pair]
            histogram[rules[pair]] += 1
        else:
            result += pair[0]
        histogram[pair[0]] += 1

    histogram[input] += 1
    return result + input


if __name__ == "__main__":
    with open(sys.argv[1]) as f:
        input = f.readline().strip()
        a = input[:]
        f.readline()

        alphabet = set(input)

        rules = {}
        for line in f:
            l, r = line.strip().split(' -> ')
            alphabet |= set(l) | set(r)
            rules[l] = r

        # part 1, naive solution, expand the string and count everything
        histogram = {}
        for i in range(10):
            histogram = dict.fromkeys(alphabet, 0)
            input = expand(rules, input, histogram)

        print(histogram[max(histogram, key=histogram.get)] - histogram[min(histogram, key=histogram.get)])

        # part 2. Naive does not scale here. Idea is to count the letters added in each step
        # The basic idea is to count how many new letters each step will add
        # To do that:
        #  - Generate the pairs for the input
        #  - Modify the rules to directly generate both pairs
        #
        # For each step:
        #   - go over the current list of pairs and their occurence and add the newly generated character
        #     count (number of old pairs)  to the overall statistics
        #   - Using the new rules and the list of occurences for the current step, generate the list of pairs for
        #     the next step

        new_rules = {}
        current_step_pair_count = {}
        for l, r in rules.items():
            # Fill the
            current_step_pair_count[l] = 0
            current_step_pair_count[l[0]+r] = 0
            current_step_pair_count[r+l[1]] = 0
            new_rules[l] = [f'{l[0]}{r}', f'{r}{l[1]}']

        # Take a copy of this for later
        base = dict(current_step_pair_count)

        # Fill current step count from the initial input
        pairs = [f'{a[x]}{a[x+1]}' for x in range(len(a) - 1)]
        for k in pairs:
            current_step_pair_count[k] += 1

        # the overall produced letter count
        # Add the count of each letter in the initial word, as the grammar is only producing new characters
        letters_added = dict.fromkeys(alphabet, 0)
        for c in a:
            letters_added[c] += 1

        # Run all the steps as described above
        for x in range(40):
            tmp = dict(base)
            for p in current_step_pair_count:
                # Add the number of added letters to the overall statistics according to production rule
                letters_added[rules[p]] += current_step_pair_count[p]

                # generate the new pair count
                a, b = new_rules[p]
                tmp[a] += current_step_pair_count[p]
                tmp[b] += current_step_pair_count[p]

            current_step_pair_count = dict(tmp)

        print(max(letters_added.values()) - min(letters_added.values()))