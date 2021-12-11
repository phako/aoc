#!/usr/bin/env python

import sys

class Submarine:
    def __init__(self):
        self.depth = 0
        self.position = 0

    def forward(self, value):
        self.position += value

    def down(self, value):
        self.depth += value

    def up(self, value):
        self.down(-value)

    def parse_command(self, command):
        args = command.split(' ', 2)
        move = getattr(self, args[0])

        move(int(args[1]))

    def __str__(self):
        return f'Submarine at p: {self.position}, d: {self.depth}, d*p: {self.depth * self.position}'

class AimedSubmarine(Submarine):
    def __init__(self):
        Submarine.__init__(self)
        self.aim = 0

    def down(self, value):
        self.aim += value

    def forward(self, value):
        Submarine.forward(self, value)
        self.depth += self.aim * value

    def __str__(self):
        return f'AimedSubmarine at p: {self.position}, d: {self.depth}, a: {self.aim}, d*p: {self.depth * self.position}'

if __name__ == '__main__':
    s = Submarine()
    a = AimedSubmarine()
    with open(sys.argv[1]) as f:
        for line in f:
            s.parse_command(line)
            a.parse_command(line)

    print(s)
    print(a)

