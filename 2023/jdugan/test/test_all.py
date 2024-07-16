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
from src.day13.runner import Day13
from src.day14.runner import Day14
from src.day15.runner import Day15
from src.day16.runner import Day16
from src.day17.runner import Day17
from src.day18.runner import Day18
from src.day19.runner import Day19
from src.day20.runner import Day20
from src.day21.runner import Day21
from src.day22.runner import Day22
from src.day23.runner import Day23
from src.day24.runner import Day24
from src.day25.runner import Day25


# ---------------------------------------------------------
# Daily Tests
# ---------------------------------------------------------

def test_day01():
    assert Day01().puzzle1() == 54601
    assert Day01().puzzle2() == 54078

def test_day02():
    assert Day02().puzzle1() == 3035
    assert Day02().puzzle2() == 66027

def test_day03():
    assert Day03().puzzle1() == 551094
    assert Day03().puzzle2() == 80179647

def test_day04():
    assert Day04().puzzle1() == 26346
    assert Day04().puzzle2() == 8467762

def test_day05():
    assert Day05().puzzle1() == 282277027
    assert Day05().puzzle2() == 11554135                # too lazy to do proper solution :)

def test_day06():
    assert Day06().puzzle1() == 800280
    assert Day06().puzzle2() == 45128024

def test_day07():
    assert Day07().puzzle1() == 251216224
    assert Day07().puzzle2() == 250825971

def test_day08():
    assert Day08().puzzle1() == 23147
    assert Day08().puzzle2() == 22289513667691

def test_day09():
    assert Day09().puzzle1() == 1819125966
    assert Day09().puzzle2() == 1140

def test_day10():
    assert Day10().puzzle1() == 6815
    assert Day10().puzzle2() == 269                     # try shoelace algorithm instead?

def test_day11():
    assert Day11().puzzle1() == 9522407
    assert Day11().puzzle2() == 544723432977

def test_day12():
    assert Day12().puzzle1() == 7260
    assert Day12().puzzle2() == 1909291258644

def test_day13():
    assert Day13().puzzle1() == 26957
    assert Day13().puzzle2() == 42695                   # slightly slow (600ms)

def test_day14():
    assert Day14().puzzle1() == 110677
    assert Day14().puzzle2() == 90551                   # pretty slow (2200ms)

def test_day15():
    assert Day15().puzzle1() == 517015
    assert Day15().puzzle2() == 286104

def test_day16():
    assert Day16().puzzle1() == 6883
    assert Day16().puzzle2() == 7228                    # pretty slow (1500ms)

def test_day17():
    assert Day17().puzzle1() == 936                     # very slow
    assert Day17().puzzle2() == 1157                    # very slow

def test_day18():
    assert Day18().puzzle1() == 48652
    assert Day18().puzzle2() == 45757884535661

def test_day19():
    assert Day19().puzzle1() == 418498
    assert Day19().puzzle2() == 123331556462603

def test_day20():
    assert Day20().puzzle1() == 825167435
    assert Day20().puzzle2() == 225514321828633

def test_day21():
    assert Day21().puzzle1() == 3632
    assert Day21().puzzle2() == 600336060511101

def test_day22():
    assert Day22().puzzle1() == 527
    # assert Day22().puzzle2() == 100376                # absurdly slow, sick-brain bad :)

def test_day23():
    assert Day23().puzzle1() == 2010
    assert Day23().puzzle2() == 6318                    # slow, need to speed up finding paths

def test_day24():
    assert Day24().puzzle1() == 16589
    assert Day24().puzzle2() == 781390555762385

def test_day25():
    assert Day25().puzzle1() == 562912
    assert Day25().puzzle2() == -2