import pytest

from .solver import parse, part1, part2


@pytest.fixture(scope="module")
def data():
    with open("aoc/three/input.txt") as f:
        yield parse(f.readlines())


def test_part_1(data):
    assert part1(*data) == 553825


def test_part_2(data):
    assert part2(*data) == 93994191
