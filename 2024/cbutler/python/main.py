from datetime import datetime

from puzzles import (
	boilerplate,
	dec01
)


def main():
	current_date = datetime.now().strftime("%d-%m-%Y")
	
	print(f"AoC: {current_date}")
	print('-----------------\n')

	# test boilerplate
	test_date = "01-12-2000"
	boilerplate.solve_part1(test_date)
	boilerplate.solve_part2(test_date, use_test_data=False)

	# Run puzzle solvers
	# dec01.solve_part1(current_date)
	# dec01.solve_part2(current_date, use_test_data=False)

# Run main function
if __name__ == "__main__":
	main()
