import pytest

from .solver import part1, part2


@pytest.fixture(scope="module")
def data():
    with open("aoc/eleven/input.txt") as f:
        yield f.readlines()


def test_part_1(data):
    assert part1(data) == 9608724


def test_part_2(data):
    assert part2(data) == 904633799472