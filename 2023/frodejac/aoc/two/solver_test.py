import pytest

from .solver import p1, p2


@pytest.fixture(scope="module")
def data():
    with open("aoc/two/input.txt") as f:
        yield f.readlines()


def test_part_1(data):
    assert p1(data) == 2367


def test_part_2(data):
    assert p2(data) == 77021
