from src.day01.runner import Day01
from src.day02.runner import Day02
from src.day03.runner import Day03
from src.day04.runner import Day04
from src.day05.runner import Day05
from src.day06.runner import Day06
from src.day07.runner import Day07
from src.day08.runner import Day08
from src.day09.runner import Day09
from src.day10.runner import Day10
from src.day11.runner import Day11
from src.day12.runner import Day12


# ---------------------------------------------------------
# Daily Tests
# ---------------------------------------------------------

def test_day01():
    assert Day01().puzzle1() == 982
    assert Day01().puzzle2() == 6106

def test_day02():                                    # annoyingly slow
    assert Day02().puzzle1() == 19574776074
    assert Day02().puzzle2() == 25912654282

def test_day03():
    assert Day03().puzzle1() == 17403
    assert Day03().puzzle2() == 173416889848394

def test_day04():
    assert Day04().puzzle1() == 1393
    assert Day04().puzzle2() == 8643

def test_day05():
    assert Day05().puzzle1() == 674
    assert Day05().puzzle2() == 352509891817881

def test_day06():
    assert Day06().puzzle1() == -1
    assert Day06().puzzle2() == -2

def test_day07():
    assert Day07().puzzle1() == -1
    assert Day07().puzzle2() == -2

def test_day08():
    assert Day08().puzzle1() == -1
    assert Day08().puzzle2() == -2

def test_day09():
    assert Day09().puzzle1() == -1
    assert Day09().puzzle2() == -2

def test_day10():
    assert Day10().puzzle1() == -1
    assert Day10().puzzle2() == -2

def test_day11():
    assert Day11().puzzle1() == -1
    assert Day11().puzzle2() == -2

def test_day12():
    assert Day12().puzzle1() == -1
    assert Day12().puzzle2() == -2