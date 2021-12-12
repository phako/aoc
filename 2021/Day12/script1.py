#!/usr/bin/env python3

import re
import sys


def is_lower(cave: str) -> bool:
    return re.match(r'[a-z]+', cave) is not None


def is_upper(cave: str) -> bool:
    return re.match(r'[A-Z]+', cave) is not None


def find_paths_part_2(path, paths, nodes, visited):
    start = path[-1]

    for reachable in nodes[start]:
        if reachable == 'end':
            paths.append(path + ['end'])
        elif reachable == 'start':
            continue
        else:
            if reachable not in path or is_upper(reachable):
                path.append(reachable)
                if reachable in visited.keys():
                    visited[reachable] += 1
                find_paths_part_2(path, paths, nodes, visited)
                path.pop()
                if reachable in visited.keys():
                    visited[reachable] -= 1
            else:
                if 2 not in visited.values():
                    path.append(reachable)
                    if reachable in visited.keys():
                        visited[reachable] += 1
                    find_paths_part_2(path, paths, nodes, visited)
                    path.pop()
                    if reachable in visited.keys():
                        visited[reachable] -= 1


def find_paths(path, paths, nodes):
    start = path[-1]

    for reachable in nodes[start]:
        if reachable == 'end':
            paths.append(path + ['end'])
        else:
            if reachable not in path or is_upper(reachable):
                path.append(reachable)
                find_paths(path, paths, nodes)
                path.pop()


if __name__ == '__main__':
    with open(sys.argv[1]) as f:
        adjacent_hash = {}
        for line in f:
            a, b = line.strip().split('-', 1)
            if a in adjacent_hash:
                adjacent_hash[a].append(b)
            else:
                adjacent_hash[a] = [b]

            if b in adjacent_hash:
                adjacent_hash[b].append(a)
            else:
                adjacent_hash[b] = [a]

    # Find all paths from start to end
    path = ['start']
    paths = []
    for reachable in adjacent_hash['start']:
        find_paths(path + [reachable], paths, adjacent_hash)

    print(len(paths))

    # Find all paths from start to end
    path = ['start']
    paths = []
    for reachable in adjacent_hash['start']:
        visited = dict.fromkeys([x for x in list(adjacent_hash.keys()) if is_lower(x)], 0)
        del visited['start']
        del visited['end']
        if is_lower(reachable):
            visited[reachable] = 1
        find_paths_part_2(path + [reachable], paths, adjacent_hash, visited)

    print(len(paths))
