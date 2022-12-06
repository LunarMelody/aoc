from collections import Counter
from util import read_input

DATA_STREAM = read_input(__file__)


def is_unique(s: str):
    counter = Counter(s)
    return all(val == 1 for val in counter.values())


def find_first_marker(unique_length: int = 4):
    first_marker: str | None = None
    first_marker_after = -1

    for idx in range(len(DATA_STREAM)):
        if first_marker is not None:
            break

        sequence = DATA_STREAM[idx : idx + unique_length]

        if len(sequence) < unique_length:
            break

        if is_unique(sequence):
            first_marker = sequence
            first_marker_after = idx + unique_length

    return first_marker_after, first_marker


def part1():
    marker_after, _ = find_first_marker(4)
    print(f"{marker_after}")


def part2():
    marker_after, _ = find_first_marker(14)
    print(f"{marker_after}")
