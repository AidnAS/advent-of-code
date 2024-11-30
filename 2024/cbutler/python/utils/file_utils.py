import os



def data_file_reader(filename: str = '', date: str = '', use_test_data: bool = False, part: int = 1, verbose: bool = False) -> list:
	current_directory = os.getcwd()
	parent_directory = os.path.dirname(current_directory)
	if verbose:
		print(f"current_directory: {current_directory}")
		print(f"parent_directory: {parent_directory}")

	filepath = f"{parent_directory}/common/data/{date}-{'part' + str(part) + '-' if use_test_data else ''}{'test-' if use_test_data else ''}data.txt"

	if filename:
		filepath = filename

	if verbose:
		print(f"filepath: {filepath}")

	with open(filepath, 'r') as file:
		lines = file.readlines()
	return [line.strip() for line in lines]
