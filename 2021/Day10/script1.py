#!/usr/bin/env python

import sys


if __name__ == '__main__':
    syntax_error = []
    incomplete = []
    syntax_score = 0

    expected = {
        "(": ")",
        "[": "]",
        "{": "}",
        "<": ">"
    }

    with open(sys.argv[1]) as f:
        scores = {
            ")": 3,
            "]": 57,
            "}": 1197,
            ">": 25137
        }

        score = 0

        # part1: Filter out incomplete / syntax error
        for line in f:
            stack = []
            for c in line.strip():
                if c in '([{<':
                    stack.append(c)
                else:
                    last = stack.pop()
                    if c != expected[last]:
                        syntax_error.append(line.strip())
                        syntax_score += scores[c]
                        break

            if len(syntax_error) == 0 or syntax_error [-1] != line.strip():
                incomplete.append(stack)

    print(f'{len(incomplete)} incomplete lines, {len(syntax_error)} lines with syntax error, Score: {syntax_score}')

    completion_scores = {
        ")": 1,
        "]": 2,
        "}": 3,
        ">": 4
    }

    scores = []
    for line in incomplete:
        line_score = 0
        print(f'Calculating score for {"".join(line)}')
        missing = ""
        for c in reversed(line):
            line_score *= 5
            line_score += completion_scores[expected[c]]
            missing += expected[c]

        scores.append(line_score)
        print(f'Missing was {missing}, score: {line_score}')

    scores = sorted(scores)
    # The puzzle guarantees an odd number. Since we would have
    # add 1 to the integer division, but start at 0, we can just
    # use the division as is
    middle = len(scores) // 2
    print(f'Middle completion score: {scores[middle]}')
