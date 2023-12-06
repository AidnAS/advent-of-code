import importlib
import multiprocessing
import os
import time


def solve(day):
    try:
        solver = importlib.import_module(f"aoc.{day}.solver")
        t0 = time.perf_counter_ns()
        p1, p2 = solver.solve(os.path.join("aoc", day, "input.txt"))
        elapsed = time.perf_counter_ns() - t0
        return day, p1, p2, elapsed
    except ModuleNotFoundError:
        return day, "nil", "nil", 0


def output(results, elapsed, w=20, d=5):
    print(f"Completed in {elapsed/(10**6):.{d}} ms\n")
    print(f"{'day':>12} | {'part1':^{w}} | {'part2':^{w}} | {'elapsed (ms)':^{w}}")
    print(f" {'-' * 12}+{'-' * (w + 2)}+{'-' * (w + 2)}+{'-' * (w + 2)}")
    for day, p1, p2, elapsed in results:
        print(f"{day:>12} | {p1:>{w}} | {p2:>{w}} | {elapsed/(10**6):>{w}.{d}}")


def main():
    t0 = time.perf_counter_ns()
    with multiprocessing.Pool() as pool:
        results = pool.map(
            solve,
            (
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
            ),
        )
    elapsed = time.perf_counter_ns() - t0
    output(results, elapsed)


if __name__ == "__main__":
    main()
