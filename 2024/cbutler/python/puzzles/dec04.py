from utils.file_utils import data_file_reader
from utils.output_utils import print_output


def solve_part1(date, use_test_data = False):
	print("Day 02, Part 1")

	lines = data_file_reader(date = date, use_test_data = use_test_data, part = 1)
	
	matrix = create_matrix(lines)
	# print_matrix(matrix)

	total: int = scan_matrix(matrix)

	print_output(str(total), use_test_data)



def solve_part2(date, use_test_data = False):
	print("Day 02, Part 2")

	lines = data_file_reader(date = date, use_test_data = use_test_data, part = 1)
	
	matrix = create_matrix(lines)
	# print_matrix(matrix)

	total: int = turbo_scan(matrix)

	print_output(str(total), use_test_data)


def create_matrix(lines: list) -> list:
	num_rows = len(lines)
	num_cols = len(lines[0])
	matrix = [['' for _ in range(num_cols)] for _ in range(num_rows)]

	for i in range(num_rows):
		for j in range(num_cols):
			matrix[i][j] = lines[i][j]
	
	return matrix

def scan_matrix(matrix: list) -> int:
	num_rows = len(matrix)
	num_cols = len(matrix[0])

	count: int = 0

	# pseudo code: Find XMAS
	# go char by char through the matrix.
	# check if the char is an X.
	# if yes, look in every direction to see if you can make the full word.
	# be aware of the edges of the matrix.

	for i in range(num_rows):
		for j in range(num_cols):
			if matrix[i][j] == 'X':
				# check up left
				if i -3 >= 0 and j - 3 >= 0:
					if matrix[i-1][j-1] == 'M' and matrix[i-2][j-2] == 'A' and matrix[i-3][j-3] == 'S':
						count += 1
				# check up
				if i - 3 >= 0:
					if matrix[i-1][j] == 'M' and matrix[i-2][j] == 'A' and matrix[i-3][j] == 'S':
						count += 1
				# check up right
				if i - 3 >= 0 and j + 3 < num_cols:
					if matrix[i-1][j+1] == 'M' and matrix[i-2][j+2] == 'A' and matrix[i-3][j+3] == 'S':
						count += 1
				# check right
				if j + 3 < num_cols:
					if matrix[i][j+1] == 'M' and matrix[i][j+2] == 'A' and matrix[i][j+3] == 'S':
						count += 1
				# check down right
				if i + 3 < num_rows and j + 3 < num_cols:
					if matrix[i+1][j+1] == 'M' and matrix[i+2][j+2] == 'A' and matrix[i+3][j+3] == 'S':
						count += 1
				# check down
				if i + 3 < num_rows:
					if matrix[i+1][j] == 'M' and matrix[i+2][j] == 'A' and matrix[i+3][j] == 'S':
						count += 1
				# check down left
				if i + 3 < num_rows and j - 3 >= 0:
					if matrix[i+1][j-1] == 'M' and matrix[i+2][j-2] == 'A' and matrix[i+3][j-3] == 'S':
						count += 1
				# check left
				if j - 3 >= 0:
					if matrix[i][j-1] == 'M' and matrix[i][j-2] == 'A' and matrix[i][j-3] == 'S':
						count += 1
	
	return count

def turbo_scan(matrix: list) -> int:
	num_rows = len(matrix)
	num_cols = len(matrix[0])

	count: int = 0

	# psuedo code: Find X-MAS
	# go char by char through the matrix. (scanning down)
	# check if you can make the shape and if it is there. 
	# i.e. the char at [i,j] must be M or S to start the upper left corner of the shape.
	# be aware of the edges of the matrix again.
	# only count if ALL conditions are met.

	for i in range(num_rows):
		for j in range(num_cols):
			if matrix[i][j] == 'M' or matrix[i][j] == 'S':
				if bf_check(matrix, i, j):
					count += 1
	
	return count


def bf_check(m: list, i: int, j: int) -> bool:
	num_rows = len(m)
	num_cols = len(m[0])

	if i+2 >= num_rows or j+2 >= num_cols:
		return False

	# 1a. the letter is M
	# M.M
	# .A.
	# S.S
	mm = m[i][j] == 'M' and m[i+1][j+1] == 'A' and m[i+2][j+2] == 'S' and m[i+2][j] == 'M' and m[i][j+2] == 'S'
	
	# 1b. the letter is M
	# M.S
	# .A.
	# M.S
	ms = m[i][j] == 'M' and m[i+1][j+1] == 'A' and m[i+2][j+2] == 'S' and m[i+2][j] == 'S' and m[i][j+2] == 'M'

	# 2a. the letter is S
	# S.S
	# .A.
	# M.M
	ss = m[i][j] == 'S' and m[i+1][j+1] == 'A' and m[i+2][j+2] == 'M' and m[i+2][j] == 'S' and m[i][j+2] == 'M'

	# 2b. the letter is S
	# S.M
	# .A.
	# S.M
	sm = m[i][j] == 'S' and m[i+1][j+1] == 'A' and m[i+2][j+2] == 'M' and m[i+2][j] == 'M' and m[i][j+2] == 'S'

	return mm or ms or ss or sm


def print_matrix(matrix: list):
	for row in matrix:
		formatted_row = ' '.join(f'[{char}]' for char in row)
		print(formatted_row)