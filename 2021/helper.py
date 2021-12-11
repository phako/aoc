from typing import Iterable, Sequence

def write_pgm(file: str, data: Iterable[Iterable[int]]) -> None:
    with open (file, 'w') as f:
        print()