#!/usr/bin/env python3

import sys
import re
import colorama

class BingoBoard:
    def __init__(self, board_size=5, ):
        self.board_size = board_size
        self.__score = 0
        self.__current_row = 0

        # Generate a board which has a pair of Number, drawn
        self.board = [list() for y in range(self.board_size)]

    def append_line(self, line):
        numbers_in_line = re.split(r'\W+', line.strip())
        for number in numbers_in_line:
            self.board[self.__current_row].append([number, False])

        self.__current_row += 1

    def score(self):
        self.__score = 0
        for row in self.board:
            self.__score += sum([int(y[0]) for y in row if not y[1]])

        return self.__score

    def mark(self, number):
        for row in self.board:
            for column in row:
                if column[0] == number:
                    column[1] = True

        # check rows
        for row in self.board:
            if sum([1 if x[1] else 0 for x in row ]) == self.board_size:
                return True

        for x in range(self.board_size):
            col = [row[x] for row in self.board]

            if sum([1 if x[1] else 0 for x in col]) == self.board_size:
                return True

    def __str__(self):
        s = ''
        for row in self.board:
            for cell in row:
                s += f'{colorama.Fore.RED if cell[1] else colorama.Style.NORMAL}{cell[0]}{colorama.Style.RESET_ALL}\t'
            s += '\n'

        return s


def mark_until_bingo(boards, numbers):
    for number in numbers:
        for board in boards:
            if board.mark(number):
                return board, int(number)

    return None, None


if __name__ == '__main__':
    with open(sys.argv[1]) as f:
        # read drawn numbers
        numbers = f.readline().strip().split(',')

        # swallow next line
        f.readline()
        boards = [BingoBoard()]
        for line in f:
            if line.strip() == '':
                boards.append(BingoBoard())
            else:
                boards[-1].append_line(line)

        winning_boards = list()
        winning_board, winning_number = mark_until_bingo(boards, numbers)
        print('Score of first winning board: ', winning_board.score() * winning_number)
        print(winning_board)

        while winning_board:
            boards.remove(winning_board)
            winning_boards.append(winning_board)
            i = numbers.index(str(winning_number))
            numbers = numbers[i:]
            winning_board, winning_number = mark_until_bingo(boards, numbers)
            print(winning_board)

        print('Score of last: ', winning_boards[-1].score() * int(numbers[0]))
        print(winning_boards[-1])
