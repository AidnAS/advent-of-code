from datetime import datetime

from puzzles import (
	boilerplate,
	dec01,
	dec02,
	dec03,
	dec04
)


def main():
	current_date = datetime.now().strftime("%d-%m-%Y")
	
	print(f"AoC: {current_date}")
	print('-----------------\n')

	# test boilerplate
	# test_date = "01-12-2000"
	# boilerplate.solve_part1(test_date)
	# boilerplate.solve_part2(test_date, use_test_data=False)

	# Run puzzle solvers
	# dec01.solve_part1(current_date, use_test_data=False) # ans: 3508942
	# dec01.solve_part2(current_date, use_test_data=False) # ans: 26593248

	# dec02.solve_part1(current_date, use_test_data=True) # ans:
	# dec02.solve_part2(current_date, use_test_data=True) # ans:

	# dec02.solve_part1(current_date, use_test_data=True) # ans:
	# dec02.solve_part2(current_date, use_test_data=True) # ans:

	dec04.solve_part1("04-12-2024", use_test_data=False) # ans: 2464
	dec04.solve_part2("04-12-2024", use_test_data=False) # ans: 1982

# Run main function
if __name__ == "__main__":
	main()
