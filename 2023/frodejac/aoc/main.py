import importlib
import os
import time

days = [
    "eight",
    "eighteen",
    "eleven",
    "fifteen",
    "five",
    "four",
    "fourteen",
    "nine",
    "nineteen",
    "one",
    "seven",
    "seventeen",
    "six",
    "sixteen",
    "ten",
    "thirteen",
    "three",
    "twelve",
    "twenty",
    "twentyfive",
    "twentyfour",
    "twentyone",
    "twentythree",
    "twentytwo",
    "two",
]


def main():
    print(f"{'day':>12} | {'part1':^15} | {'part2':^15} | {'elapsed':^15}")
    print("-------------+-----------------+-----------------+-----------------")
    for i, day in enumerate(days):
        try:
            solver = importlib.import_module(f"aoc.{day}.solver")
            t0 = time.time()
            p1, p2 = solver.solve(os.path.join("aoc", day, "input.txt"))
            elapsed = time.time() - t0
            print(f"{day:>12} | {p1:>15} | {p2:>15} | {elapsed:>15.6f}")
        except ModuleNotFoundError:
            print(f"{day:>12} | {'nil':>15} | {'nil':>15} | {'n/a':>15}")


if __name__ == "__main__":
    main()
