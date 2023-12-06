import pytest

from .solver import part1, part2


@pytest.fixture(scope="module")
def data():
    with open("aoc/six/input.txt") as f:
        yield f.readlines()


def test_part_1(data):
    assert part1(data) == 128700


def test_part_2(data):
    assert part2(data) == 39594072
