from typing import Callable, Sequence, Tuple


def write_pgm(file: str, data: Sequence[Sequence[int]], invert : bool = False) -> None:
    pgm_data = 'P2\n#heightmap\n'
    pgm_data += f'{len(data)} {len(data[0])}\n'
    max_value = max([max(x) for x in data])
    pgm_data += f'{max_value}\n'

    for line in data:
        if invert:
            line = [max - x for x in line]

        pgm_data += ' '.join(map(str, line)) + '\n'

    with open(file, 'w') as g:
        g.write(pgm_data)


def read_int_matrix(file: str) -> list[list[int]]:
    with open(file) as f:
        data = [list(map(int, x.strip())) for x in f.readlines()]

        return data, len(data), len(data[0])


def find_in_data(data: Sequence[Sequence[int]], condition : Callable[[int], bool]) -> list[Tuple[int,int]]:
    result = []
    for y in range(len(data)):
        for x in range(len(data[0])):
            if condition(data[y][x]):
                result.append((x, y))

    return result